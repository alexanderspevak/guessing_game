use guessing_game::logging::{log_error, log_warn};
use guessing_game::messages::{Message, OpponentSelected, Role, Streamable};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::ConnectionError;
use crate::player::Player;

pub fn handle_opponent_selected(
    stream: Arc<Mutex<dyn Streamable>>,
    players: Arc<Mutex<HashMap<String, Player>>>,
    opponent_selected: OpponentSelected,
    player_id: &Option<String>,
) -> Result<(), ConnectionError> {
    if player_id.is_none() {
        return Err(ConnectionError {
            err: "Unauthorized action".into(),
            player_id: None,
        });
    }

    let player_id = player_id.as_ref().unwrap();
    let players_lock = players.lock().unwrap();
    let opponent = players_lock.get(&opponent_selected.asking_player);

    if opponent.is_none()
        || opponent.is_some_and(|oposing_player: &Player| {
            oposing_player
                .role
                .as_ref()
                .is_some_and(|role| role != &Role::AskingPlayer)
                || oposing_player.opponent.is_some()
        })
    {
        log_warn("Selected opponent is not available");
        let mut stream_lock = stream.lock().unwrap();
        if let Err(err) = stream_lock.write(&Message::PlayerNotAvailable(
            opponent_selected.asking_player,
        )) {
            log_error(err);
        };
        return Ok(());
    }

    let opponent = opponent.unwrap();
    let opponent_selected = OpponentSelected {
        guessing_player: player_id.into(),
        asking_player: opponent.id.clone(),
    };

    let mut opponent_stream_lock = opponent.stream.lock().unwrap();
    if let Err(err) = opponent_stream_lock.write(&Message::OpponentSelected(opponent_selected)) {
        log_error(err)
    }

    Ok(())
}
