use sea_orm::Statement;
use sea_orm_migration::prelude::*;

use app::Assets;

#[derive(DeriveMigrationName)]
pub struct Migration {
    assets: &'static dyn Assets,
}

impl Migration {
    pub fn new(assets: &'static dyn Assets) -> Self {
        Self { assets }
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = self
            .assets
            .initial_jokes_seed()
            .await
            .map_err(|e| DbErr::Migration(format!("failed to seed jokes: {e}")))?;
        seed_jokes(sql, manager).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        truncate_jokes(manager).await?;
        Ok(())
    }
}

async fn seed_jokes(sql: String, manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let db = manager.get_connection();
    let stmt = Statement::from_string(manager.get_database_backend(), sql);
    db.execute_unprepared(&stmt.to_string()).await?;
    Ok(())
}

async fn truncate_jokes(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let db = manager.get_connection();
    let stmt = Statement::from_string(
        manager.get_database_backend(),
        r#"TRUNCATE "jokes" CASCADE"#.to_string(),
    );
    db.execute(stmt).await?;
    Ok(())
}
