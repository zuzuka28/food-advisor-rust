use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use std::{error::Error, sync::Arc};
use uuid::Uuid;

use crate::model::{category as app_model, SearchResult};

use super::{model as db_model, scheme};

pub struct CategoryRepository {
    pool: Arc<Pool>,
}

impl CategoryRepository {
    pub async fn new(pool: Arc<Pool>) -> Self {
        CategoryRepository { pool }
    }

    pub async fn create(
        &self,
        item: app_model::CreateCategoryCommand,
    ) -> Result<app_model::Category, Box<dyn Error>> {
        let conn = self.pool.get().await?;

        let category_resp = conn
            .interact(move |conn| {
                let mut new_category: db_model::CreateCategory = item.into();
                new_category.uuid = Uuid::new_v4().to_string();

                diesel::insert_into(scheme::categories::table)
                    .values(new_category)
                    .returning(db_model::Category::as_returning())
                    .get_result(conn)
            })
            .await??;

        Ok(category_resp.into())
    }

    pub async fn fetch(
        &self,
        q: app_model::CategoryQuery,
    ) -> Result<app_model::Category, Box<dyn Error>> {
        let conn = self.pool.get().await?;

        let category_resp = conn
            .interact(|conn| {
                scheme::categories::table
                    .filter(scheme::categories::uuid.eq(q.id))
                    .limit(1)
                    .select(db_model::Category::as_select())
                    .get_result(conn)
            })
            .await??;

        Ok(category_resp.into())
    }

    pub async fn update(
        &self,
        q: app_model::UpdateCategoryCommand,
    ) -> Result<app_model::Category, Box<dyn Error>> {
        let conn = self.pool.get().await?;

        let category_resp = conn
            .interact(|conn| {
                let category_id = q.id.clone();
                let category_update: db_model::UpdateCategory = q.into();

                diesel::update(scheme::categories::table)
                    .filter(scheme::categories::uuid.eq(&category_id))
                    .set(category_update)
                    .execute(conn)?;

                scheme::categories::table
                    .filter(scheme::categories::uuid.eq(&category_id))
                    .limit(1)
                    .select(db_model::Category::as_select())
                    .get_result(conn)
            })
            .await??;

        Ok(category_resp.into())
    }

    pub async fn delete(
        &self,
        q: app_model::DeleteCategoryCommand,
    ) -> Result<app_model::Category, Box<dyn Error>> {
        let conn = self.pool.get().await?;

        let category_resp = self
            .fetch(app_model::CategoryQuery { id: q.id.clone() })
            .await?;

        conn.interact(|conn| {
            diesel::delete(scheme::categories::table)
                .filter(scheme::categories::uuid.eq(q.id))
                .execute(conn)
        })
        .await??;

        Ok(category_resp)
    }

    pub async fn search(
        &self,
        q: app_model::CategorySearchQuery,
    ) -> Result<SearchResult<app_model::Category>, Box<dyn Error>> {
        let conn = self.pool.get().await?;

        let category_resp = conn
            .interact(|conn| {
                let mut myq = scheme::categories::table.into_boxed();

                if let Some(category_ids) = q.ids {
                    myq = myq.filter(scheme::categories::uuid.eq_any(category_ids));
                }

                myq.select(db_model::Category::as_select())
                    .get_results(conn)
            })
            .await??;

        Ok(SearchResult {
            count: category_resp.len() as i64,
            items: category_resp.into_iter().map(|item| item.into()).collect(),
        })
    }
}
