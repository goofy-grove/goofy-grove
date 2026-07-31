pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20260217_201018_create_personas_table;
mod m20260321_155310_create_tokens_users;
mod m20260427_082333_add_character;
mod m20260530_132459_add_file_table;
mod m20260530_145742_add_avatar_uid_field;
mod m20260611_145415_add_avatar_uid_field_for_character;
mod m20260611_162629_update_uid_naming_conventions;
mod m20260731_142015_add_chats;
mod m20260731_142806_add_chats_members;
mod m20260731_150107_add_chats_characters;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20260217_201018_create_personas_table::Migration),
            Box::new(m20260321_155310_create_tokens_users::Migration),
            Box::new(m20260427_082333_add_character::Migration),
            Box::new(m20260530_132459_add_file_table::Migration),
            Box::new(m20260530_145742_add_avatar_uid_field::Migration),
            Box::new(m20260611_145415_add_avatar_uid_field_for_character::Migration),
            Box::new(m20260611_162629_update_uid_naming_conventions::Migration),
            Box::new(m20260731_142015_add_chats::Migration),
            Box::new(m20260731_142806_add_chats_members::Migration),
            Box::new(m20260731_150107_add_chats_characters::Migration),
        ]
    }
}
