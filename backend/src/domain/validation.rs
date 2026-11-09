//! Minimal, dependency-free validation helpers.
//!
//! Entities implement `validate()` by pushing human-readable messages into a [`Violations`]
//! collector; all messages are joined into a single [`ValidationError`].

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError(pub String);

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ValidationError {}

#[derive(Default)]
pub struct Violations(Vec<String>);

impl Violations {
    pub fn new() -> Self {
        Self::default()
    }

    /// Records `message` when `ok` is false.
    pub fn check(&mut self, ok: bool, message: impl Into<String>) -> &mut Self {
        if !ok {
            self.0.push(message.into());
        }
        self
    }

    pub fn finish(self) -> Result<(), ValidationError> {
        if self.0.is_empty() {
            Ok(())
        } else {
            Err(ValidationError(self.0.join(", ")))
        }
    }
}

pub fn not_blank(s: &str) -> bool {
    !s.trim().is_empty()
}

pub fn max_chars(s: &str, max: usize) -> bool {
    s.chars().count() <= max
}

pub fn chars_between(s: &str, min: usize, max: usize) -> bool {
    (min..=max).contains(&s.chars().count())
}

/// Pragmatic e-mail check: one `@`, non-empty local part and domain, no whitespace.
pub fn is_email(s: &str) -> bool {
    let s = s.trim();
    match s.split_once('@') {
        Some((local, domain)) => {
            !local.is_empty()
                && !domain.is_empty()
                && !domain.contains('@')
                && !domain.starts_with('.')
                && !domain.ends_with('.')
                && !s.chars().any(char::is_whitespace)
        }
        None => false,
    }
}

/// Trims and turns empty strings into `None`.
pub fn normalize_opt(s: Option<String>) -> Option<String> {
    s.map(|v| v.trim().to_owned()).filter(|v| !v.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_rules() {
        assert!(is_email("a@b.com"));
        assert!(is_email("admin@example"));
        assert!(!is_email("a@"));
        assert!(!is_email("@b.com"));
        assert!(!is_email("a b@c.com"));
        assert!(!is_email("a@b@c"));
    }

    #[test]
    fn violations_are_joined() {
        let mut v = Violations::new();
        v.check(false, "a").check(true, "b").check(false, "c");
        assert_eq!(v.finish(), Err(ValidationError("a, c".into())));
    }
}
