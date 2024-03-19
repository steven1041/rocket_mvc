use rocket_db_pools::{
    sqlx::{self, database},
    Database,
};

#[derive(Database)]
#[database("example")]
pub struct MyDatabase(sqlx::MySqlPool);
