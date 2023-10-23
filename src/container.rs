use shaku::module;

use app::{joke::JokeService, AppContainer, Assets, DbPool, JokeRepository};
use infra::InfrastructureContainer;

module! {
    pub RootContainer: AppContainer {
        components = [],
        providers = [JokeService],

        use dyn InfrastructureContainer {
            components = [dyn DbPool, dyn Assets],
            providers = [dyn JokeRepository]
        }
    }
}
