use super::*;
use crate::cli::{get_user_input, get_wants_to_quit};
use guessing_game::logging::{log_info, log_warn};
use guessing_game::messages::{Message, Streamable};

fn send_guess(
    stream: &mut impl Streamable,
    guessing_player: &str,
    asking_player: &str,
) -> Result<(), String> {
    let guess = get_user_input("Provide guess:");

    let guess = Riddle {
        sender: guessing_player.into(),
        asking_player: asking_player.into(),
        guessing_player: guessing_player.into(),
        hint: None,
        message: guess,
    };

    stream
        .write(&Message::Riddle(guess))
        .map_err(|e| e.to_string())?;
    log_info("Waiting for evaluation");
    Ok(())
}

pub fn handle_guessing_role(stream: &mut impl Streamable, player_id: &str) -> Result<(), String> {
    let mut guess_count = 0;
    let mut asking_player_id: Option<String> = None;

    loop {
        match stream.read().map_err(|e| e.to_string())? {
            Message::PlayerNotAvailable(id) => {
                log_warn(format!(
                    "Asking player with id {} has exited before game end",
                    id
                ));
                return Ok(());
            }
            Message::Evaluation(evaluation) => {
                if guess_count == 0 || asking_player_id.is_none() {
                    return Err("Recevied evaluation before guess".into());
                }
                if evaluation.guessed {
                    log_info(format!(
                        "Congratulations, you have won after {} guesses.",
                        guess_count
                    ));
                    return Ok(());
                }

                log_info("You have not guessed.");

                if let Some(hint) = evaluation.hint {
                    log_info(format!("Hint provided by opponent: {}", hint));
                }

                if get_wants_to_quit() {
                    log_info("Goodbye");
                    return Ok(());
                }

                send_guess(stream, player_id, asking_player_id.as_ref().unwrap())?;
                guess_count += 1;
            }
            Message::Riddle(riddle) => {
                if guess_count > 0 {
                    return Err("Received riddle for second time".into());
                }
                log_info(format!(
                    "Player {} provided you with riddle. ",
                    riddle.asking_player
                ));
                asking_player_id = Some(String::from(&riddle.asking_player));
                if let Some(hint) = riddle.hint {
                    log_info(format!("Hint provided by opponent: {}", hint));
                }
                send_guess(stream, player_id, &riddle.asking_player)?;
                guess_count += 1;
            }
            _ => return Ok(()),
        }
    }
}
