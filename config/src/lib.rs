pub mod api_config;
pub mod infra_config;

use std::{env, io, path::PathBuf, str::FromStr};

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

    fn get_directory_from_env(key: &str) -> io::Result<PathBuf> {
        let path_str = env::var(key).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        let path = PathBuf::from(path_str);
        let dir = if path.is_absolute() {
            path
        } else {
            env::current_dir()?.join(path)
        };

        if !dir.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("{} directory does not exist", key),
            ));
        }

        dir.canonicalize()
    }
}
