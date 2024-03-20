mod controller;
mod dao;
mod logic;
mod models;
use controller::user::login;
use dao::mysql::mysql::MyDatabase;
use rocket::{fairing::AdHoc, routes};
use rocket_db_pools::Database;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    print!("test");
    let _rocket = rocket::build()
        .attach(MyDatabase::init())
        .attach(AdHoc::try_on_ignite("MySQL Initializer", |rocket| {
            Box::pin(async move {
                let mut connection = match MyDatabase::fetch(&rocket) {
                    Some(pool) => match pool.acquire().await {
                        Ok(connection) => connection,
                        Err(_) => return Err(rocket),
                    },
                    None => return Err(rocket),
                };

                // let result = sqlx::query(
                //     "CREATE TABLE IF NOT EXISTS `example` ( `id` INT NOT NULL AUTO_INCREMENT, \
                //      `key` varchar(100), `value` TEXT , PRIMARY KEY (`id`), UNIQUE KEY (`key`) )",
                // )
                // .execute(&mut *connection)
                // .await;

                // match result {
                //     Ok(_) => Ok(rocket),
                //     Err(_) => Err(rocket),
                // }
                Ok(rocket)
            })
        }))
        .mount("/", routes![login])
        .launch()
        .await?;

    Ok(())
}
