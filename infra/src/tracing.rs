use config::api_config;
use opentelemetry::{
    global, runtime::TokioCurrentThread, sdk::propagation::TraceContextPropagator,
};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub fn init(name: &str, infra_config: &api_config::ApiConfig) {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_endpoint(format!("{}:6831", infra_config.jaeger_host()))
        .with_service_name(name)
        .install_batch(TokioCurrentThread)
        .expect("Failed to install OpenTelemetry tracer");
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    let formatting_layer = BunyanFormattingLayer::new(name.into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(filter_layer)
        .with(telemetry_layer)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to install `tracing` subscriber");
}
