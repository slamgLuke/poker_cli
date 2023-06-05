mod poker;
mod gameset;


fn main() {
    use crate::gameset::*;

    let john = Player::new(String::from("John"));
    let man = Player::new("Man".into());
    let p3 = Player::new("p3".to_string());
    let players = vec![john, man, p3];

    let game = Game::new(players, true);
    game.print_table();

    game.players[0].calculate_hand(&game);
}

#[cfg(test)]
mod test;
