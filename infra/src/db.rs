pub mod joke_repository;
pub(self) mod map_err;
pub mod pool;

use sea_orm::{ConnectOptions, Database, DbConn};
use std::{
    io::{self, Error, ErrorKind},
    time::Duration,
};
use tokio::time::timeout;

use app::Assets;
use config::InfraConfig;
use migration::{init_migrator, Migrator, MigratorTrait};

pub use joke_repository::*;
pub use pool::*;

const PAGE_SIZE: u64 = 50;

pub fn init(config: &InfraConfig, assets: impl Assets) -> io::Result<DbConn> {
    let result = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            timeout(Duration::from_secs(60 * 3), async {
                let mut opt = ConnectOptions::new(config.db_url().to_owned());
                opt.sqlx_logging(config.db_logging_enabled())
                    .sqlx_logging_level(log::LevelFilter::Debug);

                let connection = Database::connect(opt)
                    .await
                    .map_err(|e| Error::new(ErrorKind::Other, e))?;

                if config.run_migrations_on_start() {
                    init_migrator(Box::new(assets))
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

                    Migrator::up(&connection, None)
                        .await
                        .map_err(|e| Error::new(ErrorKind::Other, e))?;
                }

                Ok(connection)
            })
            .await
            .map_err(|_| Error::new(ErrorKind::TimedOut, "Database connection attempt timed out"))
        });

    match result {
        Ok(Ok(db)) => Ok(db),
        Ok(Err(err)) => Err(err),
        Err(err) => Err(err),
    }
}
