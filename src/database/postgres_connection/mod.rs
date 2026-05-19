use crate::messages::errors_messages::MIGRATION_ERROR;
use crate::messages::info_messages::MIGRATION_APPLIED;
use sqlx::{Pool, Postgres};

pub async fn connect() -> Pool<Postgres> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let check_migrate = sqlx::migrate!("./src/database/postgres_connection/migrations")
        .run(&pool)
        .await;

    match check_migrate {
        Ok(_) => println!("{}", MIGRATION_APPLIED),
        Err(e) => println!("{} {}", MIGRATION_ERROR, e),
    }

    pool
}
