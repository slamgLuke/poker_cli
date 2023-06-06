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

    pub fn hand(&self, game: &Game) -> Hand {
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

    pub fn bet(&mut self, amount: u32) {
        self.balance -= amount;
        self.bet += amount;
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

    fn advance(&mut self) {
        todo!();
        let len = self.players.len();
        self.turn.1 = (self.turn.1 + 1) % len;
        
        let mut last = 0;
        if let Some(l) = self.last {
            last = l;
        } else {
            for i in (0..len).rev() {
                if self.players[i].is_playing && !self.players[i].folded {
                    last = i;
                    break;
                }
            }
        }

        if self.turn.1 == last {
            self.turn.0.next();
        }
    }

    pub fn play_turn(&mut self) {
        use crate::playerinput::*;

        let current_player = &mut self.players[self.turn.1];
        if current_player.folded && !current_player.is_playing {
            self.advance();
            return;
        }
        loop {
            if let Ok(action) = get_action() {
                match action {
                    Action::Check => {
                        if self.bet > 0 {
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
                        self.bet += amount;
                        current_player.bet(amount - current_player.bet);
                        self.last = Some(self.turn.1);
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
                        current_player.bet(self.bet - current_player.bet);
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
        self.advance();
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
