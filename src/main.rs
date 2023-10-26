mod container;

use std::sync::Arc;

use app::AppState;
use config;
use config::{
    api::Config as ApiConfig, common::Config as CommonConfig, infra::Config as InfraConfig,
};
use container::RootContainer;
use infra::{
    db::{Pool, PoolParameters},
    fs::{FsAssets, FsAssetsParameters},
    InfraContainer,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let app_name = env!("CARGO_BIN_NAME");
    let common_config = CommonConfig::new()?;
    let infra_config = InfraConfig::new()?;
    let api_config = ApiConfig::new(app_name)?;

    // infra::logging::init(&common_config);
    infra::spawn_signal_handler();

    let assets_params = FsAssetsParameters::new(app_name)?;
    let assets = FsAssets::new(&assets_params);
    let db_conn = infra::db::init(&infra_config, assets)?;
    let db_container = InfraContainer::builder()
        .with_component_parameters::<Pool>(PoolParameters { db_conn })
        .with_component_parameters::<FsAssets>(assets_params)
        .build();
    let root_container = RootContainer::builder(Arc::new(db_container)).build();
    let state = AppState::new(
        app_name,
        std::env::var("API_KEY").expect("API secret key to be set"),
        Arc::new(root_container),
    );

    api::main(&common_config, &api_config, state);

    Ok(())
}
