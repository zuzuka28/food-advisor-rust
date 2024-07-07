mod category;
mod model;
mod recipe;

use axum::{routing::get, Router};

use crate::state::AppState;

pub fn new_api(state: AppState) -> Router {
    Router::new().route("/ping", get(ping)).nest(
        "/recipes",
        recipe::build(state.clone()).nest("/categories", category::build(state.clone())),
    )
}

async fn ping() -> &'static str {
    "pong!"
}
