use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("chats")
                    .col(string("uid").primary_key())
                    .col(string("name"))
                    .col(string_null("avatar_uid"))
                    .col(string("creator_uid"))
                    .col(timestamp("created_at").default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_chats_files")
                            .from("chats", "avatar_uid")
                            .to("files", "uid")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_chats_users")
                            .from("chats", "creator_uid")
                            .to("users", "uid")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("chats").to_owned())
            .await
    }
}
