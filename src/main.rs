// main.rs
#[allow(dead_code)]
mod gameset;

#[allow(dead_code)]
mod hands;

#[allow(dead_code)]
mod poker;

#[allow(dead_code)]
mod playerinput;

use crate::gameset::*;

fn main() {
    let john = Player::new(String::from("John"));
    let man = Player::new("Man".into());
    let p3 = Player::new("p3".to_string());
    let players = vec![john, man, p3];

    let game = Game::new(players, 5, true);
    game.print_table();

    for p in game.players.iter() {
        let hand = p.hand(&game);
        println!("{:?}", hand);
    }
}

#[cfg(test)]
mod test;
