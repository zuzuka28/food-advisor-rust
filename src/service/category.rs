use crate::{model::category::*, model::SearchResult, repository};
use std::{error::Error, sync::Arc};

pub struct Config {
    pub category_storage: Arc<repository::CategoryRepository>,
}

pub struct CategoryService {
    pub category_storage: Arc<repository::CategoryRepository>,
}

impl CategoryService {
    pub fn new(cfg: Config) -> Self {
        Self {
            category_storage: cfg.category_storage,
        }
    }

    pub async fn fetch(&self, q: CategoryQuery) -> Result<Category, Box<dyn Error>> {
        self.category_storage.fetch(q).await
    }

    pub async fn create(&self, q: CreateCategoryCommand) -> Result<Category, Box<dyn Error>> {
        self.category_storage.create(q).await
    }

    pub async fn update(&self, q: UpdateCategoryCommand) -> Result<Category, Box<dyn Error>> {
        self.category_storage.update(q).await
    }

    pub async fn delete(&self, q: DeleteCategoryCommand) -> Result<Category, Box<dyn Error>> {
        self.category_storage.delete(q).await
    }

    pub async fn search(
        &self,
        q: CategorySearchQuery,
    ) -> Result<SearchResult<Category>, Box<dyn Error>> {
        self.category_storage.search(q).await
    }
}
