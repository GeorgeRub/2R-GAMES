use aws_sdk_dynamodb::types::AttributeValue;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::Error;
use std::string::ToString;
use std::sync::Mutex;

lazy_static! {
    pub static ref TABLES_NAMES: Mutex<HashMap<String, String>> = {
        let mut m = HashMap::new();
        m.insert("USER_TABLE_NAME".to_string(), "2R-GAMES-USERS".to_string());
        Mutex::new(m)
    };
}

mod assigned_exercise;
mod config;
mod event;
pub mod exercise;
pub mod user;


mod user_activity;
mod utils;
pub mod services;

pub trait Entity {
    type Item;
    fn serialize_object(&self) -> Option<HashMap<String, AttributeValue>>;
    fn deserialize_object(items: HashMap<String, AttributeValue>) -> Result<Self::Item, Error>;

    async fn save(&self) -> Result<Self::Item, Error>;

    fn update(&self) -> Result<Self::Item, Error>;

    fn delete(&self) -> Result<(), Error>;
}
