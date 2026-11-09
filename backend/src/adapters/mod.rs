//! Adapters: concrete implementations of the application's ports, and the HTTP interface.
//!
//! | adapter        | implements                                                       |
//! |----------------|------------------------------------------------------------------|
//! | `http`         | the REST API (axum) that drives the use cases                    |
//! | `persistence`  | every `*Persistence` port + `StorageMaintenance` on SQLite (sqlx) |
//! | `crypto`       | `PasswordHasher` (bcrypt), `AccessTokenService` (JWT), `RefreshTokenFactory` |
//! | `pdf`          | `RecipeRenderer` (krilla)                                        |
//! | `llm`          | `RecipeExtractor` (Ollama)                                       |
//! | `documents`    | `DocumentReader` (PDF / HTML to text)                            |
//! | `net`          | `RemoteFetcher` (SSRF-safe HTTP client)                          |

pub mod crypto;
pub mod documents;
pub mod http;
pub mod llm;
pub mod net;
pub mod pdf;
pub mod persistence;
