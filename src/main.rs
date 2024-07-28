mod handler;
mod middleware;
mod utility;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

// Lambda Function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db: DatabaseConnection = Database::connect("sqlite://store.sqlite?mode=rwc")
        .await
        .expect("Failed to connect Database");
    Migrator::up(&db, None)
        .await
        .expect("Failed to migrate database schema");

    HttpServer::new(|| App::new().configure(handler::init))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
