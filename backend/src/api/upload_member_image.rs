use crate::api::api_error::ApiError;
use axum::extract::Multipart;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn upload_member_image(mut multipart: Multipart) -> Result<(), ApiError> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();

        println!("Field Name: {name}");
        println!("File Name: {file_name}");
        println!("Content Type: {content_type}");

        let data = field.bytes().await.unwrap();

        let file_path = format!("./uploads/{file_name}");

        let mut file = File::create(&file_path).await.unwrap();
        file.write(&data).await.unwrap();

        println!("Saved file to: {file_path}");
    }

    Ok(())
}
