use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

#[derive(Deserialize, Serialize, Debug, Clone, FromRow)]
pub struct User {
    pub user_id: Option<i64>,
    pub user_email: String,
    pub doc_id: Option<String>,
    pub user_first_name: String,
    pub user_last_name: String,
    pub user_active: bool,
}

impl User {
    pub fn new(user_email: String, user_first_name: String, user_last_name: String) -> Self {
        Self {
            user_id: None,
            user_email,
            doc_id: None,
            user_first_name,
            user_last_name,
            user_active: true,
        }
    }

    pub fn map_row_to_user(row: PgRow) -> Self {
        Self {
            user_id: row.try_get("user_id").unwrap(),
            user_email: row.try_get("user_email").unwrap(),
            doc_id: row.try_get("doc_id").unwrap(),
            user_first_name: row.try_get("user_first_name").unwrap(),
            user_last_name: row.try_get("user_last_name").unwrap(),
            user_active: row.try_get("user_active").unwrap(),
        }
    }

    pub fn empty() -> Self {
        Self {
            user_id: None,
            user_email: "".to_string(),
            doc_id: None,
            user_first_name: "".to_string(),
            user_last_name: "".to_string(),
            user_active: true,
        }
    }

    pub fn create_new_user(req_user: User) -> User {
        let mut user = User::new(
            req_user.user_email,
            req_user.user_first_name,
            req_user.user_last_name,
        );
        if let Some(doc_id) = req_user.doc_id {
            user.doc_id = Some(doc_id);
        }
        user.user_id = None;
        user.user_active = req_user.user_active;
        user
    }
}
