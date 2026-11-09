//! Text extraction algorithms (PDF and HTML) producing compact text for the model.

/// Maximum characters of source text sent to the model (keeps small context windows happy).
pub const MAX_SOURCE_CHARS: usize = 8000;
const MAX_PDF_PAGES: usize = 20;
const MAX_PDF_PAGE_DECOMPRESSED: usize = 16 * 1024 * 1024;

/// Extracts the text of the first pages of a PDF. CPU-bound: call from a blocking task.
pub fn pdf_text(bytes: &[u8]) -> Result<String, String> {
    let doc = lopdf::Document::load_mem(bytes).map_err(|e| e.to_string())?;
    let pages: Vec<u32> = doc
        .get_pages()
        .keys()
        .copied()
        .take(MAX_PDF_PAGES)
        .collect();
    let mut text = String::new();
    for chunk in doc.extract_text_chunks_with_limit(&pages, MAX_PDF_PAGE_DECOMPRESSED) {
        // Unreadable pages are skipped rather than failing the whole import.
        if let Ok(chunk) = chunk {
            text.push_str(&chunk);
            text.push('\n');
        }
        if text.len() > MAX_SOURCE_CHARS * 4 {
            break;
        }
    }
    let text = collapse_whitespace(&text);
    if text.is_empty() {
        Err("the PDF contains no extractable text".into())
    } else {
        Ok(truncate_chars(text, MAX_SOURCE_CHARS))
    }
}

/// Readable text of an HTML page.
///
/// Recipe sites almost always embed a schema.org `Recipe` as JSON-LD; when present it is put
/// first because it is far more reliable than the visible text.
pub fn html_text(html: &str) -> String {
    let mut structured = String::new();
    let mut visible = String::with_capacity(html.len() / 4);

    let lower = html.to_ascii_lowercase();
    let mut i = 0;
    let bytes = html.as_bytes();
    while i < bytes.len() {
        if bytes[i] == b'<' {
            let tag_end = match lower[i..].find('>') {
                Some(off) => i + off + 1,
                None => break,
            };
            let tag = &lower[i..tag_end];
            let name: String = tag
                .trim_start_matches('<')
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric())
                .collect();

            if matches!(
                name.as_str(),
                "script" | "style" | "noscript" | "svg" | "template" | "iframe"
            ) {
                let close = format!("</{name}");
                let content_end = lower[tag_end..]
                    .find(&close)
                    .map_or(bytes.len(), |off| tag_end + off);
                if name == "script" && tag.contains("ld+json") {
                    let json = &html[tag_end..content_end];
                    if json.contains("Recipe") {
                        structured.push_str(json.trim());
                        structured.push('\n');
                    }
                }
                i = lower[content_end..]
                    .find('>')
                    .map_or(bytes.len(), |off| content_end + off + 1);
            } else {
                visible.push(' ');
                i = tag_end;
            }
        } else {
            let next = lower[i..].find('<').map_or(bytes.len(), |off| i + off);
            visible.push_str(&html[i..next]);
            i = next;
        }
    }

    let visible = collapse_whitespace(&decode_entities(&visible));
    let text = if structured.is_empty() {
        visible
    } else {
        format!(
            "Structured data (JSON-LD):\n{}\n\nPage text:\n{visible}",
            structured.trim()
        )
    };
    truncate_chars(text, MAX_SOURCE_CHARS)
}

fn decode_entities(s: &str) -> String {
    if !s.contains('&') {
        return s.to_owned();
    }
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(amp) = rest.find('&') {
        out.push_str(&rest[..amp]);
        rest = &rest[amp..];
        let Some(semi) = rest.find(';').filter(|&p| p <= 12) else {
            out.push('&');
            rest = &rest[1..];
            continue;
        };
        let entity = &rest[1..semi];
        let decoded = match entity {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" | "#39" => Some('\''),
            "nbsp" => Some(' '),
            e if e.starts_with("#x") || e.starts_with("#X") => u32::from_str_radix(&e[2..], 16)
                .ok()
                .and_then(char::from_u32),
            e if e.starts_with('#') => e[1..].parse().ok().and_then(char::from_u32),
            _ => None,
        };
        match decoded {
            Some(c) => {
                out.push(c);
                rest = &rest[semi + 1..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

pub fn collapse_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn truncate_chars(mut s: String, max: usize) -> String {
    if let Some((idx, _)) = s.char_indices().nth(max) {
        s.truncate(idx);
        s.push_str("...");
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_to_text() {
        let html = r#"<html><head><style>p{color:red}</style>
            <script>var x = "<p>nope</p>";</script>
            <script type="application/ld+json">{"@type":"Recipe","name":"Tortilla"}</script>
            </head><body><h1>Tortilla &amp; more</h1><p>Eggs&nbsp;&#38; potatoes &#x263A;</p></body></html>"#;
        let text = html_text(html);
        assert!(text.starts_with(
            "Structured data (JSON-LD):\n{\"@type\":\"Recipe\",\"name\":\"Tortilla\"}"
        ));
        assert!(text.ends_with("Tortilla & more Eggs & potatoes ☺"));
        assert!(!text.contains("nope"));
        assert!(!text.contains("color"));
    }

    #[test]
    fn truncation_respects_char_boundaries() {
        assert_eq!(truncate_chars("ñañaña".into(), 3), "ñañ...");
        assert_eq!(truncate_chars("abc".into(), 3), "abc");
    }
}
