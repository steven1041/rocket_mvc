use crate::models::user::User;

use crate::dao::mysql::mysql::MyDatabase;
use rocket::form::Form;

use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::Error;
use rocket::{post, FromForm};
use rocket_db_pools::Connection;

#[derive(FromForm)]
pub struct UserGuard {
    pub username: String,
    pub password: String,
}

#[post("/login", data = "<user>")]
pub fn login(mut db: Connection<MyDatabase>, user: Form<UserGuard>) -> Value {
    crate::logic::user::login(db, user)
}

#[post("/signup")]
pub async fn signup() -> Value {
    json!({"message":"login"})
}
