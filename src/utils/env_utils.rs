use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvOptions {
    pub upload_dir: String,
}

impl EnvOptions {
    pub fn get() -> Self {
        // If we are on a docker enviorment the UPLOAD_DIR env will be set, it will default to assets/recipes on non-docker enviorments.
        let upload_dir =
            std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "/assets/recipes".to_string());

        EnvOptions { upload_dir }
    }
}
