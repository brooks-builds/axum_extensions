mod healthcheck;

use axum::{middleware, routing::get, Extension, Router};
use healthcheck::healthcheck;

pub fn create_router(address: [u8; 4], port: u16) -> Router {
    Router::new()
        .route("/healthcheck", get(healthcheck).layer(Extension(port)))
        .merge(
            Router::new()
                .route("/tasks", get(get_all_tasks))
                .route("/tasks", post(create_task)),
        )
        .route_layer(middleware::from_fn(firewall))
        .route_layer(Extension(address))
        .route("/tasks/:id", get(get_task_by_id))
        .route_layer(middleware::from_fn(authorize))
        .route("/tasks/:id", put(update_task_by_id))
        .route("/tasks/:id", delete(delete_task_by_id))
        .layer(Extension(port))
}
