use std::io;

use crate::Config;

#[derive(Debug, Clone)]
pub struct InfraConfig {
    db_url: String,
    db_logging_enabled: bool,
    run_migrations_on_start: bool,
}

impl Config for InfraConfig {}

impl InfraConfig {
    pub fn new() -> io::Result<Self> {
        let db_url = Self::get_value_from_env("DATABASE_URL", None)?;
        let db_logging_enabled = Self::get_flag_from_env("DB_LOGGING_ENABLED");
        let run_migrations_on_start = Self::get_flag_from_env("RUN_MIGRATIONS_ON_START");

        Ok(Self {
            db_url,
            db_logging_enabled,
            run_migrations_on_start,
        })
    }

    pub fn db_url(&self) -> &str {
        &self.db_url
    }

    pub fn db_logging_enabled(&self) -> bool {
        self.db_logging_enabled
    }

    pub fn run_migrations_on_start(&self) -> bool {
        self.run_migrations_on_start
    }
}
