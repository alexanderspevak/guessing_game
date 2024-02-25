use guessing_game::messages::Role;

use crate::player::Player;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, io::prelude::*, net::TcpStream};

pub fn handle_http_request(
    mut stream: TcpStream,
    players: Arc<Mutex<HashMap<String, Player>>>,
) -> Result<(), &'static str> {
    let status_line = "HTTP/1.1 200 OK";
    let mut html = r##"
            <!doctype html>
            <html lang="en">
              <head>
                <meta charset="utf-8" />
                <title>Guessing game</title>
              </head>
              <body>
              <table>
              <thead>
                <tr>
                    <th>Asking Player</th>
                    <th>Guessing Player</th>
                    <th>Guesses</th>
                </tr>
             </thead>
             <tbody>
        "##
    .to_string();
    let players_lock = players.lock().unwrap();
    players_lock.values().for_each(|player| {
        if player.opponent.is_some()
            && player
                .role
                .as_ref()
                .is_some_and(|role| role == &Role::AskingPlayer)
        {
            let (opponent_id, guesses) = player.opponent.as_ref().unwrap();
            let table_row = format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
                &player.id, opponent_id, guesses
            );

            html.push_str(&table_row);
        }
    });

    let closing = r##"
        </tbody>
        </table>
         </body>
         </html>
    "##;
    html.push_str(closing);
    let length = html.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{html}");
    stream
        .write_all(response.as_bytes())
        .map_err(|_| "error writing http request")?;
    Ok(())
}
