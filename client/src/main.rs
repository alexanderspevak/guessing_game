use behaviour::{handle_asking_role, handle_guessing_role, login};
use cli::get_role;
use guessing_game::logging::{log_error, log_info};
use guessing_game::messages::{
    ConnectionType, Message, Role, Streamable, TcpMessageStream, UnixMessageStream,
};
use std::net::TcpStream;
use std::os::unix::net::UnixStream;

use crate::cli::get_connection_type;

mod behaviour;
mod cli;

fn process_client(mut stream: impl Streamable) {
    match stream.read() {
        Ok(message) => {
            if let Message::GameStart = message {
                log_info("game starts");
            } else {
                log_error("Initial message should be sent from server so game can start");
                return;
            }
        }
        Err(e) => {
            log_error(e);
            return;
        }
    }
    let player_id = login(&mut stream);
    if let Err(e) = player_id {
        log_error(e);
        return;
    }
    let player_id = player_id.unwrap();
    let player_id = player_id.player_id;

    if let Err(e) = stream.write(&Message::RequestGuessingPlayers) {
        log_error(e);
        return;
    };

    let guessing_players = stream.read();

    if let Err(e) = guessing_players {
        log_error(e);
        return;
    }

    let message = guessing_players.unwrap();
    let mut guessing_players = Vec::new();

    if let Message::PlayerList(player_list) = message {
        guessing_players = player_list.opponent_ids;
    } else {
        log_error("Did not receive message about guessing players ids");
    }

    match get_role(&guessing_players) {
        Role::GuessingPlayer => {
            if let Err(err) = stream.write(&Message::RegisterPlayerRole(Role::GuessingPlayer)) {
                log_error(err);
                return;
            };
            log_info("Please wait until player provides you with riddle.");
            if let Err(err) = handle_guessing_role(&mut stream, &player_id) {
                log_error(err);
            }
            let _ = stream.shutdown();
            return;
        }
        Role::AskingPlayer => {
            if let Err(err) = handle_asking_role(&mut stream, &player_id, &guessing_players) {
                log_error(err);
                let _ = stream.shutdown();
                return;
            }
        }
    }

    if let Err(err) = stream.shutdown() {
        log_error(err)
    }
}

fn main() {
    log_info("Welcome to guessing game");
    match get_connection_type() {
        ConnectionType::Tcp => match TcpStream::connect("localhost:9000") {
            Ok(stream) => {
                let stream = TcpMessageStream { stream };
                process_client(stream);
            }
            Err(e) => {
                log_error(e);
            }
        },
        ConnectionType::UnixSocket => match UnixStream::connect("/tmp/guessing_game") {
            Ok(stream) => {
                let stream = UnixMessageStream { stream };
                process_client(stream);
            }
            Err(e) => {
                log_error(e);
            }
        },
    }
}
