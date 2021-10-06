use monsieurcc::schemas::UserData;

pub fn is_registration_valid(_uid: &str, _displayname: &str, _password: &str) -> bool {
    true
}

pub fn get_user() -> UserData {
    UserData {
        uid: "no@example.org".to_owned(),
        displayname: "user42".to_owned(),
        ..Default::default()
    }
}

pub fn get_user_from_credentials(
    _username: &str,
    _password: &str,
) -> Result<UserData, Box<dyn std::error::Error>> {
    Ok(get_user())
}

pub fn get_user_from_token(_token: &str) -> Result<UserData, Box<dyn std::error::Error>> {
    Ok(get_user())
}

pub fn generate_auth_token() -> String {
    "f2f5de852d7aa4bdc3b8efe9a6ecbb002da17864c3805844".to_owned()
}
