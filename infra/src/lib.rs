pub mod db;
pub mod fs;
mod tcp;
pub mod tracing;

use signal_hook::{consts, iterator::Signals, low_level::exit};

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

pub fn spawn_signal_handler() {
    let mut signals = Signals::new(&[consts::SIGINT, consts::SIGTERM])
        .expect("signal handler failed to initialize");

    std::thread::spawn(move || {
        let mut stop_in_progress = false;
        for _sig in signals.forever() {
            std::thread::spawn(move || {
                opentelemetry::global::shutdown_tracer_provider();
                exit(0)
            });
            if stop_in_progress {
                exit(1);
            }
            stop_in_progress = true;
        }
    });
}
