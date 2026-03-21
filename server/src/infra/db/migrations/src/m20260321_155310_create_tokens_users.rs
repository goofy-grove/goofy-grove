use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("tokens")
                    .col(string("uid").primary_key())
                    .col(string("hashed_token").unique_key())
                    .col(string("user_id"))
                    .col(string("user_agent"))
                    .col(date_time("last_accessed_at"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_tokens_users")
                            .from("tokens", "user_id")
                            .to("users", "uid")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("tokens").to_owned())
            .await
    }
}
