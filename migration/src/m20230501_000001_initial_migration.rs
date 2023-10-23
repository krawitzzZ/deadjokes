use sea_orm::{EntityName, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        create_uuid_extension(manager).await?;
        create_jokes_table(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        drop_jokes_table(manager).await?;
        drop_uuid_extension(manager).await?;
        Ok(())
    }
}

async fn create_uuid_extension(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let stmt = Statement::from_string(
        manager.get_database_backend(),
        r#"CREATE EXTENSION IF NOT EXISTS "uuid-ossp";"#.to_string(),
    );
    manager.get_connection().execute(stmt).await?;
    Ok(())
}

async fn drop_uuid_extension(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let stmt = Statement::from_string(
        manager.get_database_backend(),
        r#"DROP EXTENSION IF EXISTS "uuid-ossp";"#.to_string(),
    );
    manager.get_connection().execute(stmt).await?;
    Ok(())
}

async fn create_jokes_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_table(
            Table::create()
                .if_not_exists()
                .table(entity::JokeEntity.table_ref())
                .col(
                    ColumnDef::new(entity::JokeColumn::Id)
                        .uuid()
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(entity::JokeColumn::Body).text().not_null())
                .col(
                    ColumnDef::new(entity::joke::Column::CreatedAt)
                        .timestamp_with_time_zone()
                        .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                        .not_null(),
                )
                .col(
                    ColumnDef::new(entity::joke::Column::LastUpdatedAt)
                        .timestamp_with_time_zone()
                        .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp))
                        .not_null(),
                )
                .to_owned(),
        )
        .await?;

    let idx = Index::create()
        .if_not_exists()
        .name("idx_joke_body_full_text_search")
        .table(entity::JokeEntity.table_ref())
        .to_owned()
        .build(PostgresQueryBuilder)
        .replace("()", "");
    let idx_sql = format!(
        r#"{} USING GIN (to_tsvector('english', "{}"));"#,
        idx.trim(),
        entity::JokeColumn::Body.to_string()
    );
    let stmt = Statement::from_string(manager.get_database_backend(), idx_sql);

    manager
        .get_connection()
        .execute_unprepared(&stmt.to_string())
        .await?;

    Ok(())
}

async fn drop_jokes_table(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .drop_index(
            Index::drop()
                .name("idx_joke_body_full_text_search")
                .table(entity::JokeEntity.table_ref())
                .to_owned(),
        )
        .await?;

    manager
        .drop_table(
            Table::drop()
                .table(entity::JokeEntity.table_ref())
                .if_exists()
                .to_owned(),
        )
        .await?;
    Ok(())
}
