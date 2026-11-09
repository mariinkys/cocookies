use crate::domain::validation::{ValidationError, Violations, chars_between, is_email, not_blank};

/// A registered account.
#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub failed_login_attempts: i64,
    /// ISO-8601 UTC instant until which logins are refused.
    pub locked_until: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl User {
    pub fn is_locked_at(&self, now_iso: &str) -> bool {
        self.locked_until
            .as_deref()
            .is_some_and(|until| until > now_iso)
    }
}

/// Registration input.
#[derive(Debug, Clone)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl NewUser {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut v = Violations::new();
        validate_profile(&mut v, &self.name, &self.email);
        validate_new_password(&mut v, &self.password);
        v.finish()
    }
}

/// Changes a user may make to their own profile.
#[derive(Debug, Clone)]
pub struct ProfileUpdate {
    pub name: String,
    pub email: String,
}

impl ProfileUpdate {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut v = Violations::new();
        validate_profile(&mut v, &self.name, &self.email);
        v.finish()
    }
}

#[derive(Debug, Clone)]
pub struct PasswordChange {
    pub current_password: Option<String>,
    pub new_password: String,
}

impl PasswordChange {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut v = Violations::new();
        validate_new_password(&mut v, &self.new_password);
        v.finish()
    }
}

/// Login input.
#[derive(Debug, Clone)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

impl Credentials {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut v = Violations::new();
        v.check(not_blank(&self.email), "Email is required")
            .check(
                !not_blank(&self.email) || is_email(&self.email),
                "Must be a valid email",
            )
            .check(!self.password.is_empty(), "Password is required");
        v.finish()
    }
}

fn validate_profile(v: &mut Violations, name: &str, email: &str) {
    v.check(not_blank(name), "Name is required")
        .check(
            !not_blank(name) || chars_between(name.trim(), 2, 100),
            "Name must be between 2 and 100 characters",
        )
        .check(not_blank(email), "Email is required")
        .check(
            !not_blank(email) || is_email(email),
            "Must be a valid email",
        );
}

fn validate_new_password(v: &mut Violations, password: &str) {
    v.check(!password.is_empty(), "Password is required")
        .check(
            password.is_empty() || password.chars().count() >= 8,
            "Password must be at least 8 characters",
        )
        // bcrypt only looks at the first 72 bytes; refuse longer input rather than truncating.
        .check(password.len() <= 72, "Password must be at most 72 bytes");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registration_rules() {
        let new = |name: &str, email: &str, password: &str| NewUser {
            name: name.into(),
            email: email.into(),
            password: password.into(),
        };
        assert!(
            new("Ana", "ana@example.com", "password1")
                .validate()
                .is_ok()
        );
        let err = new("A", "nope", "short").validate().unwrap_err().0;
        assert!(err.contains("between 2 and 100"));
        assert!(err.contains("valid email"));
        assert!(err.contains("at least 8"));
        assert!(
            new("Ana", "ana@example.com", &"x".repeat(73))
                .validate()
                .is_err()
        );
    }

    #[test]
    fn lock_is_time_bound() {
        let mut u = User {
            id: "1".into(),
            name: "n".into(),
            email: "e".into(),
            password_hash: "h".into(),
            failed_login_attempts: 5,
            locked_until: Some("2026-01-01T10:15:00Z".into()),
            created_at: String::new(),
            updated_at: String::new(),
        };
        assert!(u.is_locked_at("2026-01-01T10:00:00Z"));
        assert!(!u.is_locked_at("2026-01-01T10:15:00Z"));
        u.locked_until = None;
        assert!(!u.is_locked_at("2026-01-01T10:00:00Z"));
    }
}
