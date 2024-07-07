use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use std::{error::Error, sync::Arc};
use uuid::Uuid;

use crate::model::{self as app_model, SearchResult};

use super::{model as db_model, scheme};

pub struct RecipeRepository {
    pool: Arc<Pool>,
}

impl RecipeRepository {
    pub async fn new(pool: Arc<Pool>) -> Self {
        RecipeRepository { pool }
    }

    pub async fn create(
        &self,
        item: app_model::CreateRecipeCommand,
    ) -> Result<app_model::Recipe, Box<dyn Error>> {
        let conn = self.pool.get().await?;

        let mut new_recipe: db_model::CreateRecipe = item.into();
        let new_recipe_uuid = Uuid::new_v4().to_string();
        new_recipe.uuid = new_recipe_uuid.clone();

        let recipe_resp = conn
            .interact(|conn| {
                diesel::insert_into(scheme::recipes::table)
                    .values(new_recipe)
                    .returning(db_model::Recipe::as_returning())
                    .get_result(conn)
            })
            .await??;

        Ok(recipe_resp.into())
    }

    pub async fn fetch(
        &self,
        q: app_model::RecipeQuery,
    ) -> Result<app_model::Recipe, Box<dyn Error>> {
        let conn = self.pool.get().await?;

        let recipe_resp = conn
            .interact(|conn| {
                scheme::recipes::table
                    .filter(scheme::recipes::uuid.eq(q.id))
                    .limit(1)
                    .select(db_model::Recipe::as_select())
                    .get_result(conn)
            })
            .await??;

        Ok(recipe_resp.into())
    }

    pub async fn update(
        &self,
        q: app_model::UpdateRecipeCommand,
    ) -> Result<app_model::Recipe, Box<dyn Error>> {
        let conn = self.pool.get().await?;

        let recipe_resp = conn
            .interact(|conn| {
                let recipe_id = q.id.clone();
                let recipe_update: db_model::UpdateRecipe = q.into();

                diesel::update(scheme::recipes::table)
                    .filter(scheme::recipes::uuid.eq(&recipe_id))
                    .set(recipe_update)
                    .execute(conn)?;

                scheme::recipes::table
                    .filter(scheme::recipes::uuid.eq(&recipe_id))
                    .limit(1)
                    .select(db_model::Recipe::as_select())
                    .get_result(conn)
            })
            .await??;

        Ok(recipe_resp.into())
    }

    pub async fn delete(
        &self,
        q: app_model::DeleteRecipeCommand,
    ) -> Result<app_model::Recipe, Box<dyn Error>> {
        let conn = self.pool.get().await?;

        let recipe_resp = self
            .fetch(app_model::RecipeQuery { id: q.id.clone() })
            .await?;

        conn.interact(|conn| {
            diesel::delete(scheme::recipes::table)
                .filter(scheme::recipes::uuid.eq(q.id))
                .execute(conn)
        })
        .await??;

        Ok(recipe_resp)
    }

    pub async fn search(
        &self,
        q: app_model::RecipeSearchQuery,
    ) -> Result<SearchResult<app_model::Recipe>, Box<dyn Error>> {
        let conn = self.pool.get().await?;

        let recipe_resp = conn
            .interact(|conn| {
                let mut myq = scheme::recipes::table.into_boxed();

                if let Some(category_id) = q.category_id {
                    myq = myq.filter(scheme::recipes::category_id.eq(category_id));
                }

                myq.select(db_model::Recipe::as_select()).get_results(conn)
            })
            .await??;

        Ok(SearchResult {
            count: recipe_resp.len() as i64,
            items: recipe_resp.into_iter().map(|item| item.into()).collect(),
        })
    }
}
