use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("persons")
                    .if_not_exists()
                    .col(string("uid").primary_key())
                    .col(string("name"))
                    .col(text("description"))
                    .col(string("creator_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_persons_users")
                            .from("persons", "creator_id")
                            .to("users", "uid")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("persons").to_owned())
            .await
    }
}
