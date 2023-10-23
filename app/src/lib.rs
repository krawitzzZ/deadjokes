pub mod error;
pub mod joke;
pub mod query;
pub mod traits;

use std::sync::Arc;

use joke::*;

pub use error::*;
pub use query::*;
pub use traits::*;

pub trait AppContainer: shaku::HasProvider<JokeService> {}

pub struct AppState {
    app_name: String,
    app_key: String,
    container: Arc<dyn AppContainer>,
}

impl AppState {
    pub fn new(app_name: &str, app_key: String, container: Arc<dyn AppContainer>) -> Self {
        AppState {
            app_name: String::from(app_name),
            app_key,
            container,
        }
    }

    pub fn app_name(&self) -> &str {
        &self.app_name
    }

    pub fn app_key(&self) -> &str {
        &self.app_key
    }

    pub fn container(&self) -> Arc<dyn AppContainer> {
        Arc::clone(&self.container)
    }
}
