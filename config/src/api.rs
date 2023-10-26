use std::{
    io,
    path::{Path, PathBuf},
};

use crate::get_value_from_env;

#[derive(Debug, Clone)]
pub struct Config {
    port: u16,
    static_dir: PathBuf,
}

impl Config {
    pub fn new(app_name: &str) -> io::Result<Self> {
        let port = get_value_from_env("API_PORT")?;
        let static_dir = dirs::data_dir()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "failed to get user data dir"))?
            .join(app_name)
            .join("static");

        std::fs::create_dir_all(&static_dir)?;

        Ok(Self { port, static_dir })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn static_dir(&self) -> &Path {
        self.static_dir.as_path()
    }
}
