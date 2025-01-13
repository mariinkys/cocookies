use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[server(UploadFile, "/api/uploadfile")]
pub async fn upload_file(file_data: Vec<u8>, file_path: String) -> Result<String, ServerFnError> {
    use async_std::fs::{self, File};
    use async_std::io::prelude::*;
    use std::path::Path;

    // TODO: This is awful
    let fpath = if file_path.starts_with('/') && file_path.starts_with("/assets/") {
        file_path.strip_prefix("/").unwrap_or_default()
    } else {
        &file_path
    };

    let path = Path::new(&fpath);

    // Check if the file path has a parent directory
    let parent_dir = path.parent();

    // If there is a parent directory, else don't create
    if let Some(parent) = parent_dir {
        if fs::create_dir_all(parent).await.is_err() {
            return Err(ServerFnError::new("Error Creating Directory Path"));
        }
    } else {
        return Err(ServerFnError::new(format!("Wrong Path: {}", file_path)));
    }

    // Now create the file itself
    let file = File::create(path).await;
    match file {
        Ok(mut file) => {
            let res = file.write_all(&file_data).await;
            match res {
                Ok(_) => Ok(format!("File uploaded to {}", file_path)),
                Err(err) => Err(ServerFnError::new(format!("Server Error: {}", err))),
            }
        }
        Err(err) => Err(ServerFnError::new(format!("Server Error: {}", err))),
    }
}

#[server(DeleteFile, "/api/uploadfile")]
pub async fn delete_file(file_path: String) -> Result<String, ServerFnError> {
    use async_std::fs;
    use async_std::path::Path;

    // TODO: This is awful
    let fpath = if file_path.starts_with('/') && file_path.starts_with("/assets/") {
        file_path.strip_prefix("/").unwrap_or_default()
    } else {
        &file_path
    };

    let file_path = Path::new(&fpath);

    // Check if the file exists asynchronously
    if !file_path.exists().await {
        return Err(ServerFnError::ServerError(format!(
            "File not found: {}",
            file_path.display()
        )));
    }

    // Attempt to delete the file asynchronously
    match fs::remove_file(file_path).await {
        Ok(_) => Ok(format!(
            "File deleted successfully: {}",
            file_path.display()
        )),
        Err(e) => Err(ServerFnError::ServerError(format!(
            "Failed to delete file: {}. Error: {}",
            file_path.display(),
            e
        ))),
    }
}

pub fn get_file_extension(file: &web_sys::File) -> Option<&'static str> {
    match file.type_().as_str() {
        "image/png" => Some("png"),
        "image/jpeg" => Some("jpeg"),
        "image/jpg" => Some("jpg"),
        "image/webp" => Some("webp"),
        "image/gif" => None,
        "image/svg+xml" => None,
        _ => None, // Handle unsupported file types
    }
}

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
