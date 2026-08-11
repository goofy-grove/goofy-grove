use serde::Deserialize;

use crate::file::FileScope;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PoliciesConfig {
    #[serde(default)]
    pub files: FilesPoliciesConfig,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct FilesPoliciesConfig {
    #[serde(default)]
    pub user_avatar: FilePolicyConfig,
    #[serde(default)]
    pub persona_avatar: FilePolicyConfig,
    #[serde(default)]
    pub character_avatar: FilePolicyConfig,
    #[serde(default)]
    pub chat_avatar: FilePolicyConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilePolicyConfig {
    #[serde(default = "default_max_file_size")]
    pub max_file_size: FileSizeConfig,
    #[serde(default = "default_allowed_content_types")]
    pub allowed_content_types: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FileSizeConfig(
    pub u64,
    #[serde(deserialize_with = "deserialize_size_unit")] pub SizeUnit,
);

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SizeUnit {
    B,
    KB,
    MB,
    GB,
}

impl FileSizeConfig {
    pub fn to_bytes(&self) -> u64 {
        let multiplier = match self.1 {
            SizeUnit::B => 1,
            SizeUnit::KB => 1024,
            SizeUnit::MB => 1024 * 1024,
            SizeUnit::GB => 1024 * 1024 * 1024,
        };

        self.0.saturating_mul(multiplier)
    }
}

fn default_max_file_size() -> FileSizeConfig {
    FileSizeConfig(5, SizeUnit::MB)
}

fn default_allowed_content_types() -> Vec<String> {
    vec![
        "image/jpeg".to_string(),
        "image/png".to_string(),
        "image/gif".to_string(),
    ]
}

fn deserialize_size_unit<'de, D>(deserializer: D) -> Result<SizeUnit, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.to_uppercase().as_str() {
        "B" => Ok(SizeUnit::B),
        "KB" => Ok(SizeUnit::KB),
        "MB" => Ok(SizeUnit::MB),
        "GB" => Ok(SizeUnit::GB),
        other => Err(serde::de::Error::custom(format!(
            "unknown size unit: {other}"
        ))),
    }
}

impl Default for FilePolicyConfig {
    fn default() -> Self {
        Self {
            max_file_size: default_max_file_size(),
            allowed_content_types: default_allowed_content_types(),
        }
    }
}

impl FilesPoliciesConfig {
    pub fn policy_for_scope(&self, scope: &FileScope) -> Option<&FilePolicyConfig> {
        let config = match scope {
            FileScope::UserAvatar { .. } => &self.user_avatar,
            FileScope::PersonaAvatar { .. } => &self.persona_avatar,
            FileScope::CharacterAvatar { .. } => &self.character_avatar,
            FileScope::ChatAvatar { .. } => &self.chat_avatar,
        };

        if config.allowed_content_types.is_empty() && config.max_file_size.0 == 0 {
            return None;
        }

        Some(config)
    }

    pub fn max_file_size_bytes(&self) -> u64 {
        [
            self.user_avatar.max_file_size.to_bytes(),
            self.persona_avatar.max_file_size.to_bytes(),
            self.character_avatar.max_file_size.to_bytes(),
        ]
        .into_iter()
        .max()
        .unwrap_or(0)
    }
}

impl PoliciesConfig {
    pub fn max_upload_body_limit(&self) -> usize {
        const MULTIPART_OVERHEAD: u64 = 1024 * 1024;

        self.files
            .max_file_size_bytes()
            .saturating_add(MULTIPART_OVERHEAD) as usize
    }
}
