#[macro_use]
extern crate rocket;
pub mod admin;
pub mod mcc;
pub mod utils;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/mcc/api/v1", mcc::routes())
        .mount("/admin", admin::routes())
}
