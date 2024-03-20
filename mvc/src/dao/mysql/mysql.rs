use rocket_db_pools::{
    sqlx::{self},
    Database,
};

#[derive(Database)]
#[database("example")]
pub struct MyDatabase(sqlx::MySqlPool);
