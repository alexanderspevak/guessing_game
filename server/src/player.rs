use guessing_game::messages::Role;
use guessing_game::messages::Streamable;
use std::sync::{Arc, Mutex};

pub struct Player {
    pub id: String,
    pub stream: Arc<Mutex<dyn Streamable>>,
    pub opponent: Option<(String, usize)>,
    pub question: Option<String>,
    pub role: Option<Role>,
}
