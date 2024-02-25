mod communication;
mod constants;
mod helpers;
mod message_error;
mod messages;
mod traits;

use constants::HEADERS_LEN;
use helpers::split_u16;

pub use communication::tcp::TcpMessageStream;
pub use communication::unix_socket::UnixMessageStream;
pub use constants::MESSAGE_PREFIX;
pub use helpers::get_random_id;
pub use message_error::MessageError;
pub use messages::{Evaluation, OpponentSelected, Password, PlayerId, PlayerList, Riddle};
pub use traits::{Packable, Streamable};

pub enum ConnectionType {
    UnixSocket,
    Tcp,
}

#[derive(PartialEq, Debug)]
pub enum Role {
    AskingPlayer,
    GuessingPlayer,
}

pub enum Message {
    Riddle(Riddle),
    Evaluation(Evaluation),
    OpponentSelected(OpponentSelected),
    Password(Password),
    PlayerList(PlayerList),
    PlayerNotAvailable(String),
    RegisterPlayerRole(Role),
    RequestGuessingPlayers,
    PlayerId(PlayerId),
    Unknown,
    GameStart,
}

pub fn pack(message: &Message) -> Vec<u8> {
    let mut result = Vec::new();
    result.extend_from_slice(MESSAGE_PREFIX.as_bytes());

    match message {
        Message::Riddle(riddle) => {
            let message_body = riddle.pack();
            let raw_message_length = (message_body.len() + 1) as u16;
            let (high, low) = split_u16(raw_message_length);
            result.push(high);
            result.push(low);
            result.push(0);
            result.extend_from_slice(&message_body);
        }

        Message::Evaluation(evaluation) => {
            let message_body = evaluation.pack();
            let raw_message_length = (message_body.len() + 1) as u16;
            let (high, low) = split_u16(raw_message_length);
            result.push(high);
            result.push(low);
            result.push(1);
            result.extend_from_slice(&message_body);
        }

        Message::OpponentSelected(opponent_selected) => {
            let message_body = opponent_selected.pack();
            let raw_message_length = (message_body.len() + 1) as u16;
            let (high, low) = split_u16(raw_message_length);
            result.push(high);
            result.push(low);
            result.push(2);
            result.extend_from_slice(&message_body);
        }
        Message::Password(password) => {
            let message_body = password.pack();
            let raw_message_length = (message_body.len() + 1) as u16;
            let (high, low) = split_u16(raw_message_length);
            result.push(high);
            result.push(low);
            result.push(3);
            result.extend_from_slice(&message_body);
        }
        Message::PlayerList(player_list) => {
            let message_body = player_list.pack();
            let raw_message_length = (message_body.len() + 1) as u16;
            let (high, low) = split_u16(raw_message_length);
            result.push(high);
            result.push(low);
            result.push(4);
            result.extend_from_slice(&message_body);
        }

        Message::PlayerNotAvailable(id) => {
            let raw_message_length = (id.as_bytes().len() + 1) as u16;
            let (high, low) = split_u16(raw_message_length);
            result.push(high);
            result.push(low);
            result.push(5);
            result.extend_from_slice(id.as_bytes());
        }
        Message::RegisterPlayerRole(role) => {
            let role_id: u8 = match role {
                Role::AskingPlayer => 1,
                Role::GuessingPlayer => 0,
            };

            result.push(0);
            result.push(2);
            result.push(6);
            result.push(role_id);
        }
        Message::PlayerId(player_id) => {
            let message_body = player_id.pack();
            let raw_message_length = (message_body.len() + 1) as u16;
            let (high, low) = split_u16(raw_message_length);
            result.push(high);
            result.push(low);
            result.push(7);
            result.extend_from_slice(&message_body);
        }
        Message::RequestGuessingPlayers => {
            result.push(0);
            result.push(1);
            result.push(8);
        }
        Message::GameStart => {
            result.push(0);
            result.push(1);
            result.push(9);
        }
        Message::Unknown => {
            result.push(0);
            result.push(1);
            result.push(10);
        }
    };

    result
}

pub fn unpack(message: &[u8]) -> Result<Message, MessageError> {
    unpack_without_headers(&message[HEADERS_LEN..])
}

pub fn unpack_without_headers(message: &[u8]) -> Result<Message, MessageError> {
    match message[0] {
        0 => {
            let mut riddle = Riddle::default();
            riddle.unpack(&message[1..])?;
            Ok(Message::Riddle(riddle))
        }
        1 => {
            let mut evaluation = Evaluation::default();
            evaluation.unpack(&message[1..])?;

            Ok(Message::Evaluation(evaluation))
        }
        2 => {
            let mut opponent = OpponentSelected::default();
            opponent.unpack(&message[1..])?;

            Ok(Message::OpponentSelected(opponent))
        }
        3 => {
            let mut password = Password::default();
            password.unpack(&message[1..])?;

            Ok(Message::Password(password))
        }
        4 => {
            let mut player_list = PlayerList::default();
            player_list.unpack(&message[1..])?;

            Ok(Message::PlayerList(player_list))
        }
        5 => {
            let disconnected_id = String::from_utf8(message[1..].to_vec()).map_err(|_| {
                MessageError::BadUnpack("Could not convert bytes to disconnected player id")
            })?;

            Ok(Message::PlayerNotAvailable(disconnected_id))
        }
        6 => {
            let message_type = message[1];

            let mut role = Role::AskingPlayer;
            if message_type == 0 {
                role = Role::GuessingPlayer;
            }

            Ok(Message::RegisterPlayerRole(role))
        }
        7 => {
            let mut player_id = PlayerId::default();
            player_id.unpack(&message[1..])?;

            Ok(Message::PlayerId(player_id))
        }
        8 => Ok(Message::RequestGuessingPlayers),
        9 => Ok(Message::GameStart),

        _ => Ok(Message::Unknown),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use helpers::get_random_id;

    #[test]
    fn test_pack_unpack_riddle() {
        let riddle = Riddle {
            sender: get_random_id(),
            asking_player: get_random_id(),
            guessing_player: get_random_id(),
            message: String::from(""),
            hint: None,
        };

        let bytes = pack(&Message::Riddle(riddle));
        let message = unpack(&bytes).unwrap();

        if let Message::Riddle(_) = message {
        } else {
            panic!("message should be riddle");
        }
    }
    #[test]
    fn test_pack_unpack_evaluation() {
        let riddle = Evaluation::default();
        let bytes = pack(&Message::Evaluation(riddle));
        let message = unpack(&bytes).unwrap();

        if let Message::Evaluation(_) = message {
        } else {
            panic!("message should be evaluation");
        }
    }

    #[test]
    fn test_pack_unpack_opponent_selected() {
        let opponent_selected = OpponentSelected {
            guessing_player: get_random_id(),
            asking_player: get_random_id(),
        };

        let bytes = pack(&Message::OpponentSelected(opponent_selected));
        let message = unpack(&bytes).unwrap();

        if let Message::OpponentSelected(_) = message {
        } else {
            panic!("message should be opponentSelected");
        }
    }

    #[test]
    fn test_pack_unpack_player_list() {
        let player_list = PlayerList::default();
        let bytes = pack(&Message::PlayerList(player_list));
        let message = unpack(&bytes).unwrap();

        if let Message::PlayerList(_) = message {
        } else {
            panic!("message should be player list");
        }
    }

    #[test]
    fn test_pack_unpack_disconnected_player() {
        let player = get_random_id();
        let bytes = pack(&Message::PlayerNotAvailable(player.clone()));
        let message = unpack(&bytes).unwrap();

        if let Message::PlayerNotAvailable(player_unwrapped) = message {
            assert_eq!(player, player_unwrapped);
        } else {
            panic!("message should be player list");
        }
    }

    #[test]
    fn test_pack_unpack_register_player_guessing_role() {
        let bytes = pack(&Message::RegisterPlayerRole(Role::GuessingPlayer));
        let message = unpack(&bytes).unwrap();

        if let Message::RegisterPlayerRole(role) = message {
            match role {
                Role::AskingPlayer => panic!("Role should be GuessingPlayer"),
                Role::GuessingPlayer => {}
            }
        } else {
            panic!("message should be register asking_player");
        }
    }

    #[test]
    fn test_pack_unpack_register_player_asking_role() {
        let bytes = pack(&Message::RegisterPlayerRole(Role::AskingPlayer));
        let message = unpack(&bytes).unwrap();

        if let Message::RegisterPlayerRole(role) = message {
            match role {
                Role::AskingPlayer => {}
                Role::GuessingPlayer => {
                    panic!("Role should be AskinPlayer")
                }
            }
        } else {
            panic!("message should be register asking_player");
        }
    }

    #[test]
    fn test_pack_unpack_player_id() {
        let random_id = get_random_id();
        let player = PlayerId {
            player_id: random_id.clone(),
        };
        let bytes = pack(&Message::PlayerId(player));
        let message = unpack(&bytes).unwrap();

        if let Message::PlayerId(player_id) = message {
            assert_eq!(random_id, player_id.player_id);
        } else {
            panic!("message should be PlayerId");
        }
    }

    #[test]
    fn test_pack_unpack_request_ids() {
        let bytes = pack(&Message::RequestGuessingPlayers);
        let message = unpack(&bytes).unwrap();

        if let Message::RequestGuessingPlayers = message {
        } else {
            panic!("message should be request ids");
        }
    }

    #[test]
    fn test_pack_unpack_game_start() {
        let bytes = pack(&Message::GameStart);
        let message = unpack(&bytes).unwrap();

        if let Message::GameStart = message {
        } else {
            panic!("message should be unknown");
        }
    }
    #[test]
    fn test_pack_unpack_unknown() {
        let bytes = pack(&Message::Unknown);
        let message = unpack(&bytes).unwrap();

        if let Message::Unknown = message {
        } else {
            panic!("message should be unknown");
        }
    }

    #[test]
    fn test_pack_unpack_unknown_different_sign() {
        let mut bytes = pack(&Message::Unknown);
        let last_idx = bytes.len() - 1;
        let num = bytes.get_mut(last_idx).unwrap();
        *num = 145;
        let message = unpack(&bytes).unwrap();

        if let Message::Unknown = message {
        } else {
            panic!("message should be Unknown for not recognized number");
        }
    }
}
