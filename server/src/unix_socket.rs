use crate::behaviour::handle_game_client;
use crate::behaviour::remove_player;
use crate::player::Player;
use guessing_game::logging::log_error;
use guessing_game::messages::Streamable;
use guessing_game::messages::UnixMessageStream;
use std::collections::HashMap;
use std::os::unix::net::UnixStream;
use std::sync::{Arc, Mutex};

pub fn handle_unix_socket_client(
    stream: UnixStream,
    players: Arc<Mutex<HashMap<String, Player>>>,
    password_secret: Arc<String>,
) {
    stream
        .set_nonblocking(true)
        .expect("Set non blocking failed");

    let mut unix_stream = UnixMessageStream { stream };
    if let Err(err) = unix_stream.write(&guessing_game::messages::Message::GameStart) {
        log_error(err);
        if let Err(e) = unix_stream.shutdown() {
            log_error(e)
        }
        return;
    }

    if let Err(err) = handle_game_client(unix_stream, players.clone(), &password_secret) {
        if err.player_id.is_some() {
            remove_player(players, &err.player_id.unwrap())
        }
        log_error(err.err);
    };
}
