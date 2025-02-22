use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use prometheus::{Counter, CounterVec, Encoder, Histogram, Registry, TextEncoder};

#[derive(Clone)]
pub struct Metrics {
    pub registry: Registry,

    pub name_lookup_total: Counter,
    pub name_lookup_latency: Histogram,

    pub rate_limit_infringements: CounterVec,
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
        )
        .buckets(vec![0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5]);
        let name_lookup_latency = Histogram::with_opts(name_lookup_latency_opts).unwrap();
        registry
            .register(Box::new(name_lookup_latency.clone()))
            .unwrap();

        let rate_limit_infringements_opts = prometheus::Opts::new(
            "rate_limit_infringements",
            "Total number of rate limit infringements",
        );

        let rate_limit_infringements = CounterVec::new(rate_limit_infringements_opts, &["ip"]).unwrap();
        registry
            .register(Box::new(rate_limit_infringements.clone()))
            .unwrap();

        // let rate_limit_infringements = Counter::with_opts(rate_limit_infringements_opts).unwrap();
        // registry
        //     .register(Box::new(rate_limit_infringements.clone()))
        //     .unwrap();

        Self {
            registry,
            name_lookup_total,
            name_lookup_latency,
            rate_limit_infringements,
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
