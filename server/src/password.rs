use std::env;

pub fn get_password() -> String {
    let mut password = String::new();

    let args: Vec<String> = env::args().collect();

    for arg in args.iter() {
        if arg.starts_with("--password=") {
            password = arg.trim_start_matches("--password=").to_string();
            break;
        }
    }

    if password.is_empty() {
        panic!("Password must be provided as --password=<your password here>");
    }

    password
}
