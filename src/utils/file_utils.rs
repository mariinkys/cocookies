// SPDX-License-Identifier: GPL-3.0-only

use leptos::prelude::*;

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

    let parent_dir = path.parent();

    // if there is a parent directory, else don't create
    if let Some(parent) = parent_dir {
        if fs::create_dir_all(parent).await.is_err() {
            return Err(ServerFnError::new("Error Creating Directory Path"));
        }
    } else {
        return Err(ServerFnError::new(format!("Wrong Path: {file_path}")));
    }

    // create the file
    let file = File::create(path).await;
    match file {
        Ok(mut file) => {
            let res = file.write_all(&file_data).await;
            match res {
                Ok(_) => Ok(format!("File uploaded to {file_path}")),
                Err(err) => Err(ServerFnError::new(format!("Server Error: {err}"))),
            }
        }
        Err(err) => Err(ServerFnError::new(format!("Server Error: {err}"))),
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

    if !file_path.exists().await {
        return Err(ServerFnError::ServerError(format!(
            "File not found: {}",
            file_path.display()
        )));
    }

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
        _ => None,
    }
}
