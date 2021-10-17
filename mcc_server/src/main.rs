#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod admin;
pub mod db;
pub mod guard;
pub mod mcc;
pub mod schema;
pub mod utils;
pub mod frontend;
pub mod images;

use std::{sync::mpsc, thread};

use rocket::{serde::Deserialize, Build, Rocket, fairing::AdHoc, fs::FileServer};


async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    // This macro from `diesel_migrations` defines an `embedded_migrations`
    // module containing a function named `run`. This allows the example to be
    // run and tested without any outside setup of the database.
    embed_migrations!();

    let conn = db::DbConn::get_one(&rocket).await.expect("database connection failed");
    conn.run(|c| embedded_migrations::run(c)).await.expect("can't run migrations");

    rocket
}

/// Serves as communication between the image upate worker and `/admin/recipes/update_images` endpoint
pub(crate) struct ImagesUpdate(pub mpsc::SyncSender<Vec<i32>>);

/// Background worker/thread that awaits signal from `/admin/recipes/update_images` endpoint
/// to start updating / downloading recipe images
async fn spawn_image_import_worker(rocket: Rocket<Build>) -> Rocket<Build> {
    let (tx, rx) = mpsc::sync_channel(1);

    let conn = db::DbConn::get_one(&rocket).await.expect("database connection failed");
    thread::spawn(move || {
        loop {
            let trigger = rx.recv();

        }
    });
    rocket.manage(ImagesUpdate(tx))
}

#[derive(Deserialize, Debug)]
pub(crate) struct MccConfig {
    pub default_language: String,
    pub save_beta_recipes_as_live: bool,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::DbConn::fairing())
        .attach(AdHoc::on_ignite("Run Migrations", run_migrations))
        .attach(AdHoc::on_ignite("Spawn import worker", spawn_image_import_worker))
        .mount("/", frontend::routes())
        .mount("/mcc/api/v1", mcc::routes())
        .mount("/admin", admin::routes())
        .mount("/recipe_images", FileServer::from("recipe_images"))
        .attach(AdHoc::config::<MccConfig>())
}
