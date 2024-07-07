pub(crate) mod category;
mod model;
pub(crate) mod recipe;
mod scheme;

pub use category::*;
pub use recipe::*;

use deadpool_diesel::postgres::Pool;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub async fn connect(connection_url: String) -> Pool {
    let manager =
        deadpool_diesel::postgres::Manager::new(connection_url, deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager).build().unwrap();

    // run the migrations on server startup
    {
        let conn = pool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    pool
}
