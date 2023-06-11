// main.rs

use crate::gameset::*;

#[allow(dead_code)]
mod gameset;
#[allow(dead_code)]
mod hands;
#[allow(dead_code)]
mod playerinput;
#[allow(dead_code)]
mod poker;

#[allow(dead_code)]
#[allow(unused_imports)]

fn game_1() {
    let john = Player::new(String::from("John"));
    let man = Player::new("Man".into());
    let p3 = Player::new("p3".to_string());
    let players = vec![john, man, p3];

    let game = Game::new(players, 5, true);
    game.print_table();

    for p in game.players.iter() {
        let hand = p.get_hand(&game);
        println!("{:?}", hand);
    }
}

fn game_2() {
    let mut a = Player::new("A".into());
    let mut b = Player::new("B".into());
    let mut c = Player::new("C".into());
    let mut d = Player::new("D".into());
    let mut e = Player::new("E".into());
    let mut f = Player::new("F".into());

    a.balance = 430;
    b.balance = 295;
    c.balance = 150;
    d.balance = 0;
    e.balance = 600;
    f.balance = 55;

    let players = vec![a, b, c, d, e, f];

    let mut game = Game::new(players, 5, false);
    while !game.ended {
        println!("Pot: {}, bet: {}, round: {:?}", game.pot, game.bet, game.turn);
        game.play_turn();
    }
    game.print_table();
}

fn main() {
    // game_1();
    game_2();
}

#[cfg(test)]
mod test;
