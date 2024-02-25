use guessing_game::logging::{log_info, log_warn};
use guessing_game::messages::{ConnectionType, Role};
use std::io;

pub fn get_connection_type() -> ConnectionType {
    log_info("Please enter connection type. Tcp(t), Socket(s):");
    let mut input = String::new();

    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        match input.as_str().trim() {
            "t" | "T" | "tcp" | "Tcp" | "TCP" => return ConnectionType::Tcp,
            "s" | "S" | "Socket" | "SOCKET" => return ConnectionType::UnixSocket,
            _ => log_warn(
                r#""Invalid connection type. Enter "t"  for tcp or "s" for unix sockets ""#,
            ),
        }
    }
}

pub fn get_role(available_opponents: &[String]) -> Role {
    if available_opponents.is_empty() {
        log_info("There are no available opponents. You can wait until someone challenges you.");
        return Role::GuessingPlayer;
    }

    let decision = get_user_input(
        "Would you like to guess, or ask. Write g for guessing, otherwise press enter:",
    );
    let decision = decision.trim();
    if decision == "g" || decision == "G" {
        log_info("You are now guessing player. Please wait until someone challenges you.");
        return Role::GuessingPlayer;
    }

    Role::AskingPlayer
}

pub fn get_user_input(message_for_user: &str) -> String {
    log_info(message_for_user);
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    let trimmed_input = input.trim();

    String::from(trimmed_input)
}

pub fn get_wants_to_quit() -> bool {
    let input = get_user_input("Write in quit(q) if you want to quit or press enter to continue");
    if input == "q" || input == "quit" || input == "Q" {
        return true;
    }
    false
}

pub fn get_hint() -> Option<String> {
    let hint = get_user_input("You can provide a hint for user:");
    let mut hint_option = None;

    if !hint.is_empty() {
        hint_option = Some(hint);
    }

    hint_option
}

pub fn get_opponent(available_opponents: &[String]) -> String {
    log_info("Here is list of your opponents, please write one:");

    for opponent in available_opponents {
        log_info(opponent);
    }

    loop {
        let selected_opponent = get_user_input("");
        if available_opponents.contains(&selected_opponent) {
            return selected_opponent;
        }

        log_info("Not opponent from the list. Try again:");
    }
}

pub fn get_question(opponent_id: &str) -> String {
    let mut question = String::from("");
    while question.is_empty() {
        question = get_user_input(
            format!(
                "You have selected opponent with id: {}. Please provide a question:",
                opponent_id
            )
            .as_str(),
        )
        .trim()
        .to_owned();
    }
    question
}
