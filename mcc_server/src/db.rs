/// Database declaration and model definition
use crate::schema::*;
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::{database, diesel};

#[database("mcc_recipes")]
pub struct DbConn(diesel::SqliteConnection);

#[derive(Queryable, Deserialize, Serialize, Debug)]
pub(crate) struct RecipeInternal {
    pub id: i32,
    pub name: String,
    pub json_data: String,
    pub image_file: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "recipes"]
pub(crate) struct NewRecipeInternal {
    pub name: String,
    pub json_data: String,
}

#[derive(Queryable, Serialize, Debug)]
pub(crate) struct ShortRecipeInternal {
    pub id: i32,
    pub name: String,
    pub image_file: Option<String>,
}