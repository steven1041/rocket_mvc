use crate::controller::user::UserGuard;
use crate::dao::mysql::mysql::MyDatabase;
use rocket::form::Form;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{self, Value};
use rocket_db_pools::Connection;

pub async fn login(db: &Connection<MyDatabase>, user: Form<UserGuard>) -> Value {
    let result = crate::dao::mysql::user::login(db, user).await;
    match result {
        Some(err) => return json!({"message":err}),
        None => json!({"message":"login success"}),
    }
}

pub async fn signup() -> Value {
    json!({"message":"login"})
}
