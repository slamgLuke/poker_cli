#![allow(dead_code)]
// poker.rs

use rand::seq::SliceRandom;

// Hand: The 2 cards that each player holds
// Table: The 5 community cards that are shared by all players
// Deck: The 52 cards used for each game.
const HAND_SIZE: usize = 2;
const TABLE_SIZE: usize = 5;
const DECK_SIZE: usize = 52;

const RANKS: [Rank; 13] = [
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
    Rank::Ace,
];
const SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];

#[derive(Copy, Clone, Debug)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Copy, Clone, Debug)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

pub enum Round {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}

#[derive(Copy, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

pub struct Player {
    pub name: String,
    pub hand: [Card; HAND_SIZE],
    pub money: u32,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            hand: [Card {
                rank: Rank::Ace,
                suit: Suit::Spades,
            }; HAND_SIZE],
            money: 0,
        }
    }

    pub fn bet(&mut self, game: &mut Game, amount: u32) {
        self.money -= amount;
        game.pot += amount;
    }
}

pub struct Game {
    pub players: Vec<Player>,
    pub table: [Card; TABLE_SIZE],
    pub deck: Vec<Card>,
    pub pot: u32,
    pub turn: (Round, usize),
}

impl Game {
    pub fn new(players: Vec<Player>) -> Game {
        // initialize table to all Aces of Spades
        let table = [Card {
            rank: Rank::Ace,
            suit: Suit::Spades,
        }; TABLE_SIZE];

        let mut new_game = Game {
            players,
            table,
            deck: Vec::new(),
            pot: 0,
            turn: (Round::PreFlop, 0),
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
            for i in 0..HAND_SIZE {
                player.hand[i] = self.deck.pop().unwrap();
            }
        }
        for i in 0..TABLE_SIZE {
            self.table[i] = self.deck.pop().unwrap();
        }
    }

    pub fn print_table(&self) {
        match self.turn {
            (Round::PreFlop, p) => println!("PreFlop: {}'s turn", p),
            (Round::Flop, p) => println!("Flop: {}'s turn", p),
            (Round::Turn, p) => println!("Turn: {}'s turn", p),
            (Round::River, p) => println!("River: {}'s turn", p),
            (Round::Showdown, _) => println!("Showdown"),
        }

        println!("Table:");
        for card in self.table.iter() {
            println!("{:?} of {:?}", card.rank, card.suit);
        }

        for player in self.players.iter() {
            println!("{}'s hand:", player.name);
            for card in player.hand.iter() {
                println!("{:?} of {:?}", card.rank, card.suit);
            }
        }
        println!();
    }
}
