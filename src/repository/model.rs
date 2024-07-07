use diesel::prelude::*;

use crate::model;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = super::scheme::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub id: i32,
    pub uuid: String,
    pub name: String,
}

impl Into<model::Category> for Category {
    fn into(self) -> model::Category {
        model::Category {
            id: self.uuid,
            name: self.name,
        }
    }
}

#[derive(Default, Insertable)]
#[diesel(table_name = super::scheme::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateCategory {
    pub uuid: String,
    pub name: String,
}

impl From<model::CreateCategoryCommand> for CreateCategory {
    fn from(value: model::CreateCategoryCommand) -> Self {
        Self {
            uuid: String::default(),
            name: value.name,
        }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = super::scheme::categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateCategory {
    pub name: Option<String>,
}

impl From<model::UpdateCategoryCommand> for UpdateCategory {
    fn from(value: model::UpdateCategoryCommand) -> Self {
        Self {
            name: Some(value.name),
        }
    }
}

#[derive(Default, Clone, Queryable, Selectable, Associations, Identifiable)]
#[diesel(belongs_to(Category))]
#[diesel(table_name = super::scheme::recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Recipe {
    pub id: i32,
    pub uuid: String,
    pub cover: Vec<u8>,
    pub title: String,
    pub description: String,
    pub time_to_cook: i64,
    pub difficulty: String,
    pub servings: i64,
    pub category_id: String,
    pub ingredients: Vec<Option<String>>,
    pub guideline: String,
    pub nutrients_proteins: i64,
    pub nutrients_fats: f64,
    pub nutrients_carbohydrates: f64,
    pub nutrients_fiber: i64,
    pub nutrients_kcal: i64,
}

impl Into<model::Recipe> for Recipe {
    fn into(self) -> model::Recipe {
        model::Recipe {
            id: self.uuid,
            cover: self.cover,
            title: self.title,
            description: self.description,
            time_to_cook: self.time_to_cook,
            difficulty: self.difficulty,
            servings: self.servings,
            category: model::Category {
                id: self.category_id,
                ..model::Category::default()
            },
            ingredients: self
                .ingredients
                .iter()
                .map(|v| match v.to_owned() {
                    Some(v) => v,
                    _ => String::from(""),
                })
                .collect(),
            nutrients: model::Nutrients {
                proteins: self.nutrients_proteins,
                fats: self.nutrients_fats,
                carbohydrates: self.nutrients_carbohydrates,
                fiber: self.nutrients_fiber,
                kcal: self.nutrients_kcal,
            },
            guideline: self.guideline,
        }
    }
}

#[derive(Default, Insertable)]
#[diesel(table_name = super::scheme::recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateRecipe {
    pub uuid: String,
    pub cover: Vec<u8>,
    pub title: String,
    pub description: String,
    pub time_to_cook: i64,
    pub difficulty: String,
    pub servings: i64,
    pub category_id: String,
    pub ingredients: Vec<Option<String>>,
    pub guideline: String,
    pub nutrients_proteins: i64,
    pub nutrients_fats: f64,
    pub nutrients_carbohydrates: f64,
    pub nutrients_fiber: i64,
    pub nutrients_kcal: i64,
}

impl From<model::CreateRecipeCommand> for CreateRecipe {
    fn from(value: model::CreateRecipeCommand) -> Self {
        CreateRecipe {
            uuid: String::default(),
            cover: value.cover,
            title: value.title,
            description: value.description,
            time_to_cook: value.time_to_cook,
            difficulty: value.difficulty,
            servings: value.servings,
            category_id: String::default(),
            ingredients: value
                .ingredients
                .iter()
                .map(|v| Some(v.to_owned()))
                .collect(),
            guideline: value.guideline,
            nutrients_proteins: value.nutrients.proteins,
            nutrients_fats: value.nutrients.fats,
            nutrients_carbohydrates: value.nutrients.carbohydrates,
            nutrients_fiber: value.nutrients.fiber,
            nutrients_kcal: value.nutrients.kcal,
        }
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = super::scheme::recipes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateRecipe {
    pub cover: Option<Vec<u8>>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub time_to_cook: Option<i64>,
    pub difficulty: Option<String>,
    pub servings: Option<i64>,
    pub category_id: Option<String>,
    pub ingredients: Option<Vec<Option<String>>>,
    pub guideline: Option<String>,
    pub nutrients_proteins: Option<i64>,
    pub nutrients_fats: Option<f64>,
    pub nutrients_carbohydrates: Option<f64>,
    pub nutrients_fiber: Option<i64>,
    pub nutrients_kcal: Option<i64>,
}

impl From<model::UpdateRecipeCommand> for UpdateRecipe {
    fn from(value: model::UpdateRecipeCommand) -> Self {
        UpdateRecipe {
            cover: value.cover,
            title: value.title,
            description: value.description,
            time_to_cook: value.time_to_cook,
            difficulty: value.difficulty,
            servings: value.servings,
            category_id: value.category,
            ingredients: match value.ingredients {
                Some(items) => Some(items.iter().map(|v| Some(v.to_owned())).collect()),
                None => None,
            },
            guideline: value.guideline,
            nutrients_proteins: value.nutrients.proteins,
            nutrients_fats: value.nutrients.fats,
            nutrients_carbohydrates: value.nutrients.carbohydrates,
            nutrients_fiber: value.nutrients.fiber,
            nutrients_kcal: value.nutrients.kcal,
        }
    }
}
