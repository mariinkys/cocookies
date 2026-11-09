/// Who is making a request: a signed-in user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity {
    pub user_id: String,
    pub email: String,
}

impl Identity {
    pub fn owns(&self, owner_id: &str) -> bool {
        self.user_id == owner_id
    }
}

/// A freshly issued pair of credentials.
#[derive(Debug, Clone)]
pub struct Session {
    /// Short-lived token authenticating every request.
    pub access_token: String,
    /// Long-lived, single-use token that obtains a new session.
    pub refresh_token: String,
}

/// A stored refresh token (only its fingerprint is persisted, never the token itself).
#[derive(Debug, Clone)]
pub struct RefreshToken {
    pub id: String,
    pub user_id: String,
    pub expires_at: String,
    pub revoked: bool,
}
