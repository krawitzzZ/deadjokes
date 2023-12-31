mod logstash;

use opentelemetry::{global, runtime::TokioCurrentThread};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

use config::common::Config;

pub fn init(name: &str, common_config: &Config) {
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

    let make_logstash_writer = logstash::get_logstash_writer_factory(common_config);
    let tracer = opentelemetry_jaeger::new_collector_pipeline()
        .with_service_name(name)
        .with_endpoint(format!("{}/api/traces", common_config.jaeger_root_url()))
        .with_reqwest()
        .with_timeout(std::time::Duration::from_secs(2))
        .install_batch(TokioCurrentThread)
        .expect("failed to initiate jaeger collector pipeline");

    let filter_layer = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    let stdout_layer = BunyanFormattingLayer::new(name.into(), std::io::stdout);
    let logstash_layer = BunyanFormattingLayer::new(name.into(), make_logstash_writer);
    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    let subscriber = Registry::default()
        .with(filter_layer)
        .with(JsonStorageLayer)
        .with(stdout_layer)
        .with(logstash_layer)
        .with(telemetry_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set jaeger tracing subscriber");
}
