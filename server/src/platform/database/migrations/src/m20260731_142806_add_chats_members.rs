use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("chat_members")
                    .if_not_exists()
                    .col(string("chat_uid").not_null())
                    .col(string("user_uid").not_null())
                    .col(
                        timestamp("joined_at")
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk_chat_members")
                            .col("chat_uid")
                            .col("user_uid"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_chat_members_chats")
                            .from("chat_members", "chat_uid")
                            .to("chats", "uid")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_chat_members_users")
                            .from("chat_members", "user_uid")
                            .to("users", "uid")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("chat_members").to_owned())
            .await
    }
}
