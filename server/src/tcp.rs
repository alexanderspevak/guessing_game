use crate::behaviour::handle_game_client;
use crate::player::Player;
use guessing_game::logging::log_error;
use guessing_game::messages::Streamable;
use guessing_game::messages::TcpMessageStream;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, net::TcpStream};

pub fn handle_tcp_client(
    stream: TcpStream,
    players: Arc<Mutex<HashMap<String, Player>>>,
    password_secret: Arc<String>,
) {
    stream
        .set_nonblocking(true)
        .expect("Set non blocking for tcp stream failed");

    let mut tcp_stream = TcpMessageStream { stream };
    if let Err(err) = tcp_stream.write(&guessing_game::messages::Message::GameStart) {
        log_error(err);
        return;
    }

    if let Err(err) = handle_game_client(tcp_stream, players.clone(), &password_secret) {
        log_error(err.err);
    };
}
