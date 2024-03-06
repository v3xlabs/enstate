use std::env;

use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use tracing::{Level, warn};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use tracing_subscriber::layer::SubscriberExt;

pub fn setup() {
    let filter = EnvFilter::new(format!(
        "enstate={},ethers_ccip_read={}",
        Level::DEBUG,
        Level::DEBUG
    ));

    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        .with_env_filter(filter)
        // completes the builder.
        .finish();

    let otlp_url = env::var("OTLP_ENDPOINT");

    if let Ok(otlp_url) = otlp_url {
        let service_name = env::var("OTLP_LABEL").unwrap_or_else(|_| "enstate".to_string());

        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter()
                    .tonic()
                    .with_endpoint(otlp_url),
            )
            .with_trace_config(
                opentelemetry_sdk::trace::config().with_resource(Resource::new(vec![
                    KeyValue::new("service.name", service_name),
                ])),
            )
            .install_batch(opentelemetry_sdk::runtime::Tokio)
            .expect("to work");

        let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

        // FIX:
        tracing::subscriber::set_global_default(subscriber.with(telemetry))
            .expect("setting default subscriber failed");
    } else {
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
        warn!("not using opentelemetry tracing");
    }
}
