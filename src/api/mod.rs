mod category;
mod model;
mod recipe;

use axum::{routing::get, Router};
use tower_http::cors::{self, CorsLayer};

use crate::state::AppState;

pub fn new_api(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods(cors::Any)
        .allow_origin(cors::Any)
        .allow_headers(cors::Any);

    Router::new()
        .route("/ping", get(ping))
        .nest(
            "/recipes",
            recipe::build(state.clone()).nest("/categories", category::build(state.clone())),
        )
        .layer(cors)
}

async fn ping() -> &'static str {
    "pong!"
}
