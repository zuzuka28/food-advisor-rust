use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

use crate::{
    model::{CategoryQuery, CategorySearchQuery, DeleteCategoryCommand, UpdateCategoryCommand},
    state::AppState,
};

use super::model::{self as api_model, AppError};

pub fn build(state: AppState) -> Router {
    Router::new()
        .route(
            "/:id",
            get(fetch_category_handler)
                .put(update_category_handler)
                .delete(delete_category_handler),
        )
        .route(
            "/",
            get(search_category_handler).post(create_category_handler),
        )
        .with_state(state)
}

async fn search_category_handler(
    State(state): State<AppState>,
) -> Result<Json<api_model::SearchResult<api_model::Category>>, AppError> {
    let res = state
        .category_service
        .search(CategorySearchQuery { ids: None })
        .await
        .map_err(|e| AppError(e))?;

    Ok(Json(res.into()))
}

async fn create_category_handler(
    State(state): State<AppState>,
    Json(item): Json<api_model::CreateCategory>,
) -> Result<Json<api_model::Category>, AppError> {
    let res = state
        .category_service
        .create(item.into())
        .await
        .map_err(|e| AppError(e))?;

    Ok(Json(res.into()))
}

async fn fetch_category_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<api_model::Category>, AppError> {
    let res = state
        .category_service
        .fetch(CategoryQuery { id })
        .await
        .map_err(|e| AppError(e))?;

    Ok(Json(res.into()))
}

async fn update_category_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(item): Json<api_model::UpdateCategory>,
) -> Result<Json<api_model::Category>, AppError> {
    let mut cmd: UpdateCategoryCommand = item.into();
    cmd.id = id;

    let res = state
        .category_service
        .update(cmd)
        .await
        .map_err(|e| AppError(e))?;

    Ok(Json(res.into()))
}

async fn delete_category_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<api_model::Category>, AppError> {
    let res = state
        .category_service
        .delete(DeleteCategoryCommand { id })
        .await
        .map_err(|e| AppError(e))?;

    Ok(Json(res.into()))
}
