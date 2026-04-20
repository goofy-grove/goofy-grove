use crate::impl_new_type;

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct PersonaDescription(String);
    sanitize: |description: String| description.trim().to_owned();
);
