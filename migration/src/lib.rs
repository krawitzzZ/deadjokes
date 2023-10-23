mod m20230501_000001_initial_migration;
mod m20230502_000001_initial_seed;

use std::sync::Mutex;

use app::Assets;

pub use sea_orm_migration::prelude::*;

static ASSETS: Mutex<Option<&'static dyn Assets>> = Mutex::new(None);

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let config = get_infra_config();
        vec![
            Box::new(m20230501_000001_initial_migration::Migration),
            Box::new(m20230502_000001_initial_seed::Migration::new(config)),
        ]
    }
}

/// [`Assets`] is required in order to seed the database
pub fn init_migrator(assets: Box<impl Assets>) -> Result<(), String> {
    let mut assets_opt = ASSETS
        .lock()
        .map_err(|e| format!("failed to initialize migrator: {e}"))?;

    match *assets_opt {
        Some(_) => return Err(String::from("migrator has already been initialized")),
        None => *assets_opt = Some(Box::leak(assets)),
    }

    Ok(())
}

fn get_infra_config() -> &'static dyn Assets {
    let assets_opt = ASSETS.lock().expect("failed to initialize migrator");

    match *assets_opt {
        Some(assets) => assets,
        None => panic!("migrations are not initialized with assets"),
    }
}
