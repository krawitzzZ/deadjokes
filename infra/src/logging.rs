use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    init_config, Config,
};
use std::time::Duration;

use config::common;

pub fn init(config: &common::Config) {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{date(%Y-%m-%d %H:%M:%S%.6f %Z)(utc)} {highlight([{level:5.5}])} <{module}> - {message}{n}",
        )))
        .build();
    let logstash = qoollo_log4rs_logstash::appender::AppenderBuilder::default()
        .with_hostname(config.logstash_host())
        .with_port(config.logstash_port())
        .with_buffer_size(100)
        .with_buffer_lifetime(Duration::from_secs(2))
        .build()
        .expect("failed to initialize logstash appender");

    let stdout_appender = Appender::builder().build("stdout", Box::new(stdout));
    let stash_appender = Appender::builder().build("logstash", Box::new(logstash));
    let config = Config::builder()
        .appender(stdout_appender)
        .appender(stash_appender)
        .build(
            Root::builder()
                .appender("logstash")
                .appender("stdout")
                .build(config.log_level()),
        )
        .expect("failed to build logging config");

    init_config(config).expect("failed to initialize logging");

    log::trace!("Trace log level enabled");
    log::debug!("Debug log level enabled");
    log::info!("Info log level enabled");
    log::warn!("Warn log level enabled");
    log::error!("Error log level enabled");
}
