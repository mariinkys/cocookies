//! AI-assisted recipe import: from an uploaded PDF or photo, or from a web page.
//! The result is a [`RecipeDraft`] that pre-fills the recipe form; nothing is stored.

use std::sync::Arc;

use async_trait::async_trait;
use tracing::instrument;

use super::recipe::RemoteFetcher;
use crate::{
    app_error::{AppError, AppResult},
    entities::{imported_recipe::ExtractedRecipe, recipe::RecipeDraft},
    validation::{Violations, not_blank},
};

/// What the model is asked to read.
#[derive(Debug, Clone)]
pub enum ImportSource {
    /// Text already extracted from a document or web page.
    Text { label: &'static str, text: String },
    /// A photo (PNG, JPEG, GIF or WebP bytes).
    Image(Vec<u8>),
}

/// The language model that turns a source into a structured recipe.
#[async_trait]
pub trait RecipeExtractor: Send + Sync {
    async fn extract_recipe(
        &self,
        instructions: &str,
        source: ImportSource,
    ) -> AppResult<ExtractedRecipe>;
}

/// Turns documents into plain text suitable for the model.
#[async_trait]
pub trait DocumentReader: Send + Sync {
    async fn pdf_text(&self, pdf: Vec<u8>) -> AppResult<String>;
    fn html_text(&self, html: &str) -> String;
}

/// Maximum size of a web page downloaded for import.
const MAX_PAGE_BYTES: usize = 3 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileKind {
    Pdf,
    Image,
}

#[derive(Clone)]
pub struct RecipeImportUseCases {
    extractor: Arc<dyn RecipeExtractor>,
    documents: Arc<dyn DocumentReader>,
    fetcher: Arc<dyn RemoteFetcher>,
}

impl RecipeImportUseCases {
    pub fn new(
        extractor: Arc<dyn RecipeExtractor>,
        documents: Arc<dyn DocumentReader>,
        fetcher: Arc<dyn RemoteFetcher>,
    ) -> Self {
        Self {
            extractor,
            documents,
            fetcher,
        }
    }

    #[instrument(skip(self, bytes, instructions))]
    pub async fn import_file(
        &self,
        bytes: Vec<u8>,
        content_type: Option<&str>,
        instructions: &str,
    ) -> AppResult<RecipeDraft> {
        validate_instructions(instructions)?;
        let kind = classify(content_type, &bytes).ok_or_else(|| {
            AppError::Unprocessable(format!(
                "Unsupported file type: {}",
                content_type.unwrap_or("unknown")
            ))
        })?;

        let source = match kind {
            FileKind::Pdf => ImportSource::Text {
                label: "Document content",
                text: self.documents.pdf_text(bytes).await?,
            },
            FileKind::Image => ImportSource::Image(bytes),
        };
        Ok(self
            .extractor
            .extract_recipe(instructions, source)
            .await?
            .into_draft())
    }

    #[instrument(skip(self, instructions))]
    pub async fn import_url(&self, url: &str, instructions: &str) -> AppResult<RecipeDraft> {
        validate_instructions(instructions)?;
        let page = self
            .fetcher
            .fetch(url, MAX_PAGE_BYTES)
            .await
            .map_err(|e| match e {
                AppError::Unprocessable(reason) => {
                    AppError::Unprocessable(format!("Failed to fetch URL: {reason}"))
                }
                other => other,
            })?;
        let text = self
            .documents
            .html_text(&String::from_utf8_lossy(&page.body));
        if text.trim().is_empty() {
            return Err(AppError::Unprocessable(
                "The page has no readable content".into(),
            ));
        }
        let source = ImportSource::Text {
            label: "Page content",
            text,
        };
        Ok(self
            .extractor
            .extract_recipe(instructions, source)
            .await?
            .into_draft())
    }
}

fn validate_instructions(instructions: &str) -> AppResult<()> {
    let mut v = Violations::new();
    v.check(not_blank(instructions), "Prompt is required")
        .check(
            instructions.len() <= 4000,
            "Prompt must not exceed 4000 characters",
        );
    Ok(v.finish()?)
}

/// Kind of an uploaded file, from its declared MIME type or, failing that, its magic bytes.
pub fn classify(declared: Option<&str>, bytes: &[u8]) -> Option<FileKind> {
    let declared = declared.map(|m| m.split(';').next().unwrap_or(m).trim().to_ascii_lowercase());
    match declared.as_deref() {
        Some("application/pdf") => return Some(FileKind::Pdf),
        Some("image/png" | "image/jpeg" | "image/jpg" | "image/webp" | "image/gif") => {
            return Some(FileKind::Image);
        }
        _ => {}
    }
    match bytes {
        [b'%', b'P', b'D', b'F', ..] => Some(FileKind::Pdf),
        [0x89, b'P', b'N', b'G', ..] | [0xFF, 0xD8, 0xFF, ..] | [b'G', b'I', b'F', b'8', ..] => {
            Some(FileKind::Image)
        }
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
        ] => Some(FileKind::Image),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{entities::imported_recipe::ExtractedStep, use_cases::fakes::FakeFetcher};

    /// Echoes what it received into the recipe title.
    struct EchoExtractor;

    #[async_trait]
    impl RecipeExtractor for EchoExtractor {
        async fn extract_recipe(
            &self,
            _: &str,
            source: ImportSource,
        ) -> AppResult<ExtractedRecipe> {
            let title = match source {
                ImportSource::Text { label, text } => format!("{label}: {text}"),
                ImportSource::Image(bytes) => format!("image of {} bytes", bytes.len()),
            };
            Ok(ExtractedRecipe {
                title,
                steps: vec![ExtractedStep {
                    step_number: Some(3),
                    instructions: "Eat".into(),
                    duration: None,
                }],
                ..Default::default()
            })
        }
    }

    struct UpperReader;

    #[async_trait]
    impl DocumentReader for UpperReader {
        async fn pdf_text(&self, pdf: Vec<u8>) -> AppResult<String> {
            Ok(String::from_utf8_lossy(&pdf).to_uppercase())
        }
        fn html_text(&self, html: &str) -> String {
            html.replace("<p>", "").replace("</p>", "")
        }
    }

    fn use_cases() -> RecipeImportUseCases {
        RecipeImportUseCases::new(
            Arc::new(EchoExtractor),
            Arc::new(UpperReader),
            Arc::new(FakeFetcher),
        )
    }

    #[tokio::test]
    async fn routes_sources_and_normalizes() {
        let imports = use_cases();
        let d = imports
            .import_file(b"%PDF tortilla".to_vec(), None, "extract")
            .await
            .unwrap();
        assert_eq!(d.title, "Document content: %PDF TORTILLA");
        assert_eq!(d.steps[0].step_number, 1, "normalised by the domain");

        let d = imports
            .import_file(vec![0xFF, 0xD8, 0xFF, 0], Some("image/jpeg"), "extract")
            .await
            .unwrap();
        assert_eq!(d.title, "image of 4 bytes");

        let d = imports
            .import_url("https://example.com/r", "extract")
            .await
            .unwrap();
        assert_eq!(d.title, "Page content: fetched https://example.com/r");
    }

    #[tokio::test]
    async fn rejects_bad_input() {
        let imports = use_cases();
        assert!(matches!(
            imports
                .import_file(b"hello".to_vec(), Some("text/plain"), "x")
                .await,
            Err(AppError::Unprocessable(_))
        ));
        assert!(matches!(
            imports.import_file(b"%PDF".to_vec(), None, "  ").await,
            Err(AppError::Validation(_))
        ));
        let err = imports.import_url("http://fail", "x").await.unwrap_err();
        assert!(matches!(err, AppError::Unprocessable(m) if m.starts_with("Failed to fetch URL")));
    }

    #[test]
    fn classifies_uploads() {
        assert_eq!(classify(Some("application/pdf"), b""), Some(FileKind::Pdf));
        assert_eq!(
            classify(Some("application/octet-stream"), b"%PDF-1.7"),
            Some(FileKind::Pdf)
        );
        assert_eq!(
            classify(None, b"RIFF\0\0\0\0WEBPVP8"),
            Some(FileKind::Image)
        );
        assert_eq!(classify(Some("text/plain"), b"hello"), None);
    }
}
