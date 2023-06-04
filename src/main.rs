mod poker;

fn main() {
    let john = poker::Player::new(String::from("John"));
    let man = poker::Player::new("Man".into());
    let p3 = poker::Player::new("p3".to_string());
    let players = vec![john, man, p3];


    let game = poker::Game::new(players);
    game.print_table();
}
