use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use prometheus::{Counter, Encoder, Registry, TextEncoder};

use crate::state::AppState;

pub struct Metrics {
    pub registry: Registry,

    pub requests_total: Counter,
}

impl Metrics {
    pub fn new() -> Self {
        let registry = Registry::new();

        let requests_total_opts =
            prometheus::Opts::new("requests_total", "Total number of HTTP requests");
        let requests_total = Counter::with_opts(requests_total_opts).unwrap();
        registry.register(Box::new(requests_total.clone())).unwrap();

        Self {
            registry,
            requests_total,
        }
    }
}

pub async fn handle(State(state): State<Arc<crate::AppState>>,) -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = state.metrics.registry.gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
