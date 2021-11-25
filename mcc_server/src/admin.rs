use diesel::QueryDsl;
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
use std::convert::TryInto;

use crate::diesel::RunQueryDsl;
use crate::{
    db::{self, NewRecipeInternal, RecipeInternal, RecipeShort},
    schema,
};

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
    let ret = Api::new()
        .get_recipes(
            &fetch_settings.language,
            Some(fetch_settings.recipe_type.clone()),
        )
        .await;

    match ret {
        Ok(r) => {
            // Construct insertable rows
            let rows: Vec<NewRecipeInternal> = r
                .iter()
                .map(|r| NewRecipeInternal {
                    name: r.data.name.clone(),
                    json_data: serde_json::to_string(r).unwrap(),
                    image_file: None,
                    lang: Some(fetch_settings.language.clone()),
                    original_id: Some(r.data.id.try_into().unwrap()),
                    is_custom: Some(false),
                    recipe_type: Some(fetch_settings.recipe_type.to_string()),
                })
                .collect();

            // Batch insert rows
            // TODO: Upsert handling
            db.run(move |conn| {
                diesel::insert_into(schema::recipes::table)
                    .values(&rows)
                    .execute(conn)
            })
            .await
            .expect("Failed to batch insert synced recipes");

            Ok(Json(r))
        }
        Err(e) => Err(format!("Failed to download recipes, err: {:?}", e)),
    }
}

#[get("/recipes/update_images")]
async fn update_images() -> &'static str {
    "TODO /recipes/update_images"
}

#[get("/recipes/overview")]
async fn get_overview(db: db::DbConn) -> Json<Vec<RecipeShort>> {
    let result = db
        .run(move |conn| {
            schema::recipes::table
                .select((
                    schema::recipes::id,
                    schema::recipes::name,
                    schema::recipes::image_file,
                ))
                .load::<RecipeShort>(conn)
        })
        .await
        .unwrap();

    Json(result)
}

#[post("/recipes")]
async fn add_recipe() -> Result<Json<RecipeInternal>, &'static str> {
    Err("TODO POST /recipes")
}

#[get("/recipes/<id>")]
async fn get_recipe(id: i32) -> Result<Json<RecipeInternal>, &'static str> {
    Err("TODO GET /recipes/<id>")
}

#[put("/recipes/<id>")]
async fn update_recipe(id: i32) -> Result<Json<RecipeInternal>, &'static str> {
    Err("TODO PUT /recipes/<id>")
}

#[delete("/recipes/<id>")]
async fn delete_recipe(id: i32) -> Result<Json<RecipeInternal>, &'static str> {
    Err("TODO DELETE /recipes/<id>")
}

pub(crate) fn routes() -> Vec<Route> {
    routes![
        sync_recipes,
        update_images,
        get_overview,
        add_recipe,
        get_recipe,
        update_recipe,
        delete_recipe
    ]
}
