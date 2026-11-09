//! Minimal client for Ollama's `/api/chat` endpoint (JSON mode, non-streaming).

use std::time::Duration;

use async_trait::async_trait;
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};
use tokio::sync::Semaphore;
use tracing::{error, warn};

use super::{prompt, response::parse_model_output};
use crate::{
    app_error::{AppError, AppResult},
    entities::imported_recipe::ExtractedRecipe,
    use_cases::recipe_import::{ImportSource, RecipeExtractor},
};

pub struct OllamaRecipeExtractor {
    client: reqwest::Client,
    chat_url: String,
    model: String,
    /// Bounds concurrent generations: each may hold an upload in memory, and the model
    /// serialises requests anyway.
    permits: Semaphore,
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    stream: bool,
    format: &'static str,
    options: Options,
    messages: [Message<'a>; 1],
}

#[derive(Serialize)]
struct Options {
    temperature: f32,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'static str,
    content: &'a str,
    #[serde(skip_serializing_if = "<[String]>::is_empty")]
    images: &'a [String],
}

#[derive(Deserialize)]
struct ChatResponse {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

impl OllamaRecipeExtractor {
    pub fn new(
        base_url: &str,
        model: &str,
        timeout: Duration,
        max_concurrency: usize,
    ) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(timeout)
            .build()?;
        Ok(Self {
            client,
            chat_url: format!("{}/api/chat", base_url.trim_end_matches('/')),
            model: model.to_owned(),
            permits: Semaphore::new(max_concurrency.max(1)),
        })
    }

    async fn chat_json(&self, content: &str, images: &[String]) -> AppResult<String> {
        let request = ChatRequest {
            model: &self.model,
            stream: false,
            format: "json",
            options: Options { temperature: 0.1 },
            messages: [Message {
                role: "user",
                content,
                images,
            }],
        };

        let response = self
            .client
            .post(&self.chat_url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                if e.is_connect() || e.is_timeout() {
                    warn!(error = %e, "LLM not reachable");
                    AppError::Unprocessable("LLM service is not available".into())
                } else {
                    error!(error = %e, "LLM request failed");
                    failed()
                }
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!(%status, body = %body.chars().take(500).collect::<String>(), "LLM returned an error");
            return Err(failed());
        }

        let parsed: ChatResponse = response.json().await.map_err(|e| {
            error!(error = %e, "unexpected LLM response shape");
            failed()
        })?;
        Ok(parsed.message.content)
    }
}

#[async_trait]
impl RecipeExtractor for OllamaRecipeExtractor {
    async fn extract_recipe(
        &self,
        instructions: &str,
        source: ImportSource,
    ) -> AppResult<ExtractedRecipe> {
        let _permit = self.permits.acquire().await.map_err(AppError::internal)?;

        let reply = match source {
            ImportSource::Text { label, text } => {
                self.chat_json(&prompt::with_source(instructions, label, &text), &[])
                    .await?
            }
            ImportSource::Image(bytes) => {
                let image = STANDARD.encode(&bytes);
                drop(bytes);
                self.chat_json(&prompt::for_image(instructions), &[image])
                    .await?
            }
        };

        parse_model_output(&reply).ok_or_else(|| {
            error!(reply = %reply.chars().take(1000).collect::<String>(), "unparseable LLM reply");
            AppError::Unprocessable("LLM returned unparseable response".into())
        })
    }
}

fn failed() -> AppError {
    AppError::Unprocessable("Failed to get LLM response".into())
}
