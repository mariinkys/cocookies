//! Timestamps.
//!
//! All timestamps are stored and exchanged as ISO-8601 UTC strings with second precision
//! (`2026-01-31T18:04:05Z`). That format sorts lexicographically, so SQLite can compare and order
//! them as plain TEXT, and JavaScript's `new Date()` parses it directly. Keeping them as strings
//! avoids pulling a date/time crate for what is pure pass-through data.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Seconds since the Unix epoch.
pub fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or_default()
}

/// The current instant as `yyyy-MM-ddTHH:mm:ssZ`.
pub fn now_iso() -> String {
    iso_from_unix(unix_now())
}

/// `now + duration` as `yyyy-MM-ddTHH:mm:ssZ`.
pub fn iso_in(duration: Duration) -> String {
    iso_from_unix(unix_now() + duration.as_secs() as i64)
}

/// Formats Unix seconds as `yyyy-MM-ddTHH:mm:ssZ`.
pub fn iso_from_unix(secs: i64) -> String {
    let days = secs.div_euclid(86_400);
    let sod = secs.rem_euclid(86_400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        sod / 3600,
        sod % 3600 / 60,
        sod % 60
    )
}

/// Turns the date part of an ISO timestamp into `dd/MM/yyyy` (used in PDFs).
pub fn format_day_month_year(iso: &str) -> Option<String> {
    let date = iso.get(..10)?;
    let mut parts = date.split('-');
    let (y, m, d) = (parts.next()?, parts.next()?, parts.next()?);
    Some(format!("{d}/{m}/{y}"))
}

/// Howard Hinnant's `civil_from_days`: days since 1970-01-01 to (year, month, day).
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = yoe + era * 400 + i64::from(m <= 2);
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_known_instants() {
        assert_eq!(iso_from_unix(0), "1970-01-01T00:00:00Z");
        assert_eq!(iso_from_unix(951_782_400), "2000-02-29T00:00:00Z");
        assert_eq!(iso_from_unix(1_798_761_599), "2026-12-31T23:59:59Z");
        assert_eq!(iso_from_unix(4_107_542_400), "2100-03-01T00:00:00Z");
    }

    #[test]
    fn formats_pdf_dates() {
        assert_eq!(
            format_day_month_year("2026-03-07T10:00:00Z").as_deref(),
            Some("07/03/2026")
        );
        assert_eq!(format_day_month_year("bad"), None);
    }
}
