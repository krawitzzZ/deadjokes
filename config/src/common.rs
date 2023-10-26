use log::LevelFilter;
use std::io;

use crate::get_value_from_env;

#[derive(Debug, Clone)]
pub struct Config {
    log_level: LevelFilter,
    logstash_port: u16,
    logstash_host: String,
    jaeger_root_url: String,
}

impl Config {
    pub fn new() -> io::Result<Self> {
        let log_level = get_value_from_env("RUST_LOG").unwrap_or(LevelFilter::Info);
        let logstash_port = get_value_from_env("LOGSTASH_PORT")?;
        let logstash_host =
            get_value_from_env("LOGSTASH_HOST").unwrap_or(String::from("127.0.0.1"));
        let jaeger_root_url =
            get_value_from_env("JAEGER_ROOT_URL").unwrap_or(String::from("http://127.0.0.1:14268"));

        Ok(Self {
            log_level,
            logstash_host,
            logstash_port,
            jaeger_root_url,
        })
    }

    pub fn log_level(&self) -> LevelFilter {
        self.log_level
    }

    pub fn logstash_port(&self) -> u16 {
        self.logstash_port
    }

    pub fn logstash_host(&self) -> &str {
        &self.logstash_host
    }

    pub fn jaeger_root_url(&self) -> &str {
        &self.jaeger_root_url
    }
}
