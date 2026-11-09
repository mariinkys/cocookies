//! Session cookies.
//!
//! * `access_token`  – JWT, sent with every API call (`Path=/`).
//! * `refresh_token` – opaque random token, only sent to `/api/auth/*` so it can be rotated on
//!   `/refresh` and revoked on `/logout`.
//!
//! Both are `HttpOnly` + `SameSite=Strict`; `Secure` is controlled by `COOKIE_SECURE`.

use axum::http::{HeaderMap, HeaderValue, header};

pub const ACCESS_TOKEN: &str = "access_token";
pub const REFRESH_TOKEN: &str = "refresh_token";
const ACCESS_PATH: &str = "/";
const REFRESH_PATH: &str = "/api/auth";
/// Path used by the previous backend; cleared on logout so stale cookies do not linger.
const LEGACY_REFRESH_PATH: &str = "/api/auth/refresh";

fn build(name: &str, value: &str, path: &str, max_age_secs: u64, secure: bool) -> HeaderValue {
    let secure = if secure { "; Secure" } else { "" };
    let cookie = format!(
        "{name}={value}; Path={path}; Max-Age={max_age_secs}; HttpOnly; SameSite=Strict{secure}"
    );
    HeaderValue::from_str(&cookie).expect("cookie values are URL-safe ASCII")
}

/// `Set-Cookie` headers establishing a session.
pub fn session(
    headers: &mut HeaderMap,
    access: &str,
    access_ttl_secs: u64,
    refresh: &str,
    refresh_ttl_secs: u64,
    secure: bool,
) {
    headers.append(
        header::SET_COOKIE,
        build(ACCESS_TOKEN, access, ACCESS_PATH, access_ttl_secs, secure),
    );
    headers.append(
        header::SET_COOKIE,
        build(
            REFRESH_TOKEN,
            refresh,
            REFRESH_PATH,
            refresh_ttl_secs,
            secure,
        ),
    );
}

/// `Set-Cookie` headers that delete every session cookie.
pub fn clear(headers: &mut HeaderMap, secure: bool) {
    headers.append(
        header::SET_COOKIE,
        build(ACCESS_TOKEN, "", ACCESS_PATH, 0, secure),
    );
    headers.append(
        header::SET_COOKIE,
        build(REFRESH_TOKEN, "", REFRESH_PATH, 0, secure),
    );
    headers.append(
        header::SET_COOKIE,
        build(REFRESH_TOKEN, "", LEGACY_REFRESH_PATH, 0, secure),
    );
}

/// All values of the cookie `name` (a browser may send several with different paths).
pub fn read_all<'a>(headers: &'a HeaderMap, name: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .flat_map(|v| v.split(';'))
        .filter_map(move |pair| {
            let (k, v) = pair.trim().split_once('=')?;
            (k == name && !v.is_empty()).then_some(v)
        })
}

pub fn read<'a>(headers: &'a HeaderMap, name: &'a str) -> Option<&'a str> {
    read_all(headers, name).next()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_cookie_headers() {
        let mut h = HeaderMap::new();
        h.append(
            header::COOKIE,
            HeaderValue::from_static("a=1; refresh_token=old"),
        );
        h.append(
            header::COOKIE,
            HeaderValue::from_static("refresh_token=new; access_token=jwt"),
        );
        assert_eq!(read(&h, "access_token"), Some("jwt"));
        assert_eq!(
            read_all(&h, "refresh_token").collect::<Vec<_>>(),
            ["old", "new"]
        );
        assert_eq!(read(&h, "missing"), None);
    }
}
