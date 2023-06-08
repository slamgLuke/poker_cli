// gameset.rs
use crate::hands::*;
use crate::poker::*;
use rand::seq::SliceRandom;

const INITIAL_BALANCE: u32 = 500;

pub struct Player {
    pub name: String,
    pub hole: Vec<Card>,
    pub bet: u32,
    pub is_playing: bool,
    pub folded: bool,
    pub balance: u32,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            hole: Vec::new(),
            bet: 0,
            is_playing: true,
            folded: false,
            balance: INITIAL_BALANCE,
        }
    }

    pub fn get_hand(&self, game: &Game) -> Hand {
        let cards: Vec<Card> = self
            .hole
            .iter()
            .chain(game.table.iter())
            .copied()
            .collect::<Vec<_>>();
        calculate_hand(cards.as_slice())
    }

    pub fn reset(&mut self) {
        self.hole = Vec::new();
        self.bet = 0;
        self.folded = false;
    }
}

pub struct Game {
    pub players: Vec<Player>,
    pub table: Vec<Card>,
    pub deck: Vec<Card>,
    pub turn: (Round, usize),
    pub pot: u32,
    pub bet: u32,
    last: Option<usize>,
    looped: bool,
}

impl Game {
    pub fn new(mut players: Vec<Player>, min_bet: u32, is_first_game: bool) -> Game {
        if is_first_game {
            let mut rng = rand::thread_rng();
            players.shuffle(&mut rng);
        }
        for player in players.iter_mut() {
            if player.balance < min_bet * 2 {
                player.is_playing = false;
            }
        }
        let mut new_game = Game {
            players,
            table: Vec::new(),
            deck: Vec::new(),
            turn: (Round::PreFlop, 0),
            pot: 0,
            bet: min_bet * 2,
            last: None,
            looped: false,
        };
        new_game.setup_deck();
        new_game.deal();
        new_game
    }

    fn setup_deck(&mut self) {
        self.deck = Vec::new();

        for rank in RANKS.iter() {
            for suit in SUITS.iter() {
                self.deck.push(Card {
                    rank: *rank,
                    suit: *suit,
                });
            }
        }

        let mut rng = rand::thread_rng();
        self.deck.shuffle(&mut rng);
    }

    fn deal(&mut self) {
        for player in self.players.iter_mut() {
            for _ in 0..HOLE_SIZE {
                player.hole.push(self.deck.pop().unwrap());
            }
        }
        for _ in 0..TABLE_SIZE {
            self.table.push(self.deck.pop().unwrap());
        }
    }

    fn get_winner(&mut self) -> Vec<&Player> {
        let mut remaining_players = self
            .players
            .iter()
            .filter(|p| p.is_playing && !p.folded)
            .collect::<Vec<_>>();

        if remaining_players.len() == 1 {
            return remaining_players;
        }

        remaining_players.sort_by(|a, b| b.get_hand(self).cmp(&a.get_hand(self)));
        let winner_hand = remaining_players[0].get_hand(self);

        let mut winners: Vec<&Player> = Vec::new();
        for player in remaining_players.iter() {
            if player.get_hand(self) == winner_hand {
                winners.push(*player);
            } else {
                break;
            }
        }
        winners
    }

    fn advance(&mut self) {
        // advance
        let len = self.players.len();
        self.turn.1 = (self.turn.1 + 1) % len;
        // find last player
        let mut last_index: i32 = -1;
        if let Some(player) = self.last {
            last_index = player as i32;
        } else {
            for i in (0..len).rev() {
                if self.players[i].is_playing && !self.players[i].folded {
                    last_index = i as i32;
                    break;
                }
            }
        }
        // No playing players left
        if last_index == -1 {
            panic!("No players left!");
        } else if self.turn.1 == last_index as usize {
            // Round passes
            if self.looped {
                self.bet = 0;
                self.looped = false;
                self.turn.0.next();
                // Check if (self.turn.0 == Round::Showdown), execute showdown function
                if self.turn.0 == Round::Showdown {
                    self.end_game(self.get_winner().as_slice());
                    return;
                }
                // Check if there is more than one player playing, if not, end game and declare winner
                let mut playing_players = 0;
                for player in self.players.iter() {
                    if player.is_playing && !player.folded {
                        playing_players += 1;
                    }
                }
                if playing_players == 1 {
                    self.end_game(self.get_winner().as_slice());
                    return;
                }
            }
            self.looped = true;
        }
    }

    pub fn play_turn(&mut self) {
        use crate::playerinput::*;

        let current_player = &mut self.players[self.turn.1];
        if current_player.folded || !current_player.is_playing {
            println!("{} is out of the game! Turn skipped", current_player.name);
            self.advance();
            return;
        }
        // Player main loop
        println!("{}'s turn", current_player.name);
        loop {
            if let Ok(action) = get_action() {
                match action {
                    Action::Check => {
                        if current_player.bet == self.bet {
                            println!("Can't check! Current bet is {}$", self.bet);
                            continue;
                        }
                        break;
                    }
                    Action::Raise(amount) => {
                        if amount <= self.bet {
                            println!("Must raise higher than the current bet! {}$", self.bet);
                            continue;
                        }
                        if amount - current_player.bet > current_player.balance {
                            println!(
                                "You don't have enough money! {}$ remaining",
                                current_player.balance
                            );
                            continue;
                        }
                        self.bet = amount;
                        self.last = Some(self.turn.1);
                        // Calculate pot and player balance
                        let difference = amount - current_player.bet;
                        current_player.bet = amount;
                        current_player.balance -= difference;
                        self.pot += difference;
                        break;
                    }
                    Action::Call => {
                        if current_player.balance < self.bet - current_player.bet {
                            println!(
                                "You don't have enough money! {}$ remaining",
                                current_player.balance
                            );
                            continue;
                        }
                        // Calculate player balance
                        let difference = self.bet - current_player.bet;
                        current_player.bet = self.bet;
                        current_player.balance -= difference;
                        self.pot += difference;
                        break;
                    }
                    Action::Fold => {
                        current_player.folded = true;
                        break;
                    }
                }
            } else {
                println!("Invalid action!");
            }
        }
        // TODO: Check if player raised all-in
        self.advance();
    }

    fn end_game(&mut self, winners: &[&Player]) {
        println!("Game ended!");
        for winner in winners {
            println!("{} won {}$", winner.name, self.pot / winners.len() as u32);
            self.players
                .iter_mut()
                .find(|p| p.name == winner.name)
                .unwrap()
                .balance += self.pot / winners.len() as u32;
        }
    }

    pub fn print_table(&self) {
        match self.turn {
            (Round::PreFlop, p) => println!("PreFlop: {}'s turn", self.players[p].name),
            (Round::Flop, p) => println!("Flop: {}'s turn", self.players[p].name),
            (Round::Turn, p) => println!("Turn: {}'s turn", self.players[p].name),
            (Round::River, p) => println!("River: {}'s turn", self.players[p].name),
            (Round::Showdown, _) => println!("Showdown"),
        }

        println!("Table:");
        for card in self.table.iter() {
            println!("{:?} of {:?}", card.rank, card.suit);
        }
        println!("====================");

        for (i, player) in self.players.iter().enumerate() {
            println!("{})", i);
            println!("{}'s hand:", player.name);
            for card in player.hole.iter() {
                println!("{:?} of {:?}", card.rank, card.suit);
            }
        }
        println!();
    }
}
