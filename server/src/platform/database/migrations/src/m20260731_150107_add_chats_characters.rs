use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("chat_characters")
                    .if_not_exists()
                    .col(string("chat_uid").not_null())
                    .col(string("character_uid").not_null())
                    .col(
                        timestamp("connected_at")
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .name("pk_chat_characters")
                            .col("chat_uid")
                            .col("character_uid"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_chat_characters_chats")
                            .from("chat_characters", "chat_uid")
                            .to("chats", "uid")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_chat_characters_characters")
                            .from("chat_characters", "character_uid")
                            .to("characters", "uid")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("chat_characters").to_owned())
            .await
    }
}
