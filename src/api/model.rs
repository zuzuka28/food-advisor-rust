use std::error::Error;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde::Deserialize;
use serde::Serialize;

use crate::model;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: String,
    pub name: String,
}

impl From<model::Category> for Category {
    fn from(value: model::Category) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCategory {
    pub name: String,
}

impl Into<model::CreateCategoryCommand> for CreateCategory {
    fn into(self) -> model::CreateCategoryCommand {
        model::CreateCategoryCommand { name: self.name }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCategory {
    pub name: String,
}

impl Into<model::UpdateCategoryCommand> for UpdateCategory {
    fn into(self) -> model::UpdateCategoryCommand {
        model::UpdateCategoryCommand {
            id: String::default(),
            name: self.name,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipe {
    pub id: String,
    // pub cover: Vec<u8>,
    pub cover: String,
    pub title: String,
    pub description: String,
    pub time_to_cook: i64,
    pub difficulty: String,
    pub servings: i64,
    pub category: Category,
    pub ingredients: Vec<String>,
    pub nutrients: Nutrients,
    pub guideline: String,
}

impl From<model::Recipe> for Recipe {
    fn from(value: model::Recipe) -> Self {
        Self {
            id: value.id,
            cover: String::from_utf8(value.cover).unwrap(),
            title: value.title,
            description: value.description,
            time_to_cook: value.time_to_cook,
            difficulty: value.difficulty,
            servings: value.servings,
            category: value.category.into(),
            ingredients: value.ingredients,
            nutrients: value.nutrients.into(),
            guideline: value.guideline,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nutrients {
    pub proteins: i64,
    pub fats: f64,
    pub carbohydrates: f64,
    pub fiber: i64,
    pub kcal: i64,
}

impl Into<model::Nutrients> for Nutrients {
    fn into(self) -> model::Nutrients {
        model::Nutrients {
            proteins: self.proteins,
            fats: self.fats,
            carbohydrates: self.carbohydrates,
            fiber: self.fiber,
            kcal: self.kcal,
        }
    }
}

impl From<model::Nutrients> for Nutrients {
    fn from(value: model::Nutrients) -> Self {
        Self {
            proteins: value.proteins,
            fats: value.fats,
            carbohydrates: value.carbohydrates,
            fiber: value.fiber,
            kcal: value.kcal,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRecipe {
    pub cover: String,
    pub title: String,
    pub description: String,
    pub time_to_cook: i64,
    pub difficulty: String,
    pub servings: i64,
    pub category: String,
    pub ingredients: Vec<String>,
    pub nutrients: Nutrients,
    pub guideline: String,
}

impl Into<model::CreateRecipeCommand> for CreateRecipe {
    fn into(self) -> model::CreateRecipeCommand {
        model::CreateRecipeCommand {
            cover: self.cover.into_bytes(),
            title: self.title,
            description: self.description,
            time_to_cook: self.time_to_cook,
            difficulty: self.difficulty,
            servings: self.servings,
            category: self.category,
            ingredients: self.ingredients,
            nutrients: self.nutrients.into(),
            guideline: self.guideline,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRecipe {
    pub cover: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub time_to_cook: Option<i64>,
    pub difficulty: Option<String>,
    pub servings: Option<i64>,
    pub category: Option<String>,
    pub ingredients: Option<Vec<String>>,
    pub nutrients: UpdateNutrients,
    pub guideline: Option<String>,
}

impl Into<model::UpdateRecipeCommand> for UpdateRecipe {
    fn into(self) -> model::UpdateRecipeCommand {
        model::UpdateRecipeCommand {
            id: String::default(),
            cover: match self.cover {
                Some(item) => Some(item.into_bytes()),
                None => None,
            },
            title: self.title,
            description: self.description,
            time_to_cook: self.time_to_cook,
            difficulty: self.difficulty,
            servings: self.servings,
            category: self.category,
            ingredients: self.ingredients,
            nutrients: self.nutrients.into(),
            guideline: self.guideline,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNutrients {
    pub proteins: Option<i64>,
    pub fats: Option<f64>,
    pub carbohydrates: Option<f64>,
    pub fiber: Option<i64>,
    pub kcal: Option<i64>,
}

impl Into<model::UpdateNutrients> for UpdateNutrients {
    fn into(self) -> model::UpdateNutrients {
        model::UpdateNutrients {
            proteins: self.proteins,
            fats: self.fats,
            carbohydrates: self.carbohydrates,
            fiber: self.fiber,
            kcal: self.kcal,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecipeSearchQuery {
    pub category_id: Option<String>,
}

impl Into<model::RecipeSearchQuery> for RecipeSearchQuery {
    fn into(self) -> model::RecipeSearchQuery {
        model::RecipeSearchQuery {
            category_id: self.category_id,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult<T> {
    pub count: i64,
    pub items: Vec<T>,
}

impl<I, O> From<model::SearchResult<I>> for SearchResult<O>
where
    O: From<I>,
{
    fn from(value: model::SearchResult<I>) -> Self {
        Self {
            count: value.count,
            items: value.items.into_iter().map(|item| item.into()).collect(),
        }
    }
}

pub struct AppError(pub Box<dyn Error>);

#[derive(Default, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ErrResponse {
    msg: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrResponse {
                msg: format!("Something went wrong: {}", self.0),
            }),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<Box<dyn Error>>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
