mod handler;

use actix_web::{App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: DatabaseConnection = Database::connect("sqlite://store.sqlite?mode=rwc")
        .await
        .expect("Failed to connect Database");
    Migrator::up(&db, None)
        .await
        .expect("Failed to migrate the database");
    HttpServer::new(|| App::new().configure(handler::init))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
