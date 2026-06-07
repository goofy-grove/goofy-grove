use std::path::Path;

pub fn resolve_filename(file_id: &str, original_name: &str) -> String {
    let extension = Path::new(original_name)
        .extension()
        .and_then(|value| value.to_str())
        .map(|ext| format!(".{ext}"))
        .unwrap_or_default();

    format!("{file_id}{extension}")
}
