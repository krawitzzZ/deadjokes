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
        let var_result = env::var(key)
            .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?
            .parse::<T>();

        match var_result {
            Ok(var) => Ok(var),
            Err(_) => default.map_or(
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("failed to parse environment variable {key}"),
                )),
                Ok,
            ),
        }
    }
}
