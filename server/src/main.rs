use guessing_game::logging::{self, log_error};
use http::handle_http_request;
use password::get_password;
use player::Player;
use std::os::unix::net::UnixListener;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, net::TcpListener, thread};
use tcp::handle_tcp_client;
use unix_socket::handle_unix_socket_client;

mod behaviour;
mod http;
mod password;
mod player;
mod tcp;
mod unix_socket;

fn main() {
    let password = get_password();
    let players: HashMap<String, Player> = HashMap::new();
    let player = Arc::new(Mutex::new(players));
    let player_clone = Arc::clone(&player);
    let password = Arc::new(password);
    let password_clone = password.clone();

    let tcp_handle = thread::spawn(move || {
        let listener = TcpListener::bind("0.0.0.0:9000").unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    logging::log_info(format!("New connection: {}", stream.peer_addr().unwrap()));
                    let game_status_clone_inner = player_clone.clone();
                    let password_clone_inner = password_clone.clone();
                    thread::spawn(move || {
                        handle_tcp_client(stream, game_status_clone_inner, password_clone_inner)
                    });
                }
                Err(e) => {
                    log_error(e);
                }
            }
        }
        drop(listener);
    });

    let player_clone = Arc::clone(&player);
    let http_handle = thread::spawn(move || {
        let listener = TcpListener::bind("0.0.0.0:3000").unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    logging::log_info(format!("New connection: {}", stream.peer_addr().unwrap()));
                    let game_status_clone_inner = player_clone.clone();
                    thread::spawn(move || handle_http_request(stream, game_status_clone_inner));
                }
                Err(e) => {
                    log_error(e);
                }
            }
        }
        drop(listener);
    });

    let player_clone = Arc::clone(&player);
    let password_clone = password.clone();
    let socket_path = "/tmp/guessing_game";
    let _ = std::fs::remove_file(socket_path);

    let unix_socket_handle = thread::spawn(move || {
        let listener = UnixListener::bind(socket_path).expect("Failed to bind UnixListener");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    logging::log_info(format!(
                        "New unix connection: {:?}",
                        stream.peer_addr().unwrap()
                    ));
                    let game_status_clone_inner = player_clone.clone();
                    let password_clone_inner = password_clone.clone();
                    thread::spawn(move || {
                        handle_unix_socket_client(
                            stream,
                            game_status_clone_inner,
                            password_clone_inner,
                        )
                    });
                }
                Err(e) => {
                    log_error(e);
                }
            }
        }
        drop(listener);
    });

    let _ = tcp_handle.join();
    let _ = unix_socket_handle.join();
    let _ = http_handle.join();
}
