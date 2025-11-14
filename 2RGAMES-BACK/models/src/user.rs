use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub user_id: Option<String>,
    pub email: String,
}

impl User {
    pub fn new(email: String) -> Self {
        Self {
            user_id: None,
            email,
        }
    }
}
