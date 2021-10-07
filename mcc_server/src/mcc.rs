use crate::{db,schema::recipes,utils::{
    generate_auth_token, get_user_from_credentials, get_user_from_token, is_registration_valid,
}};
use diesel::{RunQueryDsl, query_dsl::methods::SelectDsl};
use monsieurcc::schemas::{
    AuthenticationRequest, AuthenticationResponse, Event, MachineConfig, MachineConfigResponse,
    Recipe, RecipeIds, RegistrationRequest, UserData, UserSettings,
};
use rocket::{
    http::{ContentType, Status},
    response::status::{NoContent, Unauthorized},
    serde::json::Json,
    Route,
};

#[post("/register", data = "<request>")]
fn register(request: Json<RegistrationRequest<'_>>) -> (Status, (ContentType, &'static str)) {
    if !is_registration_valid(request.uid, request.displayname, request.password) {
        // Invalid email: {"constraint":"userdata_pkey"}
        // Invalid nickname: {"constraint":"idx_unique_displayname"}
        return (
            Status::Conflict,
            (
                ContentType::JSON,
                r#"{"constraint":"idx_unique_displayname"}"#,
            ),
        );
    }

    (Status::Created, (ContentType::Plain, "Created"))
}

#[post("/authenticate", data = "<user>")]
fn authenticate(
    user: Json<AuthenticationRequest<'_>>,
) -> Result<Json<AuthenticationResponse>, Unauthorized<&'static str>> {
    let user = get_user_from_credentials(user.username, user.password);
    match user {
        Ok(userdata) => {
            let response = AuthenticationResponse {
                token: generate_auth_token(),
                displayname: userdata.displayname,
                lang: userdata.country,
            };
            Ok(Json(response))
        }
        Err(_) => Err(Unauthorized(Some("Unauthorized"))),
    }
}

#[get("/userdata")]
fn get_userdata() -> Result<Json<UserData>, Unauthorized<&'static str>> {
    // TODO: Get X-Auth-Key from request header
    let user = get_user_from_token("<token>");
    match user {
        Ok(userdata) => Ok(Json(userdata)),
        Err(_) => Err(Unauthorized(Some("Unauthorized"))),
    }
}

#[post("/userdata", data = "<data>")]
fn post_userdata(data: Json<UserSettings>) -> (Status, (ContentType, &'static str)) {
    println!("* USERDATA: {:?}", data);
    (Status::Created, (ContentType::Plain, "Created"))
}

#[get("/machineconfig/<machine_id>")]
fn get_machineconfig(
    machine_id: String,
) -> Result<Json<MachineConfigResponse>, Unauthorized<&'static str>> {
    let response = MachineConfigResponse {
        seserial: machine_id,
        config: MachineConfig {
            updatelocation: "http://someserver.com/<uuid4>/".to_owned(),
        },
    };

    Ok(Json(response))
}

#[get("/favorite")]
fn get_recipe_favorite_ids() -> Result<Json<RecipeIds>, Unauthorized<&'static str>> {
    let response = RecipeIds { ids: vec![] };

    Ok(Json(response))
}

#[put("/favorite/<_recipe_id>")]
fn add_recipe_to_favorites(_recipe_id: u32) -> NoContent {
    NoContent
}

#[delete("/favorite/<_recipe_id>")]
fn remove_recipe_from_favorites(_recipe_id: u32) -> NoContent {
    NoContent
}

#[get("/recipe/all")]
async fn get_recipe_all(db: db::DbConn) -> Result<Json<Vec<Recipe>>, Unauthorized<&'static str>> {
    // TODO: Check X-Recipe-Type header
    let result: Result<Vec<String>, diesel::result::Error> = db.run(move |conn| {
        recipes::table
            .select(recipes::json_data)
            .load(conn)
    }).await;

    let recipes = match result {
        Ok(recipe_strs) => {
            recipe_strs.iter()
                .map(|s| {
                    serde_json::from_str(&s).unwrap()
                })
                .collect()
        },
        Err(_) => {
            return Err(Unauthorized(Some("Crap...")));
        }
    };

    Ok(Json(recipes))
}

#[get("/recipe/ids")]
async fn get_recipe_ids(db: db::DbConn) -> Result<Json<RecipeIds>, Unauthorized<&'static str>> {
    let result: Result<Vec<i32>, diesel::result::Error> = db.run(move |conn| {
        recipes::table
            .select(recipes::id)
            .load(conn)
    }).await;

    let recipe_ids = match result {
        Ok(recipe_ids) => {
            RecipeIds { ids: recipe_ids }
        },
        Err(_) => {
            return Err(Unauthorized(Some("Crap...")));
        }
    };

    Ok(Json(recipe_ids))
}

#[get("/recipe/<_id>")]
fn get_recipe_single_by_id(_id: u32) -> Result<Json<Recipe>, Unauthorized<&'static str>> {
    Err(Unauthorized(Some("Unauthorized")))
}

#[post("/event", data = "<event>")]
fn telemetry_event(event: Json<Event>) -> (Status, (ContentType, &'static str)) {
    println!("* EVENT: {:?}", event);
    (Status::Created, (ContentType::Plain, "Created"))
}

#[post("/machine", data = "<telemetry>")]
fn machine_telemetry(telemetry: String) -> NoContent {
    println!("* TELEMETRY: {}", telemetry);
    NoContent
}

#[post("/privacyterms", data = "<privacy_terms>")]
fn submit_privacy_terms(privacy_terms: String) -> (Status, (ContentType, &'static str)) {
    println!("* PRIVACY TERMS: {}", &privacy_terms);
    (Status::Created, (ContentType::Plain, "Created"))
}

#[get("/privacyterms")]
fn get_privacy_terms() -> (Status, (ContentType, &'static str)) {
    (
        Status::Ok,
        (
            ContentType::JSON,
            r#"{"terms":{"data":false,"version":2,"newsletter":false}}"#,
        ),
    )
}

pub(crate) fn routes() -> Vec<Route> {
    routes![
        register,
        authenticate,
        get_userdata,
        post_userdata,
        get_machineconfig,
        get_recipe_favorite_ids,
        get_recipe_all,
        get_recipe_ids,
        get_recipe_single_by_id,
        add_recipe_to_favorites,
        remove_recipe_from_favorites,
        telemetry_event,
        machine_telemetry,
        submit_privacy_terms,
        get_privacy_terms,
    ]
}
