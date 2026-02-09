use domain::prelude::*;

#[derive(Debug, Clone)]
pub struct UuidGenerator;

impl IdGenerator for UuidGenerator {
    fn generate(&self) -> String {
        uuid::Uuid::new_v4().to_string()
    }
}
