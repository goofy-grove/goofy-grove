use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    #[serde(default = "default_files_dir")]
    pub files_dir: String,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            files_dir: default_files_dir(),
        }
    }
}

fn default_files_dir() -> String {
    "./data".to_string()
}
