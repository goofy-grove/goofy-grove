use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("files")
                    .if_not_exists()
                    .col(string("uid").primary_key())
                    .col(string("filename").unique_key())
                    .col(string("uploaded_by"))
                    .col(string("scope_kind"))
                    .col(string("scope_owner_id"))
                    .col(string("scope_entity_id").null())
                    .col(date_time("uploaded_at"))
                    .col(string("status"))
                    .col(string("original_name"))
                    .col(string("content_type"))
                    .col(integer("size"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_files_users_uploaded")
                            .from("files", "uploaded_by")
                            .to("users", "uid")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_files_users_scope_owner")
                            .from("files", "scope_owner_id")
                            .to("users", "uid")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("files").to_owned())
            .await
    }
}
