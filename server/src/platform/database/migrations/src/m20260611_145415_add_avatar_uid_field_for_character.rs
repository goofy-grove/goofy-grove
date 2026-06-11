use sea_orm::DbBackend;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const CHARACTERS_NEW: &str = "characters_new";

fn supports_inline_avatar_fk_alter(backend: DbBackend) -> bool {
    matches!(backend, DbBackend::Postgres | DbBackend::MySql)
}

fn fk_characters_files() -> ForeignKeyCreateStatement {
    ForeignKey::create()
        .name("fk_characters_files")
        .from("characters", "avatar_uid")
        .to("files", "uid")
        .on_delete(ForeignKeyAction::SetNull)
        .to_owned()
}

async fn set_sqlite_foreign_keys(manager: &SchemaManager<'_>, enabled: bool) -> Result<(), DbErr> {
    let pragma = if enabled {
        "PRAGMA foreign_keys = ON"
    } else {
        "PRAGMA foreign_keys = OFF"
    };
    manager
        .get_connection()
        .execute_unprepared(pragma)
        .await
        .map(|_| ())
}

async fn add_avatar_column(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .alter_table(
            Table::alter()
                .table("characters")
                .add_column(ColumnDef::new("avatar_uid").string().null())
                .to_owned(),
        )
        .await?;

    Ok(())
}

async fn add_avatar_foreign_key(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager.create_foreign_key(fk_characters_files()).await?;
    Ok(())
}

async fn drop_avatar_foreign_key(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_foreign_key(
            ForeignKey::drop()
                .name("fk_characters_files")
                .table("characters")
                .to_owned(),
        )
        .await?;

    Ok(())
}

async fn drop_avatar_column(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .alter_table(
            Table::alter()
                .table("characters")
                .drop_column("avatar_uid")
                .to_owned(),
        )
        .await?;

    Ok(())
}

async fn rebuild_characters_sqlite(
    manager: &SchemaManager<'_>,
    with_avatar: bool,
) -> Result<(), DbErr> {
    set_sqlite_foreign_keys(manager, false).await?;

    if with_avatar {
        manager
            .create_table(
                Table::create()
                    .table(CHARACTERS_NEW)
                    .col(string("uid").primary_key())
                    .col(string("name"))
                    .col(text("description"))
                    .col(string("user_id"))
                    .col(string_null("avatar_uid"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_characters_users")
                            .from(CHARACTERS_NEW, "user_id")
                            .to("users", "uid")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_characters_files")
                            .from(CHARACTERS_NEW, "avatar_uid")
                            .to("files", "uid")
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;
    } else {
        manager
            .create_table(
                Table::create()
                    .table(CHARACTERS_NEW)
                    .col(string("uid").primary_key())
                    .col(string("name"))
                    .col(text("description"))
                    .col(string("user_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_characters_users")
                            .from(CHARACTERS_NEW, "user_id")
                            .to("users", "uid")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
    }

    let copy_sql = if with_avatar {
        "INSERT INTO characters_new (uid, name, description, user_id, avatar_uid) \
         SELECT uid, name, description, user_id, NULL FROM characters"
    } else {
        "INSERT INTO characters_new (uid, name, description, user_id) \
         SELECT uid, name, description, user_id FROM characters"
    };
    manager
        .get_connection()
        .execute_unprepared(copy_sql)
        .await?;

    manager
        .drop_table(Table::drop().table("characters").to_owned())
        .await?;

    manager
        .rename_table(Table::rename().table(CHARACTERS_NEW, "characters").to_owned())
        .await?;

    set_sqlite_foreign_keys(manager, true).await?;

    Ok(())
}

async fn up_sqlite(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    rebuild_characters_sqlite(manager, true).await?;
    Ok(())
}

async fn down_sqlite(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    rebuild_characters_sqlite(manager, false).await?;
    Ok(())
}

async fn up_postgres_or_mysql(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    add_avatar_column(manager).await?;
    add_avatar_foreign_key(manager).await?;
    Ok(())
}

async fn down_postgres_or_mysql(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    drop_avatar_foreign_key(manager).await?;
    drop_avatar_column(manager).await?;
    Ok(())
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if supports_inline_avatar_fk_alter(manager.get_database_backend()) {
            up_postgres_or_mysql(manager).await
        } else if manager.get_database_backend() == DbBackend::Sqlite {
            up_sqlite(manager).await
        } else {
            Err(DbErr::Custom(format!(
                "Unsupported database backend for migration: {:?}",
                manager.get_database_backend()
            )))
        }
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if supports_inline_avatar_fk_alter(manager.get_database_backend()) {
            down_postgres_or_mysql(manager).await
        } else if manager.get_database_backend() == DbBackend::Sqlite {
            down_sqlite(manager).await
        } else {
            Err(DbErr::Custom(format!(
                "Unsupported database backend for migration: {:?}",
                manager.get_database_backend()
            )))
        }
    }
}
