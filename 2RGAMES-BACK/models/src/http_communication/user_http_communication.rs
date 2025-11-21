use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostUser{
    pub user_email: String,
    pub user_first_name: String,
    pub user_last_name: String,
}