use actix_web::{http::Error, web, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::{
    middleware::auth::Auth,
    utility::{
        db_structs::{LoginInfo, NewUser},
        token::sign_jwt,
    },
};
use entities::users;
use sea_orm::{entity::*, DatabaseConnection, QueryFilter};
use uuid::Uuid;

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

async fn register(
    pool: web::Data<DatabaseConnection>,
    form: web::Json<NewUser>,
) -> Result<HttpResponse, Error> {
    let hashed_passowrd = hash(&form.password, DEFAULT_COST).unwrap();

    let new_user = users::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(form.username.clone()),
        password_hash: Set(hashed_passowrd),
        email: Set(form.email.clone()),
        ..Default::default()
    };

    let res = new_user.insert(pool.get_ref()).await;

    match res {
        Ok(insert_result) => {
            // Fetch the newly inserted user to return in the response
            let user = users::Entity::find_by_id(insert_result.id)
                .one(pool.get_ref())
                .await
                .unwrap();

            match user {
                Some(user) => Ok(HttpResponse::Ok().json(user)),
                None => Ok(HttpResponse::InternalServerError().finish()),
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

async fn login(
    pool: web::Data<DatabaseConnection>,
    form: web::Json<LoginInfo>,
) -> Result<HttpResponse, Error> {
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(form.username.clone()))
        .one(pool.get_ref())
        .await;

    match user {
        Ok(Some(user)) => {
            if verify(&form.password, &user.password_hash).unwrap() {
                let token = sign_jwt(user.id).unwrap();
                Ok(HttpResponse::Ok().json(token))
            } else {
                Ok(HttpResponse::Unauthorized().finish())
            }
        }
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
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
