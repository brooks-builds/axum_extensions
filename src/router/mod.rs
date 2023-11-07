mod healthcheck;

use axum::{routing::get, Extension, Router};
use healthcheck::healthcheck;

pub fn create_router(address: [u8; 4], port: u16) -> Router {
    Router::new()
        .route("/healthcheck", get(healthcheck))
        .route_layer(Extension(address))
        .layer(Extension(port))
}
