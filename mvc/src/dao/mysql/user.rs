use crate::controller::user::UserGuard;
use crate::dao::mysql::mysql::MyDatabase;
use rocket::form::Form;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{self, Value};
use rocket_db_pools::sqlx::Error;

use rocket_db_pools::{
    sqlx::{self, FromRow, Row},
    Connection, Database, Pool,
};

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
struct RawUser {
    user_id: usize,
    username: String,
    password: String,
}
pub async fn login(db: &Connection<MyDatabase>, user: Form<UserGuard>) -> Option<Error> {
    let sql_str = "select user_id,username,password from user where username=?";
    let result: Result<RawUser, Error> = sqlx::query_as(sql_str).fetch_one(&mut **db).await;
    match result {
        Err(e) => return Some(Error::from("用户名不存在")),
        Ok(db_user) => {
            if user.password != db_user.password {
                return Some(Error::from("密码不正确"));
            } else {
                return None;
            }
        }
    }
}

pub async fn signup() -> Value {
    json!({"message":"login"})
}
