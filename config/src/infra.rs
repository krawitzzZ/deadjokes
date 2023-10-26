use std::io;

use crate::{get_flag_from_env, get_value_from_env};

#[derive(Debug, Clone)]
pub struct Config {
    db_url: String,
    db_logging_enabled: bool,
    run_migrations_on_start: bool,
    skip_initial_db_seed: bool,
}

impl Config {
    pub fn new() -> io::Result<Self> {
        let db_url = get_value_from_env("DATABASE_URL")?;
        let db_logging_enabled = get_flag_from_env("DB_LOGGING_ENABLED");
        let run_migrations_on_start = get_flag_from_env("RUN_MIGRATIONS_ON_START");
        let skip_initial_db_seed = get_flag_from_env("DB_SKIP_INITIAL_SEED");

        Ok(Self {
            db_url,
            db_logging_enabled,
            run_migrations_on_start,
            skip_initial_db_seed,
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

    pub fn skip_initial_db_seed(&self) -> bool {
        self.skip_initial_db_seed
    }
}
