//! [`RecipeRenderer`] implementation: recipe → PDF, rendered natively with
//! [krilla](https://github.com/LaurenzV/krilla).
//!
//! Pipeline: [`document::layout`] turns a recipe into pages of drawing operations
//! ([`canvas`]), using [`layout`] for line breaking and [`fonts`] for metrics; then
//! [`render::render`] replays those operations into a PDF. Fonts are parsed once and shared.
//!
//! To add a template: add a variant to `PdfTemplate` (and labels in [`labels`]); for a
//! different look, write another `document`-style module that fills a [`canvas::Canvas`].

mod canvas;
mod document;
mod fonts;
mod labels;
mod layout;
mod render;

use std::sync::Arc;

use async_trait::async_trait;
use krilla::{Data, image::Image};

use crate::{
    app_error::{AppError, AppResult},
    entities::recipe::{PdfTemplate, Recipe},
    use_cases::recipe::RecipeRenderer,
};

/// Cheap to clone; fonts are parsed once and shared.
#[derive(Clone)]
pub struct KrillaRecipeRenderer {
    fonts: Arc<fonts::Fonts>,
}

impl KrillaRecipeRenderer {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            fonts: Arc::new(fonts::Fonts::load()?),
        })
    }

    /// Renders synchronously. `image` is the raw recipe picture (PNG, JPEG, GIF or WebP).
    pub fn render(
        &self,
        recipe: &Recipe,
        template: PdfTemplate,
        image: Option<Vec<u8>>,
    ) -> Result<Vec<u8>, String> {
        let image = image.and_then(decode_image);
        let labels = labels::labels(template);
        let canvas = document::layout(recipe, labels, &self.fonts, image.is_some());
        render::render(
            &canvas,
            &self.fonts,
            image.as_ref(),
            render::Meta {
                title: &recipe.title,
                language: labels.language,
            },
        )
    }
}

#[async_trait]
impl RecipeRenderer for KrillaRecipeRenderer {
    /// Rendering is CPU-bound, so it runs on the blocking pool.
    async fn render_pdf(
        &self,
        recipe: Recipe,
        template: PdfTemplate,
        image: Option<Vec<u8>>,
    ) -> AppResult<Vec<u8>> {
        let renderer = self.clone();
        tokio::task::spawn_blocking(move || renderer.render(&recipe, template, image))
            .await
            .map_err(AppError::internal)?
            .map_err(|e| AppError::Internal(format!("PDF generation failed: {e}")))
    }
}

/// JPEGs are embedded as-is (no re-encoding), the rest are decoded by krilla.
fn decode_image(bytes: Vec<u8>) -> Option<Image> {
    let result = match bytes.as_slice() {
        [0xFF, 0xD8, 0xFF, ..] => Image::from_jpeg(Data::from(bytes), true),
        [0x89, b'P', b'N', b'G', ..] => Image::from_png(Data::from(bytes), true),
        [b'G', b'I', b'F', b'8', ..] => Image::from_gif(Data::from(bytes), true),
        [
            b'R',
            b'I',
            b'F',
            b'F',
            _,
            _,
            _,
            _,
            b'W',
            b'E',
            b'B',
            b'P',
            ..,
        ] => Image::from_webp(Data::from(bytes), true),
        _ => return None,
    };
    result
        .inspect_err(|e| tracing::info!(error = %e, "unsupported recipe image"))
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{
        recipe::{Ingredient, NutrientValues, Nutrition, Step},
        taxonomy::TaxonomyItem,
    };

    fn sample(long_step: bool) -> Recipe {
        let tax = |name: &str| TaxonomyItem {
            id: "t".into(),
            user_id: "u".into(),
            name: name.into(),
            description: None,
            created_at: "2026-01-01T00:00:00Z".into(),
            updated_at: "2026-01-01T00:00:00Z".into(),
        };
        let step = |n: i64, text: String, duration: Option<i64>| Step {
            id: n,
            step_number: n,
            instructions: text,
            duration,
        };
        let ingredient =
            |n: i64, name: &str, quantity: Option<f64>, unit: Option<&str>, notes: Option<&str>| {
                Ingredient {
                    id: n,
                    name: name.into(),
                    quantity,
                    unit: unit.map(Into::into),
                    notes: notes.map(Into::into),
                    sort_order: n,
                }
            };
        Recipe {
            id: "r".into(),
            user_id: "u".into(),
            category: Some(tax("Española")),
            difficulty: Some(tax("Fácil")),
            title: "Tortilla de patatas con cebolla caramelizada".into(),
            description: Some("Jugosa por dentro y dorada por fuera. Un clásico que mejora al día siguiente.".into()),
            prep_time: Some(15),
            cook_time: Some(30),
            servings: 4,
            image_url: Some("x".into()),
            shared: true,
            steps: vec![
                step(1, if long_step { "Remueve con cuidado. ".repeat(400) } else { "Pela y corta las patatas en láminas finas; pica la cebolla.".into() }, None),
                step(2, "Fríe las patatas y la cebolla a fuego medio en abundante aceite de oliva hasta que estén tiernas pero sin dorar. Escúrrelas y reserva el aceite.".into(), Some(20)),
                step(3, "Bate los huevos con sal, mezcla con las patatas y deja reposar unos minutos.\nCuaja por ambos lados.".into(), Some(10)),
            ],
            ingredients: vec![
                ingredient(0, "Patatas", Some(1.0), Some("kg"), Some("mejor nuevas")),
                ingredient(1, "Huevos", Some(6.0), None, None),
                ingredient(2, "Cebolla", Some(0.5), None, Some("opcional, pero recomendable")),
                ingredient(3, "Aceite de oliva virgen extra", None, None, None),
            ],
            nutrition: Some(Nutrition {
                id: 1,
                serving_size_value: 100.0,
                serving_size_unit: "g".into(),
                values: NutrientValues { calories: Some(180.0), ..Default::default() },
            }),
            created_at: "2026-03-07T10:00:00Z".into(),
            updated_at: "2026-11-08T21:00:00Z".into(),
        }
    }

    #[test]
    fn renders_with_every_image_format() {
        let renderer = KrillaRecipeRenderer::new().unwrap();
        let cases: [(&str, Option<&[u8]>, bool); 4] = [
            (
                "jpeg",
                Some(include_bytes!("../../../tests/fixtures/dish.jpg")),
                false,
            ),
            (
                "png",
                Some(include_bytes!("../../../tests/fixtures/dish.png")),
                false,
            ),
            ("noimage-long", None, true),
            ("garbage-image", Some(b"definitely not an image"), false),
        ];
        for (name, image, long) in cases {
            for template in [PdfTemplate::DefaultEnglish, PdfTemplate::DefaultSpanish] {
                let pdf = renderer
                    .render(&sample(long), template, image.map(<[u8]>::to_vec))
                    .unwrap();
                assert!(pdf.starts_with(b"%PDF-"));
                if let Ok(dir) = std::env::var("PDF_OUT_DIR") {
                    let lang = labels::labels(template).language;
                    std::fs::write(format!("{dir}/{name}-{lang}.pdf"), &pdf).unwrap();
                }
            }
        }
    }
}
