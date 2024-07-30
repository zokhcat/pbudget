use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NewBudget {
    pub name: String,
    pub total_amount: f64,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateBudget {
    pub name: Option<String>,
    pub total_amount: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct NewExpense {
    pub amount: f64,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateExpense {
    pub amount: Option<f64>,
    pub description: Option<String>,
}
