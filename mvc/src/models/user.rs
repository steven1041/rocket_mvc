use rocket::form::FromForm;

pub struct User {
    pub username: String,
    pub password: String,
    pub token: String,
}
