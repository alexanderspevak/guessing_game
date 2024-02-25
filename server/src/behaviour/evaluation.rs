use guessing_game::logging::log_warn;
use guessing_game::messages::{Evaluation, Message, Role};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::ConnectionError;
use crate::player::Player;

pub fn handle_evaluation(
    players: Arc<Mutex<HashMap<String, Player>>>,
    evaluation: Evaluation,
    player_id: &Option<String>,
) -> Result<(), ConnectionError> {
    if player_id.is_none() {
        return Err(ConnectionError {
            err: "Received evaluation for question from not registered player".into(),
            player_id: None,
        });
    }
    let id = player_id.as_ref().unwrap();
    let players_lock = players.lock().unwrap();
    let player = players_lock.get(id);

    if player.is_none() {
        return Err(ConnectionError {
            err: "Received evaluation for question from not registered player".into(),
            player_id: None,
        });
    }

    let player = player.unwrap();

    if player.role.is_none() {
        return Err(ConnectionError {
            err: "Received evaluation for question from player without role".into(),
            player_id: None,
        });
    }

    let player_role = player.role.as_ref().unwrap();
    if player_role != &Role::AskingPlayer {
        return Err(ConnectionError {
            err: "Received evaluation for question from player without corresponding role".into(),
            player_id: None,
        });
    }

    if player.opponent.is_none() {
        return Err(ConnectionError {
            err: "Received evaluation for question from player without opponent".into(),
            player_id: None,
        });
    }

    let (opponent_id, _) = player.opponent.as_ref().unwrap();
    let opponent = players_lock.get(opponent_id);

    if opponent.is_none() {
        log_warn("Opponent is disconnected");
        return Ok(());
    }

    let opponent = opponent.unwrap();
    let mut opponent_connection = opponent.stream.lock().unwrap();

    if let Err(err) = opponent_connection.write(&Message::Evaluation(evaluation)) {
        log_warn(err);
    }

    Ok(())
}
