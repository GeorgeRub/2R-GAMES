use crate::utils::{
    get_bool_from_item_to_option, get_string_from_item, get_string_from_item_to_option,
};
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    // #[serde(rename = "SK")]
    pub user_id: Option<String>,
    // #[serde(rename = "PK")]
    pub user_email: String,
    pub doc_id: Option<String>,
    // #[serde(rename = "SK")]
    pub user_first_name: String,
    // #[serde(rename = "SK")]
    pub user_last_name: String,
    pub user_active: Option<bool>,
}

impl User {
    pub fn new(user_email: String, user_first_name: String, user_last_name: String) -> Self {
        Self {
            user_id: Some(Uuid::now_v7().to_string()),
            user_email,
            doc_id: None,
            user_first_name,
            user_last_name,
            user_active: Some(true),
        }
    }

    pub fn empty() -> Self {
        Self {
            user_id: None,
            user_email: "".to_string(),
            doc_id: None,
            user_first_name: "".to_string(),
            user_last_name: "".to_string(),
            user_active: None,
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
        if let Some(user_id) = req_user.user_id {
            user.user_id = Some(user_id);
        }

        if let Some(user_active) = req_user.user_active {
            user.user_active = Some(user_active);
        }

        user
    }

    pub fn serialize_user(user: User) -> Option<HashMap<String, AttributeValue>> {
        let mut item = HashMap::new();
        item.insert(
            "user_email".to_string(),
            AttributeValue::S(user.user_email.to_string()),
        );
        item.insert(
            "user_first_name".to_string(),
            AttributeValue::S(user.user_first_name.to_string()),
        );
        item.insert(
            "user_last_name".to_string(),
            AttributeValue::S(user.user_last_name.to_string()),
        );

        if let Some(doc_id) = user.doc_id {
            item.insert("doc_id".to_string(), AttributeValue::S(doc_id));
        }
        if let Some(user_id) = user.user_id {
            item.insert("user_id".to_string(), AttributeValue::S(user_id));
        }
        if let Some(user_active) = user.user_active {
            item.insert("user_active".to_string(), AttributeValue::Bool(user_active));
        }
        Some(item)
    }

    pub fn deserialize_user(values: HashMap<String, AttributeValue>) -> User {
        let mut user = User::empty();
        user.user_email = get_string_from_item("user_email", &values);
        user.user_first_name = get_string_from_item("user_first_name", &values);
        user.user_last_name = get_string_from_item("user_last_name", &values);
        user.doc_id = get_string_from_item_to_option("doc_id", &values);
        user.user_id = get_string_from_item_to_option("user_id", &values);
        user.user_active = get_bool_from_item_to_option("user_active", &values);
        user
    }
}
