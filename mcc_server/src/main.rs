#[macro_use]
extern crate rocket;
pub mod mcc;
pub mod utils;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/mcc/api/v1", mcc::routes())
}
