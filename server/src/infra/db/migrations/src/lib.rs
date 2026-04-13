pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20260217_201018_create_personas_table;
mod m20260321_155310_create_tokens_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20260217_201018_create_personas_table::Migration),
            Box::new(m20260321_155310_create_tokens_users::Migration),
        ]
    }
}
