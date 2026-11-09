//! Per-IP token-bucket rate limiting for the credential endpoints (login / register).
//!
//! A tiny in-process implementation: one `f32` + `Instant` per client, pruned periodically, and
//! hard-capped so an address-spraying client cannot grow memory without bound.

use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    sync::Mutex,
    time::{Duration, Instant},
};

use axum::{
    extract::{ConnectInfo, Request, State},
    http::HeaderMap,
    middleware::Next,
    response::{IntoResponse, Response},
};

use super::app_state::AppState;
use crate::app_error::AppError;

const MAX_TRACKED_CLIENTS: usize = 50_000;

struct Bucket {
    tokens: f32,
    last: Instant,
}

pub struct RateLimiter {
    capacity: f32,
    refill_per_sec: f32,
    buckets: Mutex<HashMap<IpAddr, Bucket>>,
}

impl RateLimiter {
    /// `per_minute` requests per client, refilled continuously.
    pub fn per_minute(per_minute: u32) -> Self {
        let capacity = per_minute.max(1) as f32;
        Self {
            capacity,
            refill_per_sec: capacity / 60.0,
            buckets: Mutex::new(HashMap::new()),
        }
    }

    /// Takes one token for `ip`. On refusal returns how many seconds until the next token.
    pub fn check(&self, ip: IpAddr) -> Result<(), u64> {
        let now = Instant::now();
        let mut buckets = self.buckets.lock().unwrap_or_else(|e| e.into_inner());

        if buckets.len() >= MAX_TRACKED_CLIENTS && !buckets.contains_key(&ip) {
            self.prune_locked(&mut buckets, now);
            if buckets.len() >= MAX_TRACKED_CLIENTS {
                buckets.clear();
            }
        }

        let bucket = buckets.entry(ip).or_insert(Bucket {
            tokens: self.capacity,
            last: now,
        });
        let elapsed = now.duration_since(bucket.last).as_secs_f32();
        bucket.tokens = (bucket.tokens + elapsed * self.refill_per_sec).min(self.capacity);
        bucket.last = now;

        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            Ok(())
        } else {
            Err(((1.0 - bucket.tokens) / self.refill_per_sec).ceil() as u64)
        }
    }

    /// Forgets clients whose bucket has fully refilled (they are indistinguishable from new ones).
    pub fn prune(&self) {
        let mut buckets = self.buckets.lock().unwrap_or_else(|e| e.into_inner());
        self.prune_locked(&mut buckets, Instant::now());
        buckets.shrink_to_fit();
    }

    fn prune_locked(&self, buckets: &mut HashMap<IpAddr, Bucket>, now: Instant) {
        let full_after = Duration::from_secs_f32(self.capacity / self.refill_per_sec);
        buckets.retain(|_, b| now.duration_since(b.last) < full_after);
    }
}

/// Middleware applied to the credential routes.
pub async fn limit_credentials(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    let ip = client_ip(
        req.headers(),
        req.extensions().get::<ConnectInfo<SocketAddr>>(),
        state.config.trust_proxy,
    );
    if let Some(ip) = ip
        && let Err(retry_after_secs) = state.rate_limiter.check(ip)
    {
        return AppError::RateLimited { retry_after_secs }.into_response();
    }
    next.run(req).await
}

/// The client address. Behind a trusted reverse proxy (`TRUST_PROXY=true`) this is taken from
/// `X-Real-IP`, or else the right-most `X-Forwarded-For` hop (the one the proxy appended, which
/// the client cannot forge).
pub fn client_ip(
    headers: &HeaderMap,
    connect_info: Option<&ConnectInfo<SocketAddr>>,
    trust_proxy: bool,
) -> Option<IpAddr> {
    if trust_proxy {
        let header_ip = |name: &str| {
            headers
                .get(name)
                .and_then(|v| v.to_str().ok())
                .map(str::to_owned)
        };
        if let Some(ip) = header_ip("x-real-ip").and_then(|v| v.trim().parse().ok()) {
            return Some(ip);
        }
        if let Some(ip) = header_ip("x-forwarded-for").and_then(|v| {
            v.rsplit(',')
                .next()
                .and_then(|last| last.trim().parse().ok())
        }) {
            return Some(ip);
        }
    }
    connect_info.map(|ci| ci.0.ip())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bucket_refuses_after_capacity() {
        let rl = RateLimiter::per_minute(3);
        let ip: IpAddr = "10.0.0.1".parse().unwrap();
        assert!(rl.check(ip).is_ok());
        assert!(rl.check(ip).is_ok());
        assert!(rl.check(ip).is_ok());
        let retry = rl.check(ip).unwrap_err();
        assert!((1..=20).contains(&retry));
        // Another client is unaffected.
        assert!(rl.check("10.0.0.2".parse().unwrap()).is_ok());
    }

    #[test]
    fn proxy_headers_only_when_trusted() {
        let mut h = HeaderMap::new();
        h.insert("x-forwarded-for", "1.1.1.1, 2.2.2.2".parse().unwrap());
        let peer = ConnectInfo("9.9.9.9:1000".parse::<SocketAddr>().unwrap());
        assert_eq!(
            client_ip(&h, Some(&peer), false),
            Some("9.9.9.9".parse().unwrap())
        );
        assert_eq!(
            client_ip(&h, Some(&peer), true),
            Some("2.2.2.2".parse().unwrap())
        );
        h.insert("x-real-ip", "3.3.3.3".parse().unwrap());
        assert_eq!(
            client_ip(&h, Some(&peer), true),
            Some("3.3.3.3".parse().unwrap())
        );
    }
}
