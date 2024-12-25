use leptos::prelude::*;

#[server(UploadFile, "/api/uploadfile")]
pub async fn upload_file(file_data: Vec<u8>, file_path: String) -> Result<String, ServerFnError> {
    use async_std::fs::{self, File};
    use async_std::io::prelude::*;
    use std::path::Path;

    let path = Path::new(&file_path);

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
