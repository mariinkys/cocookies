//! [`RemoteFetcher`] implementation: outbound HTTP for user-supplied URLs (recipe pages to
//! import, recipe images for PDFs).
//!
//! Because the URL comes from a user, the server must not become a proxy into the private
//! network it runs in (SSRF). [`SafeFetcher`] therefore:
//!
//! * accepts only `http`/`https` (plus inline `data:` URLs, decoded locally),
//! * refuses hosts that are, or resolve to, loopback / private / link-local / reserved addresses
//!   (checked at DNS resolution time, so redirects and DNS tricks are covered too),
//! * caps redirects, total time and response size.

use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use async_trait::async_trait;
use base64::{Engine, engine::general_purpose::STANDARD};
use reqwest::{
    Url,
    dns::{Addrs, Name, Resolve, Resolving},
    redirect,
};

use crate::{
    app_error::{AppError, AppResult},
    use_cases::recipe::{RemoteFetcher, RemoteResource},
};

const MAX_REDIRECTS: usize = 5;

#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    #[error("invalid URL")]
    InvalidUrl,
    #[error("only http and https URLs are supported")]
    UnsupportedScheme,
    #[error("the URL points to a non-public address")]
    ForbiddenAddress,
    #[error("the response is larger than {0} bytes")]
    TooLarge(usize),
    #[error("the server answered with HTTP {0}")]
    Status(u16),
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
}

#[derive(Clone)]
pub struct SafeFetcher {
    client: reqwest::Client,
}

impl SafeFetcher {
    pub fn new(timeout: Duration) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::builder()
            .dns_resolver(Arc::new(PublicOnlyResolver))
            .redirect(redirect::Policy::custom(|attempt| {
                if attempt.previous().len() >= MAX_REDIRECTS {
                    attempt.error("too many redirects")
                } else if check_url(attempt.url()).is_err() {
                    attempt.error("redirect to a forbidden URL")
                } else {
                    attempt.follow()
                }
            }))
            .connect_timeout(Duration::from_secs(5))
            .timeout(timeout)
            .user_agent(concat!("Cocookies/", env!("CARGO_PKG_VERSION")))
            .build()?;
        Ok(Self { client })
    }

    /// Downloads `url`, refusing bodies larger than `max_bytes`.
    pub async fn download(
        &self,
        url: &str,
        max_bytes: usize,
    ) -> Result<RemoteResource, FetchError> {
        let url = url.trim();
        if let Some(data) = url.strip_prefix("data:") {
            return decode_data_url(data, max_bytes);
        }

        let url = Url::parse(url).map_err(|_| FetchError::InvalidUrl)?;
        check_url(&url)?;

        let mut response = self.client.get(url).send().await?;
        if !response.status().is_success() {
            return Err(FetchError::Status(response.status().as_u16()));
        }
        if response
            .content_length()
            .is_some_and(|len| len as usize > max_bytes)
        {
            return Err(FetchError::TooLarge(max_bytes));
        }
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned);

        let mut body = Vec::new();
        while let Some(chunk) = response.chunk().await? {
            if body.len() + chunk.len() > max_bytes {
                return Err(FetchError::TooLarge(max_bytes));
            }
            body.extend_from_slice(&chunk);
        }
        Ok(RemoteResource { body, content_type })
    }
}

#[async_trait]
impl RemoteFetcher for SafeFetcher {
    async fn fetch(&self, url: &str, max_bytes: usize) -> AppResult<RemoteResource> {
        self.download(url, max_bytes)
            .await
            .map_err(|e| AppError::Unprocessable(e.to_string()))
    }
}

/// Scheme and literal-IP checks (hostnames are checked when resolved).
fn check_url(url: &Url) -> Result<(), FetchError> {
    if !matches!(url.scheme(), "http" | "https") {
        return Err(FetchError::UnsupportedScheme);
    }
    let host = url.host_str().ok_or(FetchError::InvalidUrl)?;
    let literal = host.trim_start_matches('[').trim_end_matches(']');
    match literal.parse::<IpAddr>() {
        Ok(ip) if !is_public(ip) => Err(FetchError::ForbiddenAddress),
        _ if host.eq_ignore_ascii_case("localhost") => Err(FetchError::ForbiddenAddress),
        _ => Ok(()),
    }
}

/// `data:[<mime>][;base64],<payload>`
fn decode_data_url(data: &str, max_bytes: usize) -> Result<RemoteResource, FetchError> {
    let (meta, payload) = data.split_once(',').ok_or(FetchError::InvalidUrl)?;
    // Cheap upper bound before decoding anything.
    if payload.len() / 4 * 3 > max_bytes.saturating_add(3) {
        return Err(FetchError::TooLarge(max_bytes));
    }
    let (mime, is_base64) = match meta.strip_suffix(";base64") {
        Some(mime) => (mime, true),
        None => (meta, false),
    };
    let body = if is_base64 {
        STANDARD
            .decode(payload.trim())
            .map_err(|_| FetchError::InvalidUrl)?
    } else {
        payload.as_bytes().to_vec()
    };
    if body.len() > max_bytes {
        return Err(FetchError::TooLarge(max_bytes));
    }
    Ok(RemoteResource {
        body,
        content_type: (!mime.is_empty()).then(|| mime.to_owned()),
    })
}

struct PublicOnlyResolver;

impl Resolve for PublicOnlyResolver {
    fn resolve(&self, name: Name) -> Resolving {
        Box::pin(async move {
            let addrs: Vec<SocketAddr> = tokio::net::lookup_host((name.as_str(), 0))
                .await?
                .filter(|a| is_public(a.ip()))
                .collect();
            if addrs.is_empty() {
                return Err(FetchError::ForbiddenAddress.into());
            }
            Ok(Box::new(addrs.into_iter()) as Addrs)
        })
    }
}

/// Is `ip` a globally routable unicast address?
pub fn is_public(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => is_public_v4(v4),
        IpAddr::V6(v6) => match v6.to_ipv4_mapped() {
            Some(v4) => is_public_v4(v4),
            None => is_public_v6(v6),
        },
    }
}

fn is_public_v4(ip: Ipv4Addr) -> bool {
    let [a, b, c, _] = ip.octets();
    !(ip.is_private()
        || ip.is_loopback()
        || ip.is_link_local()
        || ip.is_broadcast()
        || ip.is_documentation()
        || ip.is_unspecified()
        || ip.is_multicast()
        || a == 0                                   // "this" network
        || (a == 100 && (64..128).contains(&b))     // carrier-grade NAT 100.64/10
        || (a == 192 && b == 0 && c == 0)           // IETF protocol assignments
        || (a == 198 && (18..20).contains(&b))      // benchmarking 198.18/15
        || a >= 240) // reserved
}

fn is_public_v6(ip: Ipv6Addr) -> bool {
    let first = ip.segments()[0];
    !(ip.is_loopback()
        || ip.is_unspecified()
        || ip.is_multicast()
        || (first & 0xfe00) == 0xfc00            // unique local fc00::/7
        || (first & 0xffc0) == 0xfe80            // link local fe80::/10
        || (first == 0x2001 && ip.segments()[1] == 0x0db8)) // documentation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_addresses() {
        for private in [
            "127.0.0.1",
            "10.1.2.3",
            "192.168.1.1",
            "172.16.0.1",
            "169.254.169.254",
            "100.64.0.1",
            "0.0.0.0",
            "::1",
            "fd00::1",
            "fe80::1",
            "::ffff:127.0.0.1",
        ] {
            assert!(
                !is_public(private.parse().unwrap()),
                "{private} should be blocked"
            );
        }
        for public in ["1.1.1.1", "8.8.8.8", "2606:4700:4700::1111"] {
            assert!(
                is_public(public.parse().unwrap()),
                "{public} should be allowed"
            );
        }
    }

    #[test]
    fn rejects_bad_urls() {
        let check = |u: &str| check_url(&Url::parse(u).unwrap());
        assert!(matches!(
            check("file:///etc/passwd"),
            Err(FetchError::UnsupportedScheme)
        ));
        assert!(matches!(
            check("http://127.0.0.1:8080/x"),
            Err(FetchError::ForbiddenAddress)
        ));
        assert!(matches!(
            check("http://[::1]/x"),
            Err(FetchError::ForbiddenAddress)
        ));
        assert!(matches!(
            check("http://localhost/x"),
            Err(FetchError::ForbiddenAddress)
        ));
        assert!(check("https://example.com/recipe").is_ok());
    }

    #[test]
    fn decodes_data_urls() {
        let f = decode_data_url("image/png;base64,iVBORw==", 100).unwrap();
        assert_eq!(f.content_type.as_deref(), Some("image/png"));
        assert_eq!(f.body, [0x89, b'P', b'N', b'G']);
        assert_eq!(
            decode_data_url("text/plain,hello", 10).unwrap().body,
            b"hello"
        );
        assert!(matches!(
            decode_data_url("text/plain,hello", 2),
            Err(FetchError::TooLarge(_))
        ));
        assert!(matches!(
            decode_data_url("no-comma", 10),
            Err(FetchError::InvalidUrl)
        ));
    }
}
