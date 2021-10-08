use std::str::FromStr;

use monsieurcc::schemas::RecipeType;
/// Guards that filter incoming requests
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

struct AuthKey<'r>(&'r str);

#[derive(Debug)]
enum AuthKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthKey<'r> {
    type Error = AuthKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn is_valid(_key: &str) -> bool {
            // TODO: Verify at some point, when authentication
            // is actually desired
            true
        }

        match req.headers().get_one("X-Auth-Key") {
            None => Outcome::Failure((Status::BadRequest, AuthKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(AuthKey(key)),
            Some(_) => Outcome::Failure((Status::BadRequest, AuthKeyError::Invalid)),
        }
    }
}

struct RecipeTypeParam(RecipeType);

#[derive(Debug)]
enum RecipeTypeError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RecipeTypeParam {
    type Error = RecipeTypeError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("X-Recipe-Type") {
            None => Outcome::Failure((Status::BadRequest, RecipeTypeError::Missing)),
            Some(rtype) => match RecipeType::from_str(rtype) {
                Ok(res) => Outcome::Success(RecipeTypeParam(res)),
                Err(_) => Outcome::Failure((Status::BadRequest, RecipeTypeError::Invalid)),
            },
        }
    }
}
