use crate::user::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::Error;
use crate::connection_to_db::get_connection_to_db;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Event {
    pub event_date: DateTime<Utc>,
    pub name: String,
    pub doc_id: User,
}

impl Event {
    pub fn new(name: String, doc_id: User) -> Self {
        Self {
            event_date: Utc::now(),
            name,
            doc_id,
        }
    }

    pub async fn get_event_by_user(user: &User) -> Result<Vec<Event>, Error> {
        //TO-DO
        let client = get_connection_to_db().await;
        

        Ok(vec![])
    }
}
