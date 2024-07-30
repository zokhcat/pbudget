use actix_web::{http::Error, web, HttpResponse};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;

use crate::{
    middleware::auth::Auth,
    utility::{
        db_structs::{
            LoginInfo, NewBudget, NewExpense, NewUser, UpdateBudget, UpdateExpense, UpdateUser,
        },
        token::sign_jwt,
    },
};
use entities::{budget, expense, users};
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

async fn get_profile(
    user_id: web::ReqData<Uuid>,
    pool: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let user = users::Entity::find_by_id(user_id.into_inner().clone())
        .one(pool.get_ref())
        .await;

    match user {
        Ok(Some(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

async fn update_profile(
    user_id: web::ReqData<Uuid>,
    pool: web::Data<DatabaseConnection>,
    form: web::Json<UpdateUser>,
) -> Result<HttpResponse, Error> {
    let user = users::Entity::find_by_id(user_id.into_inner().clone())
        .one(pool.get_ref())
        .await;

    match user {
        Ok(Some(user)) => {
            let mut user: users::ActiveModel = user.into();

            if let Some(username) = &form.username {
                user.username = Set(username.clone());
            }

            if let Some(password) = &form.password {
                let hashed_password = hash(password, DEFAULT_COST).unwrap();
                user.password_hash = Set(hashed_password);
            }

            if let Some(email) = &form.email {
                user.email = Set(email.clone())
            }

            let res = user.update(pool.get_ref()).await;

            match res {
                Ok(user) => Ok(HttpResponse::Ok().json(user)),
                Err(_) => Ok(HttpResponse::InternalServerError().finish()),
            }
        }
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

async fn get_budgets(
    user_id: web::ReqData<Uuid>,
    pool: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let all_budgets = budget::Entity::find()
        .filter(budget::Column::UserId.eq(user_id.into_inner().clone()))
        .all(pool.get_ref())
        .await;

    match all_budgets {
        Ok(all_budgets) => Ok(HttpResponse::Ok().json(all_budgets)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

async fn get_budget(
    user_id: web::ReqData<Uuid>,
    budget_id: web::Path<Uuid>,
    pool: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let budget = budget::Entity::find()
        .filter(budget::Column::Id.eq(budget_id.into_inner().clone()))
        .filter(users::Column::Id.eq(user_id.into_inner().clone()))
        .one(pool.get_ref())
        .await;

    match budget {
        Ok(Some(budget)) => Ok(HttpResponse::Ok().json(budget)),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

async fn post_budget(
    user_id: web::ReqData<Uuid>,
    pool: web::Data<DatabaseConnection>,
    form: web::Json<NewBudget>,
) -> Result<HttpResponse, Error> {
    let new_budget = budget::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(user_id.into_inner().clone()),
        name: Set(form.name.clone()),
        total_amount: Set(form.total_amount.clone()),
        created_at: Set(Utc::now().naive_utc().to_string()),
        updated_at: Set(Utc::now().naive_utc().to_string()),
        ..Default::default()
    };

    let res = new_budget.insert(pool.get_ref()).await;

    match res {
        Ok(insert_budget) => {
            let budget = budget::Entity::find_by_id(insert_budget.id)
                .one(pool.get_ref())
                .await
                .unwrap();

            match budget {
                Some(budget) => Ok(HttpResponse::Ok().json(budget)),
                None => Ok(HttpResponse::InternalServerError().finish()),
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

async fn update_budget(
    user_id: web::ReqData<Uuid>,
    budget_id: web::Path<Uuid>,
    pool: web::Data<DatabaseConnection>,
    form: web::Data<UpdateBudget>,
) -> Result<HttpResponse, Error> {
    let budget = budget::Entity::find()
        .filter(users::Column::Id.eq(user_id.into_inner().clone()))
        .filter(budget::Column::Id.eq(budget_id.into_inner().clone()))
        .one(pool.get_ref())
        .await;

    match budget {
        Ok(Some(budget)) => {
            let mut budget: budget::ActiveModel = budget.into();

            if let Some(name) = &form.name {
                budget.name = Set(name.clone());
            }

            if let Some(total_amount) = &form.total_amount {
                budget.total_amount = Set(total_amount.clone());
            }

            budget.updated_at = Set(Utc::now().naive_utc().to_string());

            let res = budget.update(pool.get_ref()).await;

            match res {
                Ok(budget) => Ok(HttpResponse::Ok().json(budget)),
                Err(_) => Ok(HttpResponse::InternalServerError().finish()),
            }
        }
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

async fn delete_budget(
    user_id: web::ReqData<Uuid>,
    budget_id: web::Path<Uuid>,
    pool: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let res = budget::Entity::delete_many()
        .filter(users::Column::Id.eq(user_id.into_inner().clone()))
        .filter(budget::Column::Id.eq(budget_id.into_inner().clone()))
        .exec(pool.get_ref())
        .await;

    match res {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

async fn get_expenses(
    user_id: web::ReqData<Uuid>,
    budget_id: web::Path<Uuid>,
    pool: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let expenses = expense::Entity::find()
        .filter(users::Column::Id.eq(user_id.into_inner().clone()))
        .filter(budget::Column::Id.eq(budget_id.into_inner().clone()))
        .all(pool.get_ref())
        .await;

    match expenses {
        Ok(expenses) => Ok(HttpResponse::Ok().json(expenses)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

async fn get_expense(
    user_id: web::ReqData<Uuid>,
    budget_id: web::Path<Uuid>,
    expense: web::Path<Uuid>,
    pool: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let expense = expense::Entity::find()
        .filter(users::Column::Id.eq(user_id.into_inner().clone()))
        .filter(budget::Column::Id.eq(budget_id.into_inner().clone()))
        .filter(expense::Column::Id.eq(expense.into_inner().clone()))
        .one(pool.get_ref())
        .await;

    match expense {
        Ok(Some(expense)) => Ok(HttpResponse::Ok().json(expense)),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

async fn post_expense(
    budget_id: web::Path<Uuid>,
    form: web::Json<NewExpense>,
    pool: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let new_expense = expense::ActiveModel {
        id: Set(Uuid::new_v4()),
        budget_id: Set(budget_id.into_inner().clone()),
        amount: Set(form.amount.clone()),
        description: Set(form.description.clone()),
        date: Set(Utc::now().date_naive().to_string()),
        created_at: Set(Utc::now().naive_utc().to_string()),
        updated_at: Set(Utc::now().naive_utc().to_string()),
        ..Default::default()
    };

    let res = new_expense.insert(pool.get_ref()).await;

    match res {
        Ok(insert_expense) => {
            let expense = expense::Entity::find_by_id(insert_expense.id)
                .one(pool.get_ref())
                .await
                .unwrap();

            match expense {
                Some(expense) => Ok(HttpResponse::Ok().json(expense)),
                None => Ok(HttpResponse::InternalServerError().finish()),
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

async fn update_expense(
    user_id: web::ReqData<Uuid>,
    budget_id: web::Path<Uuid>,
    expense_id: web::Path<Uuid>,
    form: web::Data<UpdateExpense>,
    pool: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let expense = expense::Entity::find()
        .filter(users::Column::Id.eq(user_id.into_inner().clone()))
        .filter(budget::Column::Id.eq(budget_id.into_inner().clone()))
        .filter(expense::Column::Id.eq(expense_id.into_inner().clone()))
        .one(pool.get_ref())
        .await;

    match expense {
        Ok(Some(expense)) => {
            let mut expense: expense::ActiveModel = expense.into();

            if let Some(amount) = &form.amount {
                expense.amount = Set(amount.clone());
            }

            if let Some(description) = &form.description {
                expense.description = Set(description.clone());
            }

            expense.updated_at = Set(Utc::now().naive_utc().to_string());

            let res = expense.update(pool.as_ref()).await;

            match res {
                Ok(expense) => Ok(HttpResponse::Ok().json(expense)),
                Err(_) => Ok(HttpResponse::InternalServerError().finish()),
            }
        }
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}

async fn delete_expense(
    user_id: web::ReqData<Uuid>,
    budget_id: web::Path<Uuid>,
    expense_id: web::Path<Uuid>,
    pool: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, Error> {
    let res = budget::Entity::delete_many()
        .filter(users::Column::Id.eq(user_id.into_inner().clone()))
        .filter(budget::Column::Id.eq(budget_id.into_inner().clone()))
        .filter(expense::Column::Id.eq(expense_id.into_inner().clone()))
        .exec(pool.get_ref())
        .await;

    match res {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
