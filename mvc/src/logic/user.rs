use rocket::serde::json::serde_json::json;
use rocket::serde::json::{self, Value};

pub async fn login() -> Value {
    json!({"message":"login"})
}

pub async fn signup() -> Value {
    json!({"message":"login"})
}
