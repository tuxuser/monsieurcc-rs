use crate::schema::*;
use rocket_sync_db_pools::{diesel, database};
use rocket::fairing::AdHoc;
use rocket::serde::{Serialize, Deserialize};

#[database("mcc_recipes")]
pub struct DbConn(diesel::SqliteConnection);

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name="recipes"]
struct RecipeInternal {
    id: Option<i32>,
    name: String,
    json_data: String,
    image_file: Option<String>,
}


pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket.attach(DbConn::fairing())
    })
}