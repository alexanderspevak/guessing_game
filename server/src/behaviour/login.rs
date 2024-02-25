use super::ConnectionError;
use crate::player::Player;
use guessing_game::messages::{get_random_id, Message, Password, PlayerId, Streamable};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn handle_login(
    stream: Arc<Mutex<dyn Streamable>>,
    players: Arc<Mutex<HashMap<String, Player>>>,
    password: Password,
    password_secret: &str,
) -> Result<Option<String>, ConnectionError> {
    let mut stream_lock = stream.lock().unwrap();
    if password.password.trim() != password_secret {
        return Err(ConnectionError {
            err: "Invalid login attempt".to_string(),
            player_id: None,
        });
    }

    let mut players_lock = players.lock().unwrap();
    let player_id = get_random_id();
    let player_id_instance = PlayerId {
        player_id: player_id.clone(),
    };

    match stream_lock.write(&Message::PlayerId(player_id_instance)) {
        Ok(_) => {
            let new_player = Player {
                id: player_id.clone(),
                stream: stream.clone(),
                opponent: None,
                question: None,
                role: None,
            };
            players_lock.insert(player_id.clone(), new_player);

            Ok(Some(player_id))
        }
        Err(err) => Err(ConnectionError {
            err: err.to_string(),
            player_id: Some(player_id),
        }),
    }
}
