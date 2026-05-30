use axum::extract::Multipart;
use gg_core::domain::prelude::*;

pub async fn read_multipart_file(
    mut multipart: Multipart,
) -> Result<(FileOriginalName, FileContentType, FileContent), String> {
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

        let original_name = FileOriginalName::try_new(original_name)
            .map_err(|err| format!("invalid original name: {err}"))?;
        let content_type = FileContentType::try_new(content_type)
            .map_err(|err| format!("invalid content type: {err}"))?;

        return Ok((original_name, content_type, FileContent::new(bytes)));
    }

    Err("file field is required".to_string())
}
