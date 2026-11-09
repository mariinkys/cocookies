//! [`DocumentReader`] implementation: PDF text via lopdf, HTML text via a small tag stripper.

mod text;

use async_trait::async_trait;

use crate::{
    app_error::{AppError, AppResult},
    use_cases::recipe_import::DocumentReader,
};

#[derive(Default)]
pub struct TextDocumentReader;

#[async_trait]
impl DocumentReader for TextDocumentReader {
    /// CPU-bound parsing runs on the blocking pool.
    async fn pdf_text(&self, pdf: Vec<u8>) -> AppResult<String> {
        tokio::task::spawn_blocking(move || text::pdf_text(&pdf))
            .await
            .map_err(AppError::internal)?
            .map_err(|e| AppError::Unprocessable(format!("Failed to process file: {e}")))
    }

    fn html_text(&self, html: &str) -> String {
        text::html_text(html)
    }
}
