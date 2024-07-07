use std::sync::Arc;

use crate::service;

#[derive(Clone)]
pub struct AppState {
    pub recipe_service: Arc<service::RecipeService>,
    pub category_service: Arc<service::CategoryService>,
}
