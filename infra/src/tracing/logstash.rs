use std::io::{self, Write};

use config::common;

use crate::tcp::AdvancedTcpStream;

pub(crate) fn get_logstash_writer_factory(
    common_config: &common::Config,
) -> impl Fn() -> LogstashWriter {
    let logstash_host = common_config.logstash_host().to_owned();
    let logstash_port = common_config.logstash_port();
    let use_tls = common_config.logstash_use_tls();

    move || LogstashWriter::new(logstash_host.clone(), logstash_port, use_tls)
}

pub(crate) struct LogstashWriter {
    stream: AdvancedTcpStream,
}

impl LogstashWriter {
    fn new(hostname: String, port: u16, use_tls: bool) -> Self {
        LogstashWriter {
            stream: AdvancedTcpStream::new(hostname, port, use_tls),
        }
    }
}

impl Write for LogstashWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.send_bytes(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()?;
        Ok(())
    }
}
