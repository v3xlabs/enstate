use axum::extract::{MatchedPath, State};
use axum::http::{Request, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{Html, Redirect, Response};
use std::env;
use std::{net::SocketAddr, sync::Arc};

use axum::{routing::get, Router};
use dashmap::DashMap;
use std::time::{Duration, Instant};
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{info, info_span};

use crate::routes;
use crate::state::AppState;
use crate::telemetry::metrics::{self};

// Add this struct to hold rate limit configuration
#[derive(Clone)]
struct RateLimit {
    requests: u32,
    window: Duration,
}

// Add this struct to track rate limiting state
struct RateLimitState {
    last_reset: Instant,
    count: u32,
}

// Add this to your AppState
pub struct RateLimiter {
    limits: DashMap<String, RateLimit>,
    states: DashMap<(String, String), RateLimitState>, // (path, ip) -> state
}

impl RateLimiter {
    pub fn new() -> Self {
        let limits = DashMap::new();

        if env::var("RATE_LIMIT_ENABLED").unwrap_or_else(|_| "false".to_owned()) == "true" {
            limits.insert(
                "/n/:name".to_string(),
                RateLimit {
                    requests: 160,
                    window: Duration::from_secs(60),
                },
            );
            limits.insert(
                "/a/:address".to_string(),
                RateLimit {
                    requests: 160,
                    window: Duration::from_secs(60),
                },
            );
            limits.insert(
                "/bulk/a".to_string(),
                RateLimit {
                    requests: 9,
                    window: Duration::from_secs(60),
                },
            );
            limits.insert(
                "/bulk/n".to_string(),
                RateLimit {
                    requests: 9,
                    window: Duration::from_secs(60),
                },
            );
        }

        Self {
            limits,
            states: DashMap::new(),
        }
    }
}

// Add rate limiting middleware
async fn rate_limit_middleware(
    State(state): State<Arc<AppState>>,
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let ip = req
        .headers()
        .get("CF-Connecting-IP")
        .and_then(|hv| hv.to_str().ok())
        .unwrap_or_else(|| {
            req.headers()
                .get("x-real-ip")
                .and_then(|hv| hv.to_str().ok())
                .unwrap_or_else(|| {
                    req.headers()
                        .get("x-forwarded-for")
                        .and_then(|hv| hv.to_str().ok())
                        .unwrap_or("unknown")
                })
        })
        .to_string();

    let path = req
        .extensions()
        .get::<MatchedPath>()
        .map(MatchedPath::as_str)
        .unwrap_or("/")
        .to_string();

    let rate_limiter = &state.rate_limiter;

    if let Some(limit) = rate_limiter.limits.get(&path) {
        let key = (path.clone(), ip.clone());
        let now = Instant::now();

        let mut exceeded = false;

        if env::var("RATE_LIMIT_ENABLED").unwrap_or_else(|_| "false".to_owned()) == "true" {
            info!("Rate limit for {} is {}", path, ip);
        }

        rate_limiter
            .states
            .entry(key)
            .and_modify(|state| {
                if now.duration_since(state.last_reset) >= limit.window {
                    state.count = 1;
                    state.last_reset = now;
                    info!("Rate limit reset for {}", path);
                } else if state.count >= limit.requests {
                    info!("Rate limit exceeded for {}", path);
                    exceeded = true;
                } else {
                    state.count += 1;
                }
            })
            .or_insert(RateLimitState {
                last_reset: now,
                count: 1,
            });

        if exceeded {
            state
                .metrics
                .rate_limit_infringements
                .with_label_values(&[&ip])
                .inc();

            return Err(StatusCode::TOO_MANY_REQUESTS);
        }
    }

    Ok(next.run(req).await)
}

pub struct App {
    router: Router,
}

impl App {
    pub async fn listen(
        self,
        port: u16,
        shutdown_signal: CancellationToken,
    ) -> Result<(), anyhow::Error> {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));

        let listener = TcpListener::bind(&addr).await?;

        async fn await_shutdown(shutdown_signal: CancellationToken) {
            shutdown_signal.cancelled().await;
        }

        let server = axum::serve(listener, self.router.into_make_service())
            .with_graceful_shutdown(await_shutdown(shutdown_signal));

        info!("Listening HTTP on {}", addr);

        server.await?;

        info!("HTTP server shutdown");

        Ok(())
    }
}

pub fn setup(mut state: AppState) -> App {
    let docs = Router::new()
        .route("/openapi.json", get(crate::docs::openapi))
        .route("/", get(scalar_handler))
        .route("/favicon.png", get(scalar_favicon_handler))
        .route("/opengraph.png", get(scalar_opengraph_handler));

    let state = Arc::new(state);

    let router = Router::new()
        .route("/", get(|| async { Redirect::temporary("/docs") }))
        .nest("/docs", docs)
        .route("/this", get(routes::root::get))
        .route("/a/:address", get(routes::address::get))
        .route("/n/:name", get(routes::name::get))
        .route("/u/:name_or_address", get(routes::universal::get))
        .route("/i/:name_or_address", get(routes::image::get))
        .route("/h/:name_or_address", get(routes::header::get))
        .route("/bulk/a", get(routes::address::get_bulk))
        .route("/bulk/n", get(routes::name::get_bulk))
        .route("/bulk/u", get(routes::universal::get_bulk))
        .route(
            "/sse/a",
            get(routes::address::get_bulk_sse).post(routes::address::post_bulk_sse),
        )
        .route(
            "/sse/n",
            get(routes::name::get_bulk_sse).post(routes::name::post_bulk_sse),
        )
        .route(
            "/sse/u",
            get(routes::universal::get_bulk_sse).post(routes::universal::post_bulk_sse),
        )
        .route("/metrics", get(metrics::handle))
        .fallback(routes::four_oh_four::handler)
        .layer(middleware::from_fn_with_state(
            state.clone(),
            rate_limit_middleware,
        ))
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        )
        .with_state(state);

    App { router }
}

// Loads from docs/index.html with headers html
async fn scalar_handler() -> Html<&'static str> {
    let contents = include_str!("./docs/html/index.html");
    axum::response::Html(contents)
}

async fn scalar_favicon_handler() -> impl axum::response::IntoResponse {
    let contents = include_bytes!("./docs/html/favicon.png");
    contents
}

async fn scalar_opengraph_handler() -> impl axum::response::IntoResponse {
    let contents = include_bytes!("./docs/html/opengraph.png");
    contents
}
