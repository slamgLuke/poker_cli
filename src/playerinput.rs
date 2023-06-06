// playerinput.rs

use std::io;

pub enum Action {
    Check,
    Raise(u32),
    Call,
    Fold,
}

pub fn get_action() -> Result<Action, Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    buffer = buffer.trim().to_lowercase();
    match buffer.as_str() {
        "check" => Ok(Action::Check),
        "call" => Ok(Action::Call),
        "fold" => Ok(Action::Fold),
        "raise" => {
            println!("How much?");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)?;
            let amount = buffer.trim().parse::<u32>()?;
            Ok(Action::Raise(amount))
        }
        _ => Err("Invalid action".into()),
    }
}


