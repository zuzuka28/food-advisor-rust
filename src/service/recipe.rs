use std::{error::Error, sync::Arc};

use crate::{
    model::{category::CategoryQuery, recipe::*, SearchResult},
    repository::RecipeRepository,
};

use super::CategoryService;

pub struct Config {
    pub category_service: Arc<CategoryService>,
    pub recipe_storage: Arc<RecipeRepository>,
}

pub struct RecipeService {
    pub category_service: Arc<CategoryService>,
    pub recipe_storage: Arc<RecipeRepository>,
}

impl RecipeService {
    pub fn new(cfg: Config) -> Self {
        Self {
            category_service: cfg.category_service,
            recipe_storage: cfg.recipe_storage,
        }
    }

    pub async fn fetch(&self, q: RecipeQuery) -> Result<Recipe, Box<dyn Error>> {
        self.recipe_storage.fetch(q).await
    }

    pub async fn create(&self, item: CreateRecipeCommand) -> Result<Recipe, Box<dyn Error>> {
        let cat = self
            .category_service
            .fetch(CategoryQuery {
                id: item.category.clone(),
            })
            .await?;

        let mut res = self.recipe_storage.create(item).await?;

        res.category = cat;

        Ok(res)
    }

    pub async fn update(&self, q: UpdateRecipeCommand) -> Result<Recipe, Box<dyn Error>> {
        self.recipe_storage.update(q).await
    }

    pub async fn delete(&self, q: DeleteRecipeCommand) -> Result<Recipe, Box<dyn Error>> {
        self.recipe_storage.delete(q).await
    }

    pub async fn search(
        &self,
        q: RecipeSearchQuery,
    ) -> Result<SearchResult<Recipe>, Box<dyn Error>> {
        self.recipe_storage.search(q).await
    }
}
