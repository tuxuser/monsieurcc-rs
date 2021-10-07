#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

pub mod schema;
pub mod db;
pub mod admin;
pub mod mcc;
pub mod utils;

use diesel::prelude::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(db::stage())
        .mount("/mcc/api/v1", mcc::routes())
        .mount("/admin", admin::routes())
}
