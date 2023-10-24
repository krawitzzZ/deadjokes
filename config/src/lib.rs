pub mod api_config;
pub mod infra_config;

use std::{env, io, str::FromStr};

pub use api_config::ApiConfig;
pub use infra_config::InfraConfig;

pub(self) trait Config {
    fn get_flag_from_env(key: &str) -> bool {
        env::var(key).ok().is_some()
    }

    fn get_value_from_env<T: FromStr>(key: &str, default: Option<T>) -> io::Result<T> {
        match env::var(key) {
            Err(e) => default.ok_or(io::Error::new(
                io::ErrorKind::Other,
                format!("failed to get value for `{key}` from env: {e}"),
            )),
            Ok(v) => v.parse::<T>().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!("failed to parse environment variable `{key}`"),
                )
            }),
        }
    }
}
