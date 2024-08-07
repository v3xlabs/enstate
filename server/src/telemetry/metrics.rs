use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use prometheus::{Counter, Encoder, Histogram, Registry, TextEncoder};

#[derive(Clone)]
pub struct Metrics {
    pub registry: Registry,

    pub name_lookup_total: Counter,
    pub name_lookup_latency: Histogram,
}

impl Metrics {
    pub fn new() -> Self {
        let registry = Registry::new();

        let name_lookup_total_opts = prometheus::Opts::new(
            "name_lookup_total",
            "Total number of HTTP requests to the name lookup endpoint",
        );
        let name_lookup_total = Counter::with_opts(name_lookup_total_opts).unwrap();
        registry
            .register(Box::new(name_lookup_total.clone()))
            .unwrap();

        let name_lookup_latency_opts = prometheus::HistogramOpts::new(
            "name_lookup_latency",
            "Latency of the name lookup endpoint",
        );
        let name_lookup_latency = Histogram::with_opts(name_lookup_latency_opts).unwrap();
        registry
            .register(Box::new(name_lookup_latency.clone()))
            .unwrap();

        Self {
            registry,
            name_lookup_total,
            name_lookup_latency,
        }
    }
}

pub async fn handle(State(state): State<Arc<crate::AppState>>) -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = state.metrics.registry.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
