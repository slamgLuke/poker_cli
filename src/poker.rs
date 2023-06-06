// poker.rs
use std::cmp::Ordering;

// Hole: The 2 cards that each player holds
// Table: The 5 community cards that are shared by all players
// Deck: The 52 cards used for each game.
pub const HOLE_SIZE: usize = 2;
pub const TABLE_SIZE: usize = 5;
pub const HAND_SIZE: usize = 5;
pub const DECK_SIZE: usize = 52;

pub const RANKS: [Rank; 13] = [
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
pub const SUITS: [Suit; 4] = [Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
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

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Debug)]
pub enum Round {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}

impl Round {
    pub fn next(&mut self) {
        let new_round = match self {
            Round::PreFlop => Round::Flop,
            Round::Flop => Round::Turn,
            Round::Turn => Round::River,
            Round::River => Round::Showdown,
            Round::Showdown => panic!("Tried to continue when game is over!"),
        };
        *self = new_round;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn compare(&self, other: &Card) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}
