// SPDX-License-Identifier: GPL-3.0-only

pub fn truncate_str_with_ellipsis(input: &str, limit: usize) -> String {
    if input.len() <= limit {
        input.to_string()
    } else {
        let mut end = limit;
        while !input.is_char_boundary(end) {
            end -= 1;
        }
        format!("{}...", &input[..end])
    }
}
