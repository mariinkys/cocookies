//! Opaque refresh tokens: 256 random bits, stored only as a SHA-256 fingerprint.

use std::fmt::Write;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use sha2::{Digest, Sha256};

use crate::{
    app_error::{AppError, AppResult},
    use_cases::auth::RefreshTokenFactory,
};

#[derive(Default)]
pub struct RandomRefreshTokens;

impl RefreshTokenFactory for RandomRefreshTokens {
    fn generate_refresh_token(&self) -> AppResult<String> {
        let mut raw = [0u8; 32];
        getrandom::fill(&mut raw).map_err(AppError::internal)?;
        Ok(URL_SAFE_NO_PAD.encode(raw))
    }

    fn fingerprint(&self, token: &str) -> String {
        Sha256::digest(token.as_bytes())
            .iter()
            .fold(String::with_capacity(64), |mut s, b| {
                let _ = write!(s, "{b:02x}");
                s
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_are_random_and_fingerprints_stable() {
        let f = RandomRefreshTokens;
        let (a, b) = (
            f.generate_refresh_token().unwrap(),
            f.generate_refresh_token().unwrap(),
        );
        assert_ne!(a, b);
        assert_eq!(a.len(), 43);
        assert_eq!(f.fingerprint(&a), f.fingerprint(&a));
        assert_eq!(
            f.fingerprint("abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
