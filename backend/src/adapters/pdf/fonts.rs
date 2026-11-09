//! Embedded fonts and the minimal text shaping needed to lay out Latin text.
//!
//! Gelasio (SIL OFL, metric-compatible with Georgia, the font of the original HTML template) is
//! compiled into the binary as two variable fonts; krilla subsets them so a PDF only carries
//! the glyphs it uses.
//!
//! Shaping is one glyph per character with advances from the font's metrics. That is exact for
//! the Latin scripts the app targets and lets layout measurements and the final drawing use the
//! very same numbers (no shaping engine needed).

use krilla::{
    Data,
    text::{Font, GlyphId as KrillaGlyphId, KrillaGlyph, Tag},
};
use skrifa::{
    FontRef, GlyphId, MetadataProvider,
    charmap::Charmap,
    instance::{Location, Size},
    metrics::GlyphMetrics,
};

static UPRIGHT: &[u8] = include_bytes!("../../../assets/fonts/Gelasio.ttf");
static ITALIC: &[u8] = include_bytes!("../../../assets/fonts/Gelasio-Italic.ttf");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaceId {
    Regular,
    Bold,
    Italic,
}

pub struct Face {
    pub font: Font,
    charmap: Charmap<'static>,
    metrics: GlyphMetrics<'static>,
    units_per_em: f32,
    /// Ascender / descender as fractions of the em (descender is negative).
    pub ascent: f32,
    pub descent: f32,
}

pub struct Fonts {
    regular: Face,
    bold: Face,
    italic: Face,
}

impl Fonts {
    pub fn load() -> Result<Self, String> {
        Ok(Self {
            regular: Face::new(UPRIGHT, 400.0)?,
            bold: Face::new(UPRIGHT, 700.0)?,
            italic: Face::new(ITALIC, 400.0)?,
        })
    }

    pub fn face(&self, id: FaceId) -> &Face {
        match id {
            FaceId::Regular => &self.regular,
            FaceId::Bold => &self.bold,
            FaceId::Italic => &self.italic,
        }
    }
}

impl Face {
    fn new(data: &'static [u8], weight: f32) -> Result<Self, String> {
        let font_ref = FontRef::new(data).map_err(|e| e.to_string())?;
        // skrifa ties metric tables to the lifetime of the location they were created for.
        // Faces are created once per process, so leaking these few bytes is the simplest way
        // to keep fast `'static` metric tables around.
        let location: &'static Location =
            Box::leak(Box::new(font_ref.axes().location([("wght", weight)])));
        let font_metrics = font_ref.metrics(Size::unscaled(), location);
        let units_per_em = f32::from(font_metrics.units_per_em.max(1));

        let font = Font::new_variable(Data::from(data), 0, &[(Tag::new(b"wght"), weight)])
            .ok_or("unable to load embedded font")?;

        Ok(Self {
            font,
            charmap: font_ref.charmap(),
            metrics: font_ref.glyph_metrics(Size::unscaled(), location),
            units_per_em,
            ascent: font_metrics.ascent / units_per_em,
            descent: font_metrics.descent / units_per_em,
        })
    }

    fn glyph(&self, c: char) -> (GlyphId, f32) {
        let id = self.charmap.map(c).unwrap_or(GlyphId::NOTDEF);
        let advance = self.metrics.advance_width(id).unwrap_or(0.0) / self.units_per_em;
        (id, advance)
    }

    /// Width of `text` at `size` points.
    pub fn width(&self, text: &str, size: f32) -> f32 {
        text.chars().map(|c| self.glyph(c).1).sum::<f32>() * size
    }

    /// Glyphs for `text`, ready for `Surface::draw_glyphs`.
    pub fn shape(&self, text: &str) -> Vec<KrillaGlyph> {
        text.char_indices()
            .map(|(i, c)| {
                let (id, advance) = self.glyph(c);
                KrillaGlyph::new(
                    KrillaGlyphId::new(id.to_u32()),
                    advance,
                    0.0,
                    0.0,
                    0.0,
                    i..i + c.len_utf8(),
                    None,
                )
            })
            .collect()
    }
}
