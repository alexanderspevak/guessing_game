use crate::cli::get_user_input;
pub use asking_player::handle_asking_role;
use guessing_game::messages::{Message, Password, PlayerId, Riddle, Streamable};
pub use guessing_player::handle_guessing_role;

mod asking_player;
mod guessing_player;

pub fn login(stream: &mut impl Streamable) -> Result<PlayerId, String> {
    let password = get_user_input("Please provide server password:");
    let password = Password { password };
    stream
        .write(&Message::Password(password))
        .map_err(|e| e.to_string())?;
    let message = stream.read().map_err(|e| e.to_string())?;

    if let Message::PlayerId(player) = message {
        return Ok(player);
    }

    Err("Server Error. Invalid type returned".into())
}
