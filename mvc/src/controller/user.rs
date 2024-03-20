use crate::models::user::User;

use rocket::form::Form;
use rocket::post;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::Error;

#[post("/login", data = "<user>")]
pub fn login(user: Form<User>) -> Value {
    json!({"message":"login"})
}

#[post("/signup")]
pub async fn signup() -> Value {
    json!({"message":"login"})
}
