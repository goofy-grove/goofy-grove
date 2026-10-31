use sea_orm_migration::{
    prelude::*,
    schema::*,
    sea_orm::{DatabaseExecutor, DbBackend, sqlx},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

const TABLES_TO_DROP: &[&str] = &["files", "users"];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let backend = manager.get_database_backend();
        if !matches!(backend, DbBackend::Sqlite | DbBackend::Postgres) {
            return Err(DbErr::Custom(
                "This migration supports SQLite and PostgreSQL".into(),
            ));
        }

        let mut users = Table::create();
        users
            .table("users")
            .col(string("uid").primary_key())
            .col(string("name").unique_key())
            .col(string("password"))
            .col(string_null("avatar_uid"));

        // NOTE: SQLite allows referencing files before the table exists
        if backend == DbBackend::Sqlite {
            users.foreign_key(
                ForeignKey::create()
                    .name("fk_users_files")
                    .from("users", "avatar_uid")
                    .to("files", "uid")
                    .on_delete(ForeignKeyAction::SetNull),
            );
        }
        manager.create_table(users).await?;

        manager
            .create_table(
                Table::create()
                    .table("files")
                    .col(string("uid").primary_key())
                    .col(string("filename").unique_key())
                    .col(string("scope_kind"))
                    .col(string("scope_owner_uid"))
                    .col(string_null("scope_entity_uid"))
                    .col(timestamp_with_time_zone("uploaded_at"))
                    .col(string("status"))
                    .col(string("original_name"))
                    .col(string("content_type"))
                    .col(big_integer("size"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_files_users_scope_owner")
                            .from("files", "scope_owner_uid")
                            .to("users", "uid")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        if backend == DbBackend::Postgres {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_users_files")
                        .from("users", "avatar_uid")
                        .to("files", "uid")
                        .on_delete(ForeignKeyAction::SetNull)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        match manager.get_database_backend() {
            DbBackend::Sqlite => drop_sqlite_tables(manager).await,
            DbBackend::Postgres => {
                manager
                    .drop_foreign_key(
                        ForeignKey::drop()
                            .name("fk_users_files")
                            .table("users")
                            .to_owned(),
                    )
                    .await?;

                for table in TABLES_TO_DROP {
                    manager
                        .drop_table(Table::drop().table(*table).to_owned())
                        .await?;
                }
                Ok(())
            }
            _ => Err(DbErr::Custom(
                "This migration supports SQLite and PostgreSQL".into(),
            )),
        }
    }
}

fn map_sqlite_error(error: sqlx::Error) -> DbErr {
    DbErr::Custom(format!("SQLite schema rollback failed: {error}"))
}

async fn drop_sqlite_tables(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let DatabaseExecutor::Connection(db) = manager.get_connection() else {
        return Err(DbErr::Custom(
            "Run SQLite rollback outside an enclosing transaction".into(),
        ));
    };

    let mut connection = db
        .get_sqlite_connection_pool()
        .acquire()
        .await
        .map_err(map_sqlite_error)?
        .detach();

    sqlx::query("PRAGMA foreign_keys = OFF")
        .execute(&mut connection)
        .await
        .map_err(map_sqlite_error)?;

    let mut transaction = sqlx::Connection::begin(&mut connection)
        .await
        .map_err(map_sqlite_error)?;

    for table in TABLES_TO_DROP {
        sqlx::query(&format!("DROP TABLE \"{table}\""))
            .execute(&mut *transaction)
            .await
            .map_err(map_sqlite_error)?;
    }

    transaction.commit().await.map_err(map_sqlite_error)
}
