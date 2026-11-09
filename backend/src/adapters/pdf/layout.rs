//! Inline text layout: styled spans → wrapped lines of positioned runs.

use super::fonts::{FaceId, Fonts};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb(pub u8, pub u8, pub u8);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub face: FaceId,
    pub size: f32,
    pub color: Rgb,
}

pub struct Span<'a> {
    pub text: &'a str,
    pub style: Style,
}

impl<'a> Span<'a> {
    pub fn new(text: &'a str, style: Style) -> Self {
        Self { text, style }
    }
}

/// A piece of a line drawn with one style, starting at `x` (relative to the line start).
#[derive(Debug, Clone)]
pub struct Run {
    pub x: f32,
    pub text: String,
    pub style: Style,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub runs: Vec<Run>,
    /// Width of the visible content (trailing spaces excluded).
    pub width: f32,
    pub height: f32,
    /// Distance from the top of the line box to the baseline.
    pub baseline: f32,
}

/// Lays `spans` out into lines no wider than `max_width`. `line_height` is a multiple of the
/// font size (like CSS `line-height: 1.6`). Hard line breaks (`\n`) are honoured; words longer
/// than a line are broken between characters.
pub fn wrap(fonts: &Fonts, spans: &[Span<'_>], max_width: f32, line_height: f32) -> Vec<Line> {
    let mut builder = LineBuilder::new(fonts, max_width, line_height);
    for span in spans {
        let face = fonts.face(span.style.face);
        let space = face.width(" ", span.style.size);
        for (n, paragraph) in span.text.split('\n').enumerate() {
            if n > 0 {
                builder.hard_break(span.style);
            }
            let mut words = paragraph.split(' ').peekable();
            while let Some(word) = words.next() {
                let has_space = words.peek().is_some();
                if !word.is_empty() {
                    builder.push_word(word, span.style);
                }
                if has_space {
                    builder.push_space(span.style, space);
                }
            }
        }
    }
    builder.finish()
}

struct LineBuilder<'f> {
    fonts: &'f Fonts,
    max_width: f32,
    line_height: f32,
    lines: Vec<Line>,
    runs: Vec<Run>,
    x: f32,
    content_width: f32,
    tallest: Option<Style>,
}

impl<'f> LineBuilder<'f> {
    fn new(fonts: &'f Fonts, max_width: f32, line_height: f32) -> Self {
        Self {
            fonts,
            max_width,
            line_height,
            lines: Vec::new(),
            runs: Vec::new(),
            x: 0.0,
            content_width: 0.0,
            tallest: None,
        }
    }

    fn push_word(&mut self, word: &str, style: Style) {
        let width = self.fonts.face(style.face).width(word, style.size);
        if self.x + width > self.max_width && !self.runs.is_empty() {
            self.break_line();
        }
        if width <= self.max_width {
            self.append(word, style, width);
            return;
        }
        // Longer than a whole line: split between characters.
        let face = self.fonts.face(style.face);
        let mut chunk = String::new();
        let mut chunk_width = 0.0;
        for c in word.chars() {
            let mut buf = [0u8; 4];
            let w = face.width(c.encode_utf8(&mut buf), style.size);
            if self.x + chunk_width + w > self.max_width
                && !(chunk.is_empty() && self.runs.is_empty())
            {
                if !chunk.is_empty() {
                    self.append(&chunk, style, chunk_width);
                }
                self.break_line();
                chunk.clear();
                chunk_width = 0.0;
            }
            chunk.push(c);
            chunk_width += w;
        }
        if !chunk.is_empty() {
            self.append(&chunk, style, chunk_width);
        }
    }

    fn push_space(&mut self, style: Style, width: f32) {
        // Leading spaces on a wrapped line are dropped.
        if self.runs.is_empty() {
            return;
        }
        self.append(" ", style, width);
        // A trailing space does not count towards the visible width.
        self.content_width -= width;
    }

    fn append(&mut self, text: &str, style: Style, width: f32) {
        self.note_style(style);
        match self.runs.last_mut() {
            Some(last) if last.style == style => last.text.push_str(text),
            _ => self.runs.push(Run {
                x: self.x,
                text: text.to_owned(),
                style,
            }),
        }
        self.x += width;
        self.content_width = self.x;
    }

    fn note_style(&mut self, style: Style) {
        if self.tallest.is_none_or(|t| style.size > t.size) {
            self.tallest = Some(style);
        }
    }

    fn hard_break(&mut self, style: Style) {
        self.note_style(style);
        self.break_line();
    }

    fn break_line(&mut self) {
        let Some(tallest) = self.tallest.take() else {
            return;
        };
        let face = self.fonts.face(tallest.face);
        let height = tallest.size * self.line_height;
        let glyph_height = (face.ascent - face.descent) * tallest.size;
        let baseline = (height - glyph_height) / 2.0 + face.ascent * tallest.size;

        let mut runs = std::mem::take(&mut self.runs);
        if let Some(last) = runs.last_mut() {
            let trimmed = last.text.trim_end().len();
            last.text.truncate(trimmed);
        }
        runs.retain(|r| !r.text.is_empty());

        self.lines.push(Line {
            runs,
            width: self.content_width.max(0.0),
            height,
            baseline,
        });
        self.x = 0.0;
        self.content_width = 0.0;
    }

    fn finish(mut self) -> Vec<Line> {
        if !self.runs.is_empty() {
            self.break_line();
        }
        self.lines
    }
}

/// Total height of a block of lines.
pub fn height(lines: &[Line]) -> f32 {
    lines.iter().map(|l| l.height).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn style() -> Style {
        Style {
            face: FaceId::Regular,
            size: 10.0,
            color: Rgb(0, 0, 0),
        }
    }

    #[test]
    fn wraps_within_width() {
        let fonts = Fonts::load().unwrap();
        let text = "The quick brown fox jumps over the lazy dog and keeps running far away";
        let lines = wrap(&fonts, &[Span::new(text, style())], 120.0, 1.5);
        assert!(lines.len() > 1);
        for line in &lines {
            assert!(line.width <= 120.0 + 0.01, "line too wide: {}", line.width);
            assert!(!line.runs[0].text.starts_with(' '));
            assert!(!line.runs.last().unwrap().text.ends_with(' '));
        }
        let joined: Vec<String> = lines
            .iter()
            .map(|l| l.runs.iter().map(|r| r.text.as_str()).collect())
            .collect();
        assert_eq!(joined.join(" "), text);
    }

    #[test]
    fn mixes_styles_and_breaks_long_words() {
        let fonts = Fonts::load().unwrap();
        let bold = Style {
            face: FaceId::Bold,
            ..style()
        };
        let lines = wrap(
            &fonts,
            &[Span::new("Step 1. ", bold), Span::new("Mix", style())],
            500.0,
            1.5,
        );
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].runs.len(), 2);
        assert_eq!(lines[0].runs[0].text, "Step 1. ");
        assert!(lines[0].runs[1].x > 0.0);

        let long = "x".repeat(200);
        let lines = wrap(&fonts, &[Span::new(&long, style())], 100.0, 1.5);
        assert!(lines.len() > 1);
        assert!(lines.iter().all(|l| l.width <= 100.01));

        let lines = wrap(&fonts, &[Span::new("a\nb", style())], 100.0, 1.5);
        assert_eq!(lines.len(), 2);
    }
}
