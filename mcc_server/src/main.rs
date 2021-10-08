use rocket::fs::FileServer;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

pub mod admin;
pub mod db;
pub mod guard;
pub mod mcc;
pub mod schema;
pub mod utils;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::stage())
        .mount("/mcc/api/v1", mcc::routes())
        .mount("/admin", admin::routes())
        .mount("/static", FileServer::from("static"))
        .mount("/recipe_images", FileServer::from("recipe_images"))
}
