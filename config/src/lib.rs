pub mod api;
pub mod common;
pub mod infra;

use std::{env, io, str::FromStr};

pub(crate) fn get_flag_from_env(key: &str) -> bool {
    env::var(key).ok().is_some()
}

pub(crate) fn get_value_from_env<T: FromStr>(key: &str) -> io::Result<T> {
    match env::var(key) {
        Err(e) => Err(io::Error::new(
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
