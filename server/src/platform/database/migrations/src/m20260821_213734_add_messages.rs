use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("messages")
                    .if_not_exists()
                    .col(string("uid").primary_key())
                    .col(text("content"))
                    .col(string("author_persona_uid").null())
                    .col(string("author_character_uid").null())
                    .col(string("chat_uid"))
                    .col(string("reply_to_message_uid").null())
                    .col(boolean("is_removed").default(false))
                    .col(
                        timestamp("created_at")
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_messages_personas")
                            .from("messages", "author_persona_uid")
                            .to("personas", "uid")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_messages_characters")
                            .from("messages", "author_character_uid")
                            .to("characters", "uid")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_messages_chats")
                            .from("messages", "chat_uid")
                            .to("chats", "uid")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_messages_messages")
                            .from("messages", "reply_to_message_uid")
                            .to("messages", "uid")
                            .on_delete(ForeignKeyAction::NoAction),
                    )
                    .check((
                        "ck_message_at_most_one_author",
                        Expr::col("author_persona_uid")
                            .is_null()
                            .or(Expr::col("author_character_uid").is_null()),
                    ))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("messages").to_owned())
            .await
    }
}
