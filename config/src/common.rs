use std::io;

use crate::{get_flag_from_env, get_value_from_env};

#[derive(Debug, Clone)]
pub struct Config {
    logstash_use_tls: bool,
    logstash_port: u16,
    logstash_host: String,
    jaeger_root_url: String,
}

impl Config {
    pub fn new() -> io::Result<Self> {
        let logstash_use_tls = get_flag_from_env("LOGSTASH_USE_TLS");
        let logstash_port = get_value_from_env("LOGSTASH_PORT")?;
        let logstash_host =
            get_value_from_env("LOGSTASH_HOST").unwrap_or(String::from("127.0.0.1"));
        let jaeger_root_url =
            get_value_from_env("JAEGER_ROOT_URL").unwrap_or(String::from("http://127.0.0.1:14268"));

        Ok(Self {
            logstash_use_tls,
            logstash_host,
            logstash_port,
            jaeger_root_url,
        })
    }

    pub fn logstash_use_tls(&self) -> bool {
        self.logstash_use_tls
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
