use actix_web::{App, HttpServer, web::Data};
use sqlx::{Pool, Postgres};

mod database;
pub mod messages;
pub mod responses;
mod services;

use crate::services::users;

#[derive(Clone)]
pub struct AppState {
    pub client_db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Iniciando API");
    dotenv::dotenv().ok();
    env_logger::init();

    let pool = database::postgres_connection::connect().await;
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                client_db: pool.clone(),
            }))
            .configure(users::services::routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
