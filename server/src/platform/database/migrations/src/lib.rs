pub use sea_orm_migration::prelude::*;

mod m20261028_174528_add_users_and_files_tables;
mod m20261028_175641_add_personas_table;
mod m20261028_180923_add_characters_table;
mod m20261028_182037_add_tokens_table;
mod m20261028_183412_add_chats_table;
mod m20261028_184528_add_chat_members_table;
mod m20261028_185853_add_chat_characters_table;
mod m20261028_191109_add_messages_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20261028_174528_add_users_and_files_tables::Migration),
            Box::new(m20261028_175641_add_personas_table::Migration),
            Box::new(m20261028_180923_add_characters_table::Migration),
            Box::new(m20261028_182037_add_tokens_table::Migration),
            Box::new(m20261028_183412_add_chats_table::Migration),
            Box::new(m20261028_184528_add_chat_members_table::Migration),
            Box::new(m20261028_185853_add_chat_characters_table::Migration),
            Box::new(m20261028_191109_add_messages_table::Migration),
        ]
    }
}
