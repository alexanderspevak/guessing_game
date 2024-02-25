mod evaluation;
mod login;
mod opponent_selected;
mod player_list;
mod riddle;

use crate::player::Player;
use evaluation::handle_evaluation;
use guessing_game::logging::{log_error, log_warn};
use guessing_game::messages::{
    ConnectionType, Message, MessageError, PlayerList, Role, Streamable,
};
use login::handle_login;
use opponent_selected::handle_opponent_selected;
use riddle::handle_riddle;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, thread, time};

pub struct ConnectionError {
    pub err: String,
    pub player_id: Option<String>,
}

pub fn remove_player(players: Arc<Mutex<HashMap<String, Player>>>, player_id: &str) {
    let mut players_lock = players.lock().unwrap();
    match players_lock.remove(player_id) {
        Some(removed_player) => {
            if let Some(opponent) = removed_player.opponent {
                let (opponent_id, _) = opponent;
                if let Some(opponent) = players_lock.get_mut(&opponent_id) {
                    opponent.opponent = None;
                    opponent.question = None;

                    if let Err(error) = opponent
                        .stream
                        .lock()
                        .unwrap()
                        .write(&Message::PlayerNotAvailable(player_id.to_owned()))
                    {
                        log_error(error);
                    }
                }
            }
        }

        None => {
            players_lock.iter_mut().for_each(|(_, player)| {
                if let Some((opponent_id, _)) = player.opponent.as_ref() {
                    if opponent_id != player_id {
                        return;
                    }
                    player.opponent = None;
                    player.question = None;
                    let mut stream_lock = player.stream.lock().unwrap();

                    if let Err(err) =
                        stream_lock.write(&Message::PlayerNotAvailable(player_id.to_owned()))
                    {
                        log_error(err);
                    }
                }
            });
        }
    }
}
pub fn handle_game_client(
    stream: impl Streamable + 'static,
    players: Arc<Mutex<HashMap<String, Player>>>,
    password_secret: &str,
) -> Result<(), ConnectionError> {
    let stream: Arc<Mutex<dyn Streamable>> = Arc::new(Mutex::new(stream));
    let mut player_id = None;

    loop {
        let mut stream_lock = stream.lock().unwrap();
        let read_result = stream_lock.read();
        drop(stream_lock);
        match read_result {
            Ok(msg) => match msg {
                Message::Password(password) => {
                    player_id =
                        handle_login(stream.clone(), players.clone(), password, password_secret)?;
                }
                Message::Riddle(riddle) => {
                    handle_riddle(stream.clone(), players.clone(), riddle, &player_id)?;
                }
                Message::Evaluation(evaluation) => {
                    handle_evaluation(players.clone(), evaluation, &player_id)?
                }

                Message::RequestGuessingPlayers => {
                    let mut stream_lock = stream.lock().unwrap();
                    let players = players.lock().unwrap();
                    let free_guessing_player_ids = players
                        .values()
                        .filter_map(|player| {
                            if player.opponent.is_none()
                                && player
                                    .role
                                    .as_ref()
                                    .is_some_and(|role| role == &Role::GuessingPlayer)
                            {
                                return Some(player.id.clone());
                            }

                            None
                        })
                        .collect::<Vec<String>>();
                    let player_list = PlayerList {
                        opponent_ids: free_guessing_player_ids,
                    };
                    stream_lock
                        .write(&Message::PlayerList(player_list))
                        .map_err(|_| ConnectionError {
                            err: "Can not send player list".into(),
                            player_id: player_id.clone(),
                        })?;
                }

                Message::OpponentSelected(opponent_selected) => handle_opponent_selected(
                    stream.clone(),
                    players.clone(),
                    opponent_selected,
                    &player_id,
                )?,
                Message::PlayerId(_) => {}
                Message::PlayerList(_) => return Ok(()),
                Message::PlayerNotAvailable(player_id) => {
                    remove_player(players.clone(), &player_id);
                    return Ok(());
                }
                Message::GameStart => {}
                Message::Unknown => {}
                Message::RegisterPlayerRole(role) => {
                    let mut players_lock = players.lock().unwrap();
                    if player_id.is_none() {
                        return Err(ConnectionError {
                            err: "Unauthorized action".into(),
                            player_id,
                        });
                    }

                    let player_id = player_id.as_ref().unwrap();
                    match players_lock.get_mut(player_id) {
                        Some(player) => player.role = Some(role),
                        None => {
                            let mut stream_lock = stream.lock().unwrap();
                            log_warn(format!("Unwaranted id {}", player_id));
                            if let Err(err) = stream_lock.shutdown() {
                                log_error(err);
                            };
                            return Err(ConnectionError {
                                err: "Unauthorized action".into(),
                                player_id: Some(player_id.to_owned()),
                            });
                        }
                    }
                }
            },
            Err(MessageError::EmptyRead) => {
                let one_mili = time::Duration::from_millis(1);
                thread::sleep(one_mili);
            }
            Err(MessageError::BadUnpack(e)) => {
                log_error(e);
                let mut stream_lock = stream.lock().unwrap();
                if let Err(e) = stream_lock.shutdown() {
                    log_error(e);
                };

                if let Some(player_id) = player_id.as_ref() {
                    remove_player(players.clone(), player_id);
                }
                return Err(ConnectionError {
                    err: e.to_string(),
                    player_id,
                });
            }
            Err(MessageError::InvalidRead(connection_type)) => {
                match connection_type {
                    ConnectionType::UnixSocket => log_error("Invalid unix socket read"),
                    ConnectionType::Tcp => log_error("Invalid TCP read"),
                }
                let mut stream_lock = stream.lock().unwrap();
                if let Err(err) = stream_lock.write(&Message::Unknown) {
                    log_error(err);
                }
                if let Err(e) = stream_lock.shutdown() {
                    log_error(e);
                };

                if let Some(player_id) = player_id.as_ref() {
                    remove_player(players.clone(), player_id);
                }

                return Err(ConnectionError {
                    err: "Invalid data read".to_string(),
                    player_id,
                });
            }
            Err(err) => {
                if let Some(player_id) = player_id.as_ref() {
                    remove_player(players.clone(), player_id);
                }
                let mut stream_lock = stream.lock().unwrap();
                if let Err(e) = stream_lock.shutdown() {
                    log_error(e);
                };

                return Err(ConnectionError {
                    err: err.to_string(),
                    player_id,
                });
            }
        };
    }
}
