use sea_orm::DbBackend;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

const TOKENS_NEW: &str = "tokens_new";
const FILES_NEW: &str = "files_new";
const PERSONAS_NEW: &str = "personas_new";
const CHARACTERS_NEW: &str = "characters_new";

fn supports_inline_column_rename(backend: DbBackend) -> bool {
    matches!(backend, DbBackend::Postgres | DbBackend::MySql)
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

async fn rebuild_tokens_sqlite(manager: &SchemaManager<'_>, new_names: bool) -> Result<(), DbErr> {
    set_sqlite_foreign_keys(manager, false).await?;

    let user_col = if new_names { "user_uid" } else { "user_id" };

    manager
        .create_table(
            Table::create()
                .table(TOKENS_NEW)
                .col(string("uid").primary_key())
                .col(string("hashed_token").unique_key())
                .col(string(user_col))
                .col(string("user_agent"))
                .col(date_time("last_accessed_at"))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_tokens_users")
                        .from(TOKENS_NEW, user_col)
                        .to("users", "uid")
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;

    let copy_sql = if new_names {
        format!(
            "INSERT INTO {TOKENS_NEW} (uid, hashed_token, user_uid, user_agent, last_accessed_at) \
             SELECT uid, hashed_token, user_id, user_agent, last_accessed_at FROM tokens"
        )
    } else {
        format!(
            "INSERT INTO {TOKENS_NEW} (uid, hashed_token, user_id, user_agent, last_accessed_at) \
             SELECT uid, hashed_token, user_uid, user_agent, last_accessed_at FROM tokens"
        )
    };
    manager
        .get_connection()
        .execute_unprepared(&copy_sql)
        .await?;

    manager
        .drop_table(Table::drop().table("tokens").to_owned())
        .await?;

    manager
        .rename_table(Table::rename().table(TOKENS_NEW, "tokens").to_owned())
        .await?;

    set_sqlite_foreign_keys(manager, true).await?;

    Ok(())
}

async fn rebuild_files_sqlite(manager: &SchemaManager<'_>, new_names: bool) -> Result<(), DbErr> {
    set_sqlite_foreign_keys(manager, false).await?;

    let (uploaded_col, owner_col, entity_col) = if new_names {
        ("uploaded_by_uid", "scope_owner_uid", "scope_entity_uid")
    } else {
        ("uploaded_by", "scope_owner_id", "scope_entity_id")
    };

    manager
        .create_table(
            Table::create()
                .table(FILES_NEW)
                .col(string("uid").primary_key())
                .col(string("filename").unique_key())
                .col(string(uploaded_col))
                .col(string("scope_kind"))
                .col(string(owner_col))
                .col(string_null(entity_col))
                .col(date_time("uploaded_at"))
                .col(string("status"))
                .col(string("original_name"))
                .col(string("content_type"))
                .col(integer("size"))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_files_users_uploaded")
                        .from(FILES_NEW, uploaded_col)
                        .to("users", "uid")
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_files_users_scope_owner")
                        .from(FILES_NEW, owner_col)
                        .to("users", "uid")
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
        )
        .await?;

    let copy_sql = if new_names {
        format!(
            "INSERT INTO {FILES_NEW} (uid, filename, uploaded_by_uid, scope_kind, scope_owner_uid, scope_entity_uid, uploaded_at, status, original_name, content_type, size) \
             SELECT uid, filename, uploaded_by, scope_kind, scope_owner_id, scope_entity_id, uploaded_at, status, original_name, content_type, size FROM files"
        )
    } else {
        format!(
            "INSERT INTO {FILES_NEW} (uid, filename, uploaded_by, scope_kind, scope_owner_id, scope_entity_id, uploaded_at, status, original_name, content_type, size) \
             SELECT uid, filename, uploaded_by_uid, scope_kind, scope_owner_uid, scope_entity_uid, uploaded_at, status, original_name, content_type, size FROM files"
        )
    };
    manager
        .get_connection()
        .execute_unprepared(&copy_sql)
        .await?;

    manager
        .drop_table(Table::drop().table("files").to_owned())
        .await?;

    manager
        .rename_table(Table::rename().table(FILES_NEW, "files").to_owned())
        .await?;

    set_sqlite_foreign_keys(manager, true).await?;

    Ok(())
}

async fn rebuild_personas_sqlite(
    manager: &SchemaManager<'_>,
    new_names: bool,
) -> Result<(), DbErr> {
    set_sqlite_foreign_keys(manager, false).await?;

    let creator_col = if new_names {
        "creator_uid"
    } else {
        "creator_id"
    };

    manager
        .create_table(
            Table::create()
                .table(PERSONAS_NEW)
                .col(string("uid").primary_key())
                .col(string("name"))
                .col(text("description"))
                .col(string(creator_col))
                .col(string_null("avatar_uid"))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_personas_users")
                        .from(PERSONAS_NEW, creator_col)
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

    let copy_sql = if new_names {
        format!(
            "INSERT INTO {PERSONAS_NEW} (uid, name, description, creator_uid, avatar_uid) \
             SELECT uid, name, description, creator_id, avatar_uid FROM personas"
        )
    } else {
        format!(
            "INSERT INTO {PERSONAS_NEW} (uid, name, description, creator_id, avatar_uid) \
             SELECT uid, name, description, creator_uid, avatar_uid FROM personas"
        )
    };
    manager
        .get_connection()
        .execute_unprepared(&copy_sql)
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

async fn rebuild_characters_sqlite(
    manager: &SchemaManager<'_>,
    new_names: bool,
) -> Result<(), DbErr> {
    set_sqlite_foreign_keys(manager, false).await?;

    let creator_col = if new_names { "creator_uid" } else { "user_id" };

    manager
        .create_table(
            Table::create()
                .table(CHARACTERS_NEW)
                .col(string("uid").primary_key())
                .col(string("name"))
                .col(text("description"))
                .col(string(creator_col))
                .col(string_null("avatar_uid"))
                .foreign_key(
                    ForeignKey::create()
                        .name("fk_characters_users")
                        .from(CHARACTERS_NEW, creator_col)
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

    let copy_sql = if new_names {
        format!(
            "INSERT INTO {CHARACTERS_NEW} (uid, name, description, creator_uid, avatar_uid) \
             SELECT uid, name, description, user_id, avatar_uid FROM characters"
        )
    } else {
        format!(
            "INSERT INTO {CHARACTERS_NEW} (uid, name, description, user_id, avatar_uid) \
             SELECT uid, name, description, creator_uid, avatar_uid FROM characters"
        )
    };
    manager
        .get_connection()
        .execute_unprepared(&copy_sql)
        .await?;

    manager
        .drop_table(Table::drop().table("characters").to_owned())
        .await?;

    manager
        .rename_table(
            Table::rename()
                .table(CHARACTERS_NEW, "characters")
                .to_owned(),
        )
        .await?;

    set_sqlite_foreign_keys(manager, true).await?;

    Ok(())
}

async fn up_sqlite(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    rebuild_tokens_sqlite(manager, true).await?;
    rebuild_files_sqlite(manager, true).await?;
    rebuild_personas_sqlite(manager, true).await?;
    rebuild_characters_sqlite(manager, true).await?;
    Ok(())
}

async fn down_sqlite(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    rebuild_characters_sqlite(manager, false).await?;
    rebuild_personas_sqlite(manager, false).await?;
    rebuild_files_sqlite(manager, false).await?;
    rebuild_tokens_sqlite(manager, false).await?;
    Ok(())
}

async fn drop_files_foreign_keys(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_foreign_key(
            ForeignKey::drop()
                .name("fk_files_users_uploaded")
                .table("files")
                .to_owned(),
        )
        .await?;

    manager
        .drop_foreign_key(
            ForeignKey::drop()
                .name("fk_files_users_scope_owner")
                .table("files")
                .to_owned(),
        )
        .await?;

    Ok(())
}

async fn create_files_foreign_keys_new(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_foreign_key(
            ForeignKey::create()
                .name("fk_files_users_uploaded")
                .from("files", "uploaded_by_uid")
                .to("users", "uid")
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        )
        .await?;

    manager
        .create_foreign_key(
            ForeignKey::create()
                .name("fk_files_users_scope_owner")
                .from("files", "scope_owner_uid")
                .to("users", "uid")
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        )
        .await?;

    Ok(())
}

async fn create_files_foreign_keys_old(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_foreign_key(
            ForeignKey::create()
                .name("fk_files_users_uploaded")
                .from("files", "uploaded_by")
                .to("users", "uid")
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        )
        .await?;

    manager
        .create_foreign_key(
            ForeignKey::create()
                .name("fk_files_users_scope_owner")
                .from("files", "scope_owner_id")
                .to("users", "uid")
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        )
        .await?;

    Ok(())
}

async fn drop_personas_users_fk(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_foreign_key(
            ForeignKey::drop()
                .name("fk_personas_users")
                .table("personas")
                .to_owned(),
        )
        .await
}

async fn drop_characters_users_fk(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_foreign_key(
            ForeignKey::drop()
                .name("fk_characters_users")
                .table("characters")
                .to_owned(),
        )
        .await
}

async fn drop_tokens_users_fk(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_foreign_key(
            ForeignKey::drop()
                .name("fk_tokens_users")
                .table("tokens")
                .to_owned(),
        )
        .await
}

async fn rename_columns_up(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    drop_characters_users_fk(manager).await?;
    manager
        .alter_table(
            Table::alter()
                .table("characters")
                .rename_column(Alias::new("user_id"), Alias::new("creator_uid"))
                .to_owned(),
        )
        .await?;
    manager
        .create_foreign_key(
            ForeignKey::create()
                .name("fk_characters_users")
                .from("characters", "creator_uid")
                .to("users", "uid")
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        )
        .await?;

    drop_personas_users_fk(manager).await?;
    manager
        .alter_table(
            Table::alter()
                .table("personas")
                .rename_column(Alias::new("creator_id"), Alias::new("creator_uid"))
                .to_owned(),
        )
        .await?;
    manager
        .create_foreign_key(
            ForeignKey::create()
                .name("fk_personas_users")
                .from("personas", "creator_uid")
                .to("users", "uid")
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        )
        .await?;

    drop_tokens_users_fk(manager).await?;
    manager
        .alter_table(
            Table::alter()
                .table("tokens")
                .rename_column(Alias::new("user_id"), Alias::new("user_uid"))
                .to_owned(),
        )
        .await?;
    manager
        .create_foreign_key(
            ForeignKey::create()
                .name("fk_tokens_users")
                .from("tokens", "user_uid")
                .to("users", "uid")
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        )
        .await?;

    drop_files_foreign_keys(manager).await?;
    manager
        .alter_table(
            Table::alter()
                .table("files")
                .rename_column(Alias::new("uploaded_by"), Alias::new("uploaded_by_uid"))
                .to_owned(),
        )
        .await?;
    manager
        .alter_table(
            Table::alter()
                .table("files")
                .rename_column(Alias::new("scope_owner_id"), Alias::new("scope_owner_uid"))
                .to_owned(),
        )
        .await?;
    manager
        .alter_table(
            Table::alter()
                .table("files")
                .rename_column(
                    Alias::new("scope_entity_id"),
                    Alias::new("scope_entity_uid"),
                )
                .to_owned(),
        )
        .await?;
    create_files_foreign_keys_new(manager).await?;

    Ok(())
}

async fn rename_columns_down(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    drop_characters_users_fk(manager).await?;
    manager
        .alter_table(
            Table::alter()
                .table("characters")
                .rename_column(Alias::new("creator_uid"), Alias::new("user_id"))
                .to_owned(),
        )
        .await?;
    manager
        .create_foreign_key(
            ForeignKey::create()
                .name("fk_characters_users")
                .from("characters", "user_id")
                .to("users", "uid")
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        )
        .await?;

    drop_personas_users_fk(manager).await?;
    manager
        .alter_table(
            Table::alter()
                .table("personas")
                .rename_column(Alias::new("creator_uid"), Alias::new("creator_id"))
                .to_owned(),
        )
        .await?;
    manager
        .create_foreign_key(
            ForeignKey::create()
                .name("fk_personas_users")
                .from("personas", "creator_id")
                .to("users", "uid")
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        )
        .await?;

    drop_tokens_users_fk(manager).await?;
    manager
        .alter_table(
            Table::alter()
                .table("tokens")
                .rename_column(Alias::new("user_uid"), Alias::new("user_id"))
                .to_owned(),
        )
        .await?;
    manager
        .create_foreign_key(
            ForeignKey::create()
                .name("fk_tokens_users")
                .from("tokens", "user_id")
                .to("users", "uid")
                .on_delete(ForeignKeyAction::Cascade)
                .to_owned(),
        )
        .await?;

    drop_files_foreign_keys(manager).await?;
    manager
        .alter_table(
            Table::alter()
                .table("files")
                .rename_column(Alias::new("uploaded_by_uid"), Alias::new("uploaded_by"))
                .to_owned(),
        )
        .await?;
    manager
        .alter_table(
            Table::alter()
                .table("files")
                .rename_column(Alias::new("scope_owner_uid"), Alias::new("scope_owner_id"))
                .to_owned(),
        )
        .await?;
    manager
        .alter_table(
            Table::alter()
                .table("files")
                .rename_column(
                    Alias::new("scope_entity_uid"),
                    Alias::new("scope_entity_id"),
                )
                .to_owned(),
        )
        .await?;
    create_files_foreign_keys_old(manager).await?;

    Ok(())
}

async fn up_postgres_or_mysql(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    rename_columns_up(manager).await
}

async fn down_postgres_or_mysql(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    rename_columns_down(manager).await
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        if supports_inline_column_rename(manager.get_database_backend()) {
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
        if supports_inline_column_rename(manager.get_database_backend()) {
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
