mod config;
mod routes;

use std::sync::Arc;

use axum::{Router, routing::get};
pub use config::MiasmaConfig;
use tokio::sync::Semaphore;
use tower_http::{CompressionLevel, compression::CompressionLayer};

/// Build a new `axum::Router` for miasma's routes.
pub fn new_miasma_router(config: &'static MiasmaConfig) -> Router {
    let in_flight_sem = Arc::new(Semaphore::new(config.max_in_flight as usize));

    let compression = CompressionLayer::new()
        .br(true)
        .gzip(true)
        .no_deflate()
        .no_zstd()
        .quality(CompressionLevel::Best);

    Router::new()
        .fallback(get(move || routes::serve_poison(config, in_flight_sem)))
        .layer(compression)
}
