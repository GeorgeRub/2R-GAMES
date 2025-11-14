use crate::connection_to_db::get_connection_to_db;
use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Error;
use uuid::Uuid;

static EXERCISE_TABLE_NAME: &str = "Exercise";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Exercise {
    pub exercise_id: String,
    pub description: Option<String>,
    pub exercise_name: String,
    pub key_name: String,
    pub image: Option<String>,
}

impl Exercise {
    pub fn new(exercise_name: String, key_name: String) -> Self {
        Self {
            exercise_id: Uuid::now_v7().to_string(),
            description: None,
            exercise_name,
            key_name,
            image: None,
        }
    }

    fn exercise_to_put_item(&self) -> Option<HashMap<String, AttributeValue>> {
        let mut item = HashMap::new();

        item.insert(
            "exercise_id".to_string(),
            AttributeValue::S(self.exercise_id.clone()),
        );
        item.insert(
            "exercise_name".to_string(),
            AttributeValue::S(self.exercise_name.clone()),
        );
        item.insert(
            "key_name".to_string(),
            AttributeValue::S(self.key_name.clone()),
        );
        if let Some(description) = &self.description {
            if !description.is_empty() {
                item.insert(
                    "description".to_string(),
                    AttributeValue::S(description.clone()),
                );
            }
        }

        if let Some(image) = &self.image {
            if !image.is_empty() {
                item.insert("image".to_string(), AttributeValue::S(image.clone()));
            }
        }

        Some(item)
    }

    fn query_out_to_exercise(&self, item: HashMap<String, AttributeValue>) -> Exercise {
        Exercise {
            exercise_id: item
                .get("exercise_id")
                .and_then(|v| v.as_s().ok())
                .cloned()
                .unwrap_or_default(),
            exercise_name: item
                .get("exercise_name")
                .and_then(|v| v.as_s().ok())
                .cloned()
                .unwrap_or_default(),
            key_name: item
                .get("key_name")
                .and_then(|v| v.as_s().ok())
                .cloned()
                .unwrap_or_default(),
            description: item
                .get("description")
                .and_then(|v| v.as_s().ok())
                .map(|s| s.to_string()),
            image: item
                .get("image")
                .and_then(|v| v.as_s().ok())
                .map(|s| s.to_string()),
        }
    }

    pub async fn add_exercise(&self) -> Result<Exercise, Error> {
        println!("Adding exercise!!!");
        let check_if_exercise_exists = self.get_exercise_by_name().await;
        match check_if_exercise_exists {
            Some(_) => {
                return Err(Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    "Exercise already exists",
                ));
            }
            None => (),
        }
        let client = get_connection_to_db().await;
        let _ = client
            .put_item()
            .table_name(EXERCISE_TABLE_NAME)
            .set_item(self.exercise_to_put_item())
            .send()
            .await
            .expect("TODO: panic message");
        println!("Added exercise!!!");
        Ok(self.clone())
    }

    async fn get_exercise_by_name(&self) -> Option<Exercise> {
        let client = get_connection_to_db().await;
        let response = client
            .scan()
            .table_name(EXERCISE_TABLE_NAME)
            .filter_expression("exercise_name = :name_search and key_name = :key_name")
            .expression_attribute_values(
                ":name_search".to_string(),
                AttributeValue::S(self.exercise_name.clone()),
            )
            .expression_attribute_values(
                ":key_name".to_string(),
                AttributeValue::S(self.key_name.clone()),
            )
            .send()
            .await
            .expect("TODO: panic message");
        println!("Got response from db: {:?}", response);
        match response.items {
            None => None,
            Some(items) => {
                if items.len() == 0 {
                    return None;
                }
                Some(self.query_out_to_exercise(items[0].clone()))
            }
        }
    }
}
