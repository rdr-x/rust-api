use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;
use sqlx;
mod api;

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let base_url = env::var("BASE_URL").expect("BASE_URL must be set");
    let pool = sqlx::PgPool::connect(&database_url).await.unwrap();
    let pool = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(
                web::scope("/api")
                    .configure(api::routers::user::init)
            )
    })
    .bind(&base_url)
    .unwrap()
    .run()
    .await
}



