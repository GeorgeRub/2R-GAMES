use crate::utils::{
    get_bool_from_item_to_option, get_string_from_item, get_string_from_item_to_option,
};
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Error;
use uuid::Uuid;
// use crate::connection_to_db::RustPersistenceApplication;
use crate::Entity;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub user_id: Option<String>,
    pub user_email: String,
    pub doc_id: Option<String>,
    pub user_first_name: String,
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

}

impl Entity for User {
    type Item = User;

    fn serialize(&self) -> Option<HashMap<String, AttributeValue>> {
        let mut item = HashMap::new();
        if let Some(user_id) = &self.user_id {
            item.insert("user_id".to_string(), AttributeValue::S(user_id.clone()));
        }
        item.insert(
            "user_email".to_string(),
            AttributeValue::S(self.user_email.clone()),
        );
        item.insert(
            "user_first_name".to_string(),
            AttributeValue::S(self.user_first_name.clone()),
        );
        item.insert(
            "user_last_name".to_string(),
            AttributeValue::S(self.user_last_name.clone()),
        );

        if let Some(doc_id) = &self.doc_id {
            item.insert("doc_id".to_string(), AttributeValue::S(doc_id.clone()));
        }

        if let Some(active) = &self.user_active {
            item.insert(
                "user_active".to_string(),
                AttributeValue::Bool(self.user_active.unwrap()),
            );
        }
        Some(item)
    }

    fn deserialize(items: HashMap<String, AttributeValue>) -> Result<Self::Item, Error> {
        Ok(User {
            user_id: get_string_from_item_to_option("user_id", &items),
            user_email: get_string_from_item("user_email", &items),
            doc_id: get_string_from_item_to_option("doc_id", &items),
            user_first_name: get_string_from_item("user_first_name", &items),
            user_last_name: get_string_from_item("user_last_name", &items),
            user_active: get_bool_from_item_to_option("user_active", &items),
        })
    }

    fn save(&self) -> Result<Self::Item, Error> {
        todo!()
    }

    fn update(&self) -> Result<Self::Item, Error> {
        todo!()
    }

    fn delete(&self) -> Result<(), Error> {
        todo!()
    }
}
