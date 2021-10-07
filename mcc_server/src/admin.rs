use monsieurcc::{api::Api, schemas::{
    Recipe, RecipeType,
}};
use rocket::{
    serde::json::Json,
    serde::{Serialize,Deserialize},
    Route,
};

use crate::db;

#[derive(Serialize, Deserialize, Debug)]
struct FetchSettings {
    language: String,
    recipe_type: RecipeType,
}

#[post("/recipes/sync", data = "<fetch_settings>")]
async fn sync_recipes(db: db::DbConn, fetch_settings: Json<FetchSettings>) -> Result<Json<Vec<Recipe>>, String> {
    let api = Api::new(&fetch_settings.language);

    let ret = api
        .get_recipes(None, Some(fetch_settings.recipe_type.clone()))
        .await;

    match ret {
        Ok(recipes) => {
            let _ = recipes.iter()
                .map(|r| {
                    (r, serde_json::to_string(r).unwrap())
                })
                .for_each(|(r, serialized)| {
                    // TODO: Init DB rows
                });

            // TODO: Batch insert into DB

            Ok(Json(recipes))
        },
        Err(e) => Err(format!("Failed to download recipes, err: {:?}", e)),
    }
}

pub(crate) fn routes() -> Vec<Route> {
    routes![
        sync_recipes
    ]
}
