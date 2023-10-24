use std::{
    io,
    path::{Path, PathBuf},
};

use crate::Config;

#[derive(Debug, Clone)]
pub struct ApiConfig {
    port: u16,
    static_dir: PathBuf,
    jaeger_host: String,
}

impl Config for ApiConfig {}

impl ApiConfig {
    pub fn new(app_name: &str) -> io::Result<Self> {
        let data_dir = dirs::data_dir().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "failed to get user data dir")
        })?;
        let port = Self::get_value_from_env("API_PORT", Some(4343u16))?;
        let static_dir = data_dir.join(app_name).join("static");
        let jaeger_host = Self::get_value_from_env("JAEGER_HOST", Some(String::from("127.0.0.1")))?;

        std::fs::create_dir_all(&static_dir)?;

        Ok(Self {
            port,
            static_dir,
            jaeger_host,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn static_dir(&self) -> &Path {
        self.static_dir.as_path()
    }

    pub fn jaeger_host(&self) -> &str {
        &self.jaeger_host
    }
}
