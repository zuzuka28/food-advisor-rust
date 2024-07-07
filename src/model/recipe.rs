use super::category::Category;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Recipe {
    pub id: String,
    pub cover: Vec<u8>,
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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct CreateRecipeCommand {
    pub cover: Vec<u8>,
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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateRecipeCommand {
    pub id: String,
    pub cover: Option<Vec<u8>>,
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

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteRecipeCommand {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RecipeQuery {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct RecipeSearchQuery {
    pub category_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Nutrients {
    pub proteins: i64,
    pub fats: f64,
    pub carbohydrates: f64,
    pub fiber: i64,
    pub kcal: i64,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UpdateNutrients {
    pub proteins: Option<i64>,
    pub fats: Option<f64>,
    pub carbohydrates: Option<f64>,
    pub fiber: Option<i64>,
    pub kcal: Option<i64>,
}
