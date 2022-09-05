use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct UserResult {
    username: String,
}
pub fn create_post() -> UserResult {
    let user = UserResult {
        username: String::from("Joao"),
    };
    return user;
}
