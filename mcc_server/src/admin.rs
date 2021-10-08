/// Endpoints called by managment frontend
use monsieurcc::{
    api::Api,
    schemas::{Recipe, RecipeType},
};
use rocket::{
    serde::json::Json,
    serde::{Deserialize, Serialize},
    Route,
};

use crate::{
    db::{self, RecipeInternal},
    schema,
};
use diesel::RunQueryDsl;

#[derive(Serialize, Deserialize, Debug)]
struct FetchSettings {
    language: String,
    recipe_type: RecipeType,
}

#[post("/recipes/sync", data = "<fetch_settings>")]
async fn sync_recipes(
    db: db::DbConn,
    fetch_settings: Json<FetchSettings>,
) -> Result<Json<Vec<Recipe>>, String> {
    let api = Api::new(&fetch_settings.language);

    let ret = api
        .get_recipes(None, Some(fetch_settings.recipe_type.clone()))
        .await;

    match ret {
        Ok(recipes) => {
            let rows: Vec<RecipeInternal> = recipes
                .iter()
                .map(|r| {
                    RecipeInternal::new(
                        r.data.name.clone(),
                        serde_json::to_string(r).unwrap(),
                        None,
                    )
                })
                .collect();

            db.run(move |conn| {
                diesel::insert_into(schema::recipes::table)
                    .values(&rows)
                    .execute(conn)
            })
            .await
            .expect("Failed to batch insert synced recipes");

            Ok(Json(recipes))
        }
        Err(e) => Err(format!("Failed to download recipes, err: {:?}", e)),
    }
}

pub(crate) fn routes() -> Vec<Route> {
    routes![sync_recipes]
}
