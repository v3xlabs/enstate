use opentelemetry_otlp::WithExportConfig;
use tracing::Level;
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

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .expect("to work");

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing::subscriber::set_global_default(subscriber.with(telemetry))
        .expect("setting default subscriber failed");
}
