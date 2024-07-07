use axum::{
    extract::{Path, State},
    routing::get,
    Form, Json, Router,
};

use crate::{
    api::model::AppError,
    model::{DeleteRecipeCommand, RecipeQuery},
    state::AppState,
};

use super::model as api_model;

pub fn build(state: AppState) -> Router {
    Router::new()
        .route(
            "/:id",
            get(fetch_recipe_handler)
                .put(update_recipe_handler)
                .delete(delete_recipe_handler),
        )
        .route("/", get(search_recipes_handler).post(create_recipe_handler))
        .with_state(state)
}

async fn search_recipes_handler(
    State(state): State<AppState>,
    Form(item): Form<api_model::RecipeSearchQuery>,
) -> Result<Json<api_model::SearchResult<api_model::Recipe>>, AppError> {
    let res = state
        .recipe_service
        .search(item.into())
        .await
        .map_err(|e| AppError(e))?;

    Ok(Json(res.into()))
}

async fn create_recipe_handler(
    State(state): State<AppState>,
    Json(item): Json<api_model::CreateRecipe>,
) -> Result<Json<api_model::Recipe>, AppError> {
    let res = state
        .recipe_service
        .create(item.into())
        .await
        .map_err(|e| AppError(e))?;

    Ok(Json(res.into()))
}

async fn fetch_recipe_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<api_model::Recipe>, AppError> {
    let res = state
        .recipe_service
        .fetch(RecipeQuery { id })
        .await
        .map_err(|e| AppError(e))?;

    Ok(Json(res.into()))
}

async fn update_recipe_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(mut item): Json<api_model::UpdateRecipe>,
) -> Result<Json<api_model::Recipe>, AppError> {
    item.id = id;

    let res = state
        .recipe_service
        .update(item.into())
        .await
        .map_err(|e| AppError(e))?;

    Ok(Json(res.into()))
}

async fn delete_recipe_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<api_model::Recipe>, AppError> {
    let res = state
        .recipe_service
        .delete(DeleteRecipeCommand { id })
        .await
        .map_err(|e| AppError(e))?;

    Ok(Json(res.into()))
}
