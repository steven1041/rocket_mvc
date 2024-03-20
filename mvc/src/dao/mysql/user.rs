use crate::dao::mysql::mysql::MyDatabase;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{self, Value};
use rocket_db_pools::{
    sqlx::{self, Row},
    Connection, Database,
};
pub async fn login() -> Value {
    json!({"message":"login"})
}

pub async fn signup() -> Value {
    json!({"message":"login"})
}
