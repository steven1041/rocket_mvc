use rocket::form::FromForm;

#[derive(FromForm)]
pub struct User {
    pub username: String,
    pub password: String,
}
