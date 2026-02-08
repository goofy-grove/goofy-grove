use domain::prelude::*;

#[derive(Debug, Clone)]
pub struct UuidGenerator;

impl UserIdGenerator for UuidGenerator {
    fn generate(&self) -> UserId {
        UserId::new(uuid::Uuid::new_v4().to_string())
    }
}
