use actix_web::{web, HttpResponse};

use crate::middleware::auth::Auth;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("")
                    .route("/register", web::post().to(register))
                    .route("/login", web::post().to(login)),
            )
            .service(
                web::scope("")
                    .wrap(Auth)
                    .route("/profile", web::get().to(get_profile))
                    .route("/profile", web::put().to(update_profile))
                    .route("/budget", web::get().to(get_budgets))
                    .route("/budget", web::post().to(post_budget))
                    .route("/budget/{id}", web::get().to(get_budget))
                    .route("/budget/{id}", web::post().to(post_budget))
                    .route("/budget/{id}", web::put().to(update_budget))
                    .route("/budget/{id}", web::delete().to(delete_budget))
                    .route("/budget/{id}/expenses", web::get().to(get_expenses))
                    .route("/budget/{id}/expenses", web::post().to(post_expense))
                    .route(
                        "/budget/{id}/expenses/{expense_id}",
                        web::get().to(get_expense),
                    )
                    .route(
                        "/budget/{id}/expenses/{expense_id}",
                        web::put().to(update_expense),
                    )
                    .route(
                        "/budget/{id}/expenses/{expense_id}",
                        web::delete().to(delete_expense),
                    ),
            ),
    );
}

async fn register() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn login() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn get_profile() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn update_profile() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn get_budgets() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn get_budget() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn post_budget() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn update_budget() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn delete_budget() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn get_expense() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn post_expense() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn update_expense() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn delete_expense() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn get_expenses() -> HttpResponse {
    HttpResponse::Ok().finish()
}
