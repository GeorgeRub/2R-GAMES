use crate::event::Event;
use crate::exercise::Exercise;

pub struct UserActivity {
    pub email: String,
    pub exercises: Vec<Exercise>,
    pub events: Vec<Event>
}