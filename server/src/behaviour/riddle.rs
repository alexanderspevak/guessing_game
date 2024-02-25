use guessing_game::logging::log_warn;
use guessing_game::messages::{Evaluation, Message, Riddle, Streamable};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::ConnectionError;
use crate::player::Player;

fn increase_guess_count(
    players: Arc<Mutex<HashMap<String, Player>>>,
    player_id: &str,
) -> Result<(), &'static str> {
    let mut players_lock = players.lock().unwrap();
    let player = players_lock
        .get_mut(player_id)
        .ok_or("Player not registered")?;

    let (opponent, guess_count) = player
        .opponent
        .as_ref()
        .ok_or("Opponent not in players list")?;

    player.opponent = Some((opponent.to_string(), *guess_count + 1));

    Ok(())
}

pub fn handle_riddle(
    stream: Arc<Mutex<dyn Streamable>>,
    players: Arc<Mutex<HashMap<String, Player>>>,
    riddle: Riddle,
    player_id: &Option<String>,
) -> Result<(), ConnectionError> {
    if player_id.is_none() {
        return Err(ConnectionError {
            err: "Received evaluation for question from not registered player".into(),
            player_id: None,
        });
    }
    let id = player_id.as_ref().unwrap();

    if id == &riddle.asking_player {
        let mut players_lock = players.lock().unwrap();
        let guessing_player = players_lock.get(&riddle.guessing_player);
        if guessing_player.is_none() {
            let mut stream_lock = stream.lock().unwrap();
            stream_lock
                .write(&Message::PlayerNotAvailable(riddle.guessing_player))
                .map_err(|err| ConnectionError {
                    err: err.to_string(),
                    player_id: Some(riddle.asking_player),
                })?;
            return Ok(());
        }

        {
            let guessing_player = players_lock.get_mut(&riddle.guessing_player).unwrap();
            let mut guessing_player_stream_lock = guessing_player.stream.lock().unwrap();
            if let Err(err) = guessing_player_stream_lock.write(&Message::Riddle(riddle.clone())) {
                log_warn(err);
                return Ok(());
            }
            guessing_player.opponent = Some((id.into(), 0));
            guessing_player.question = Some(riddle.message.clone())
        }

        let asking_player = players_lock.get_mut(id).unwrap();
        asking_player.opponent = Some((riddle.guessing_player.clone(), 0));
        asking_player.question = Some(riddle.message);

        return Ok(());
    }

    // guessing player

    increase_guess_count(players.clone(), id).map_err(|err| ConnectionError {
        err: err.to_string(),
        player_id: player_id.clone(),
    })?;

    increase_guess_count(players.clone(), &riddle.asking_player).map_err(|err| {
        ConnectionError {
            err: err.to_string(),
            player_id: player_id.clone(),
        }
    })?;

    let mut players_lock = players.lock().unwrap();

    let asking_player = players_lock
        .get_mut(&riddle.asking_player)
        .ok_or(ConnectionError {
            err: "Opponent not registered".into(),
            player_id: player_id.clone(),
        })?;

    let mut asking_player_stream_lock = asking_player.stream.lock().unwrap();

    let question = asking_player.question.as_ref().ok_or(ConnectionError {
        err: "Player is missing question".into(),
        player_id: player_id.clone(),
    })?;

    let evaluation = Evaluation {
        hint: None,
        guessed: question == &riddle.message,
    };

    asking_player_stream_lock
        .write(&Message::Evaluation(evaluation))
        .map_err(|err| ConnectionError {
            err: err.to_string(),
            player_id: player_id.clone(),
        })?;
    Ok(())
}
