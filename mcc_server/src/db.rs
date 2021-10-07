use crate::schema::*;
use rocket::{Rocket, Build};
use rocket_sync_db_pools::{diesel, database};
use rocket::fairing::AdHoc;
use rocket::response::{Debug, status::Created};
use rocket::serde::{Serialize, Deserialize, json::Json};

#[database("mcc_recipes")]
pub struct Db(diesel::SqliteConnection);

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
        rocket.attach(Db::fairing())
    })
}