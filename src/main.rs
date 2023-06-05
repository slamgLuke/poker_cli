// main.rs
mod gameset;
#[allow(dead_code)]
mod hands;
#[allow(dead_code)]
mod poker;

use crate::gameset::*;

fn main() {
    let john = Player::new(String::from("John"));
    let man = Player::new("Man".into());
    let p3 = Player::new("p3".to_string());
    let players = vec![john, man, p3];

    let game = Game::new(players, true);
    game.print_table();

    for p in game.players.iter() {
        let hand = p.hand(&game);
        println!("{:?}", hand);
    }
}

#[cfg(test)]
mod test;
