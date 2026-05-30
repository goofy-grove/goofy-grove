use std::path::Path;
use std::sync::Arc;

use gg_core::domain::prelude::*;

use crate::infra::config::PoliciesConfig;

#[derive(Debug, Clone)]
pub struct ExtensionResolveFilename;

impl ResolveFilenamePort for ExtensionResolveFilename {
    async fn resolve_filename(
        &self,
        file_id: &FileId,
        original_name: &FileOriginalName,
    ) -> Result<Filename, ResolveFilenamePortError> {
        let extension = Path::new(original_name.inner())
            .extension()
            .and_then(|value| value.to_str())
            .map(|ext| format!(".{ext}"))
            .unwrap_or_default();

        Filename::try_new(format!("{}{extension}", file_id.inner()))
            .map_err(|err| ResolveFilenamePortError::InternalError(err.to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct ConfigScopePolicyLoader {
    policies: Arc<PoliciesConfig>,
}

impl ConfigScopePolicyLoader {
    pub fn new(policies: Arc<PoliciesConfig>) -> Self {
        Self { policies }
    }
}

impl LoadScopePolicyPort for ConfigScopePolicyLoader {
    async fn load_scope_policy(
        &self,
        scope: &FileScope,
    ) -> Result<FilePolicy, LoadScopePolicyPortError> {
        self.policies
            .files
            .to_domain_policy(scope)
            .ok_or(LoadScopePolicyPortError::PolicyForScopeNotFound)
    }
}
