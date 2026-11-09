//! [`RecipeExtractor`](crate::use_cases::recipe_import::RecipeExtractor) implementation backed
//! by a local Ollama model.

mod ollama;
mod prompt;
mod response;

pub use ollama::OllamaRecipeExtractor;
