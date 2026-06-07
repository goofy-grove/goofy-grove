use axum::extract::Multipart;

pub async fn read_multipart_file(
    mut multipart: Multipart,
) -> Result<(String, String, Vec<u8>), String> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|err| err.to_string())?
    {
        if field.name() != Some("file") {
            continue;
        }

        let original_name = field.file_name().unwrap_or("upload.bin").to_string();
        let content_type = field
            .content_type()
            .map(str::to_string)
            .unwrap_or_else(|| "application/octet-stream".to_string());
        let bytes = field.bytes().await.map_err(|err| err.to_string())?.to_vec();

        if original_name.trim().is_empty() {
            return Err("invalid original name".to_string());
        }

        if content_type.trim().is_empty() {
            return Err("invalid content type".to_string());
        }

        return Ok((original_name, content_type, bytes));
    }

    Err("file field is required".to_string())
}
