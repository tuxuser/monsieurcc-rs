/// Database declaration and model definition
use crate::schema::*;
use rocket::fairing::AdHoc;
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::{database, diesel};

#[database("mcc_recipes")]
pub struct DbConn(diesel::SqliteConnection);

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "recipes"]
pub(crate) struct RecipeInternal {
    pub id: Option<i32>,
    pub name: String,
    pub json_data: String,
    pub image_file: Option<String>,
}

impl RecipeInternal {
    pub fn new(name: String, json_data: String, image_file: Option<String>) -> Self {
        RecipeInternal {
            id: None,
            name,
            json_data,
            image_file,
        }
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket.attach(DbConn::fairing())
    })
}
