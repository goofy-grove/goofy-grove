use crate::impl_new_type;

#[cfg(test)]
mod tests;

impl_new_type!(
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct UserAgent(String);
    sanitize: |user_agent: String| user_agent.trim().to_owned();
);
