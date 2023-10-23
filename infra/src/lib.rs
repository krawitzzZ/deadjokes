pub mod db;
pub mod fs;
pub mod tracing;

use app::{Assets, DbPool, JokeRepository};
use db::{Pool, PostgresJokeRepository};
use fs::FsAssets;

pub trait InfrastructureContainer:
    shaku::HasComponent<dyn DbPool>
    + shaku::HasComponent<dyn Assets>
    + shaku::HasProvider<dyn JokeRepository>
{
}

shaku::module! {
    pub InfraContainer: InfrastructureContainer {
        components = [Pool, FsAssets],
        providers = [PostgresJokeRepository]
    }
}
