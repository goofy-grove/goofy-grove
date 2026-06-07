use sea_orm::DbBackend;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const USERS_NEW: &str = "users_new";
const PERSONAS_NEW: &str = "personas_new";

fn supports_inline_avatar_fk_alter(backend: DbBackend) -> bool {
    matches!(backend, DbBackend::Postgres | DbBackend::MySql)
}

fn fk_users_files() -> ForeignKeyCreateStatement {
    ForeignKey::create()
        .name("fk_users_files")
        .from("users", "avatar_uid")
        .to("files", "uid")
        .on_delete(ForeignKeyAction::SetNull)
        .to_owned()
}

fn fk_personas_files() -> ForeignKeyCreateStatement {
    ForeignKey::create()
        .name("fk_personas_files")
        .from("personas", "avatar_uid")
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

async fn add_avatar_columns(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .alter_table(
            Table::alter()
                .table("users")
                .add_column(ColumnDef::new("avatar_uid").string().null())
                .to_owned(),
        )
        .await?;

    manager
        .alter_table(
            Table::alter()
                .table("personas")
                .add_column(ColumnDef::new("avatar_uid").string().null())
                .to_owned(),
        )
        .await?;

    Ok(())
}

async fn add_avatar_foreign_keys(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager.create_foreign_key(fk_users_files()).await?;
    manager.create_foreign_key(fk_personas_files()).await?;
    Ok(())
}

async fn drop_avatar_foreign_keys(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_foreign_key(
            ForeignKey::drop()
                .name("fk_users_files")
                .table("users")
                .to_owned(),
        )
        .await?;

    manager
        .drop_foreign_key(
            ForeignKey::drop()
                .name("fk_personas_files")
                .table("personas")
                .to_owned(),
        )
        .await?;

    Ok(())
}

async fn drop_avatar_columns(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .alter_table(
            Table::alter()
                .table("users")
                .drop_column("avatar_uid")
                .to_owned(),
        )
        .await?;

    manager
        .alter_table(
            Table::alter()
                .table("personas")
                .drop_column("avatar_uid")
                .to_owned(),
        )
        .await?;

    Ok(())
}

async fn rebuild_users_sqlite(manager: &SchemaManager<'_>, with_avatar: bool) -> Result<(), DbErr> {
    set_sqlite_foreign_keys(manager, false).await?;

    if with_avatar {
        manager
            .create_table(
                Table::create()
                    .table(USERS_NEW)
                    .col(string("uid").primary_key())
                    .col(string("name").unique_key())
                    .col(string("password"))
                    .col(string_null("avatar_uid"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_users_files")
                            .from(USERS_NEW, "avatar_uid")
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
                    .table(USERS_NEW)
                    .col(string("uid").primary_key())
                    .col(string("name").unique_key())
                    .col(string("password"))
                    .to_owned(),
            )
            .await?;
    }

    let copy_sql = if with_avatar {
        "INSERT INTO users_new (uid, name, password, avatar_uid) \
         SELECT uid, name, password, NULL FROM users"
    } else {
        "INSERT INTO users_new (uid, name, password) \
         SELECT uid, name, password FROM users"
    };
    manager
        .get_connection()
        .execute_unprepared(copy_sql)
        .await?;

    manager
        .drop_table(Table::drop().table("users").to_owned())
        .await?;

    manager
        .rename_table(Table::rename().table(USERS_NEW, "users").to_owned())
        .await?;

    set_sqlite_foreign_keys(manager, true).await?;

    Ok(())
}

async fn rebuild_personas_sqlite(
    manager: &SchemaManager<'_>,
    with_avatar: bool,
) -> Result<(), DbErr> {
    set_sqlite_foreign_keys(manager, false).await?;

    if with_avatar {
        manager
            .create_table(
                Table::create()
                    .table(PERSONAS_NEW)
                    .col(string("uid").primary_key())
                    .col(string("name"))
                    .col(text("description"))
                    .col(string("creator_id"))
                    .col(string_null("avatar_uid"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_personas_users")
                            .from(PERSONAS_NEW, "creator_id")
                            .to("users", "uid")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_personas_files")
                            .from(PERSONAS_NEW, "avatar_uid")
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
                    .table(PERSONAS_NEW)
                    .col(string("uid").primary_key())
                    .col(string("name"))
                    .col(text("description"))
                    .col(string("creator_id"))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_personas_users")
                            .from(PERSONAS_NEW, "creator_id")
                            .to("users", "uid")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
    }

    let copy_sql = if with_avatar {
        "INSERT INTO personas_new (uid, name, description, creator_id, avatar_uid) \
         SELECT uid, name, description, creator_id, NULL FROM personas"
    } else {
        "INSERT INTO personas_new (uid, name, description, creator_id) \
         SELECT uid, name, description, creator_id FROM personas"
    };
    manager
        .get_connection()
        .execute_unprepared(copy_sql)
        .await?;

    manager
        .drop_table(Table::drop().table("personas").to_owned())
        .await?;

    manager
        .rename_table(Table::rename().table(PERSONAS_NEW, "personas").to_owned())
        .await?;

    set_sqlite_foreign_keys(manager, true).await?;

    Ok(())
}

async fn up_sqlite(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    rebuild_users_sqlite(manager, true).await?;
    rebuild_personas_sqlite(manager, true).await?;
    Ok(())
}

async fn down_sqlite(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    rebuild_personas_sqlite(manager, false).await?;
    rebuild_users_sqlite(manager, false).await?;
    Ok(())
}

async fn up_postgres_or_mysql(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    add_avatar_columns(manager).await?;
    add_avatar_foreign_keys(manager).await?;
    Ok(())
}

async fn down_postgres_or_mysql(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    drop_avatar_foreign_keys(manager).await?;
    drop_avatar_columns(manager).await?;
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
