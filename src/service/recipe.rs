use std::{collections::HashMap, error::Error, sync::Arc};

use crate::{
    model::{category::CategoryQuery, recipe::*, CategorySearchQuery, SearchResult},
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
        let mut item = self.recipe_storage.fetch(q).await?;

        item.category = self
            .category_service
            .fetch(CategoryQuery {
                id: item.category.id,
            })
            .await?;

        Ok(item)
    }

    pub async fn create(&self, item: CreateRecipeCommand) -> Result<Recipe, Box<dyn Error>> {
        let cat = self
            .category_service
            .fetch(CategoryQuery {
                id: item.category.clone(),
            })
            .await
            .map_err(|_| format!("category with id {} not found", item.category.clone()))?;

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
        let mut res = self.recipe_storage.search(q).await?;

        let category_ids = res
            .items
            .iter()
            .map(|item| Some(item.category.id.clone()))
            .collect();
        let categories = self
            .category_service
            .search(CategorySearchQuery { ids: category_ids })
            .await?;
        let category_to_name = categories
            .items
            .into_iter()
            .map(|item| (item.id, item.name))
            .collect::<HashMap<String, String>>();

        for item in res.items.iter_mut() {
            item.category.name = category_to_name.get(&item.category.id).unwrap().clone();
        }

        Ok(res)
    }
}
