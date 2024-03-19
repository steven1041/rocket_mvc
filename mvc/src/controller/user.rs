use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::{
    catch, catchers, delete, get, http::Status, launch, post, put, routes, Request, State,
};

#[post("/login")]
pub async fn login() -> Value {
    json!({"message":"login"})
}
