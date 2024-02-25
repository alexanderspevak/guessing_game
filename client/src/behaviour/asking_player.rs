use super::*;
use crate::cli::{get_hint, get_opponent, get_question};
use guessing_game::logging::{log_info, log_warn};
use guessing_game::messages::{Evaluation, Message, Role, Streamable};

pub enum ResponseResult {
    WrongAnswer,
    Quit,
    RightAnswer,
}

pub fn process_response_from_guessing_player(
    stream: &mut impl Streamable,
) -> Result<ResponseResult, String> {
    let response = stream.read().map_err(|e| e.to_string())?;

    if let Message::Evaluation(evaluation) = response {
        if evaluation.guessed {
            return Ok(ResponseResult::RightAnswer);
        }

        return Ok(ResponseResult::WrongAnswer);
    }

    if let Message::PlayerNotAvailable(_) = response {
        log_warn("Opponent is not available anymore");
        return Ok(ResponseResult::Quit);
    }

    log_warn("Received message not within role");

    Err("Received message not within role".into())
}

pub fn handle_asking_role(
    stream: &mut impl Streamable,
    player_id: &str,
    guessing_players: &[String],
) -> Result<(), String> {
    stream
        .write(&Message::RegisterPlayerRole(Role::AskingPlayer))
        .map_err(|e| e.to_string())?;
    let guessing_player_id = get_opponent(guessing_players);
    log_info("Asking player opponent selected.");
    let question = get_question(&guessing_player_id);
    let hint = get_hint();
    let riddle = Riddle {
        sender: player_id.into(),
        asking_player: player_id.into(),
        guessing_player: guessing_player_id.clone(),
        message: question,
        hint,
    };

    let riddle = Message::Riddle(riddle);
    stream.write(&riddle).map_err(|e| e.to_string())?;
    let mut guesses = 0;

    loop {
        log_info("Please wait for answer from guessing player.");
        guesses += 1;
        match process_response_from_guessing_player(stream)? {
            ResponseResult::WrongAnswer => {
                log_info(format!("User has not guessed. This is {}. try", guesses));
                let hint = get_hint();
                let evaluation = Evaluation {
                    hint,
                    guessed: false,
                };
                stream
                    .write(&Message::Evaluation(evaluation))
                    .map_err(|e| e.to_string())?;
            }
            ResponseResult::RightAnswer => {
                log_info(format!(
                    "User has guessed. You lost.  This is {}. try",
                    guesses
                ));
                let evaluation = Evaluation {
                    hint: None,
                    guessed: true,
                };
                stream
                    .write(&Message::Evaluation(evaluation))
                    .map_err(|e| e.to_string())?;
                return Ok(());
            }
            ResponseResult::Quit => {
                log_info(format!(
                    "User has quit. You wonn.  They made {} tries",
                    guesses
                ));
                return Ok(());
            }
        }
    }
}
