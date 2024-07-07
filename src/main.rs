mod api;
mod model;
mod repository;
mod service;
mod state;

use dotenvy::dotenv;
use std::{env, sync::Arc};

use crate::api::new_api;
use state::AppState;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_conn = Arc::new(repository::connect(database_url).await);

    let category_storage = Arc::new(repository::CategoryRepository::new(db_conn.clone()).await);
    let recipe_storage = Arc::new(repository::RecipeRepository::new(db_conn.clone()).await);

    let category_service = Arc::new(service::CategoryService::new(
        service::category::Config { category_storage },
    ));

    let recipe_service = Arc::new(service::RecipeService::new(
        service::recipe::Config {
            category_service: category_service.clone(),
            recipe_storage,
        },
    ));

    let app_state = AppState {
        recipe_service,
        category_service,
    };

    let myapi = new_api(app_state);

    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, myapi).await.unwrap();
}
