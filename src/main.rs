use actix_web::{App,web::Data, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod services;
use services::{create_user, create_user_article, fetch_user_articles,fetch_users };

pub struct AppState {
    pub db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set NotPresent");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool connection");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db:pool.clone() }))
            .service(fetch_users)
            .service(fetch_user_articles)
            .service(create_user_article)
            .service(create_user)
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await

}