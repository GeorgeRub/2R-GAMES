use aws_sdk_dynamodb::types::AttributeValue;
use std::collections::HashMap;
use std::io::Error;

mod assigned_exercise;
mod config;
mod connection_to_db;
pub mod event;
pub mod exercise;
pub mod user;
pub mod user_activity;
mod utils;

static USER_TABLE_NAME: &str = "2R-GAMES-USERS";

trait Entity {
    type Item;
    fn serialize(&self) -> Option<HashMap<String, AttributeValue>>;
    fn deserialize(items: HashMap<String, AttributeValue>) -> Result<Self::Item, Error>;

    fn save(&self) -> Result<Self::Item, Error>;
    
    fn update(&self) -> Result<Self::Item, Error>;
    
    fn delete(&self) -> Result<(), Error>;
}
