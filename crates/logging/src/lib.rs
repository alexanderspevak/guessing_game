use std::fmt::Display;

pub fn log_error(message: impl Display) {
    println!("GAME ERROR: {}", message);
}

pub fn log_info(message: impl Display) {
    println!("GAME INFO: {}", message);
}

pub fn log_warn(message: impl Display) {
    println!("GAME WARNING: {}", message);
}
