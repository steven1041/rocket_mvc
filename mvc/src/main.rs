mod controller;
use controller::user::login;

mod dao;

use rocket::routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    print!("test");
    let _rocket = rocket::build().mount("/", routes![login]).launch().await?;

    Ok(())
}
