use std::{
    io,
    path::{Path, PathBuf},
};

use crate::Config;

#[derive(Debug, Clone)]
pub struct ApiConfig {
    port: u16,
    static_dir: PathBuf,
}

impl Config for ApiConfig {}

impl ApiConfig {
    pub fn new() -> io::Result<Self> {
        let port = Self::get_value_from_env("API_PORT", Some(3500u16))?;
        let static_dir = Self::get_directory_from_env("STATIC_FOLDER")?;

        Ok(Self { port, static_dir })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn static_dir(&self) -> &Path {
        self.static_dir.as_path()
    }
}
