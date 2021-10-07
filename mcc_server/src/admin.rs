use crate::utils::{
    generate_auth_token, get_user_from_credentials, get_user_from_token, is_registration_valid,
};
use monsieurcc::{api::Api, schemas::{
    AuthenticationRequest, AuthenticationResponse, Event, MachineConfig, MachineConfigResponse,
    Recipe, RecipeIds, RegistrationRequest, UserData, UserSettings, RecipeType,
}};
use rocket::{
    http::{ContentType, Status},
    response::status::{NoContent, Unauthorized},
    serde::json::Json,
    serde::{Serialize,Deserialize},
    Route,
};

#[derive(Serialize, Deserialize, Debug)]
struct FetchSettings {
    language: String,
    recipe_type: RecipeType,
}

#[post("/recipes/sync", data = "<fetch_settings>")]
async fn sync_recipes(fetch_settings: Json<FetchSettings>) -> Result<Json<Vec<Recipe>>, String> {
    let api = Api::new(&fetch_settings.language);

    let ret = api.get_recipes(None, Some(fetch_settings.recipe_type.clone()))
        .await;

    // TODO: Stuff it into DB

    match ret {
        Ok(recipes) => Ok(Json(recipes)),
        Err(e) => Err(format!("Failed to download recipes, err: {:?}", e)),
    }
}

pub(crate) fn routes() -> Vec<Route> {
    routes![
        sync_recipes
    ]
}
