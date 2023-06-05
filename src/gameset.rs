use rand::seq::SliceRandom;
use crate::poker::*;

pub struct Player {
    pub name: String,
    pub hole: Vec<Card>,
    pub balance: u32,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            hole: Vec::new(),
            balance: 0,
        }
    }

    pub fn calculate_hand(&self, game: &Game) -> Hand {
        let mut cards = self
            .hole
            .iter()
            .chain(game.table.iter())
            .collect::<Vec<_>>();
        // sort cards by rank
        cards.sort_by(|a, b| b.compare(a));

        println!("{}'s cards: {:?}", self.name, cards);
        return Hand::HighCard(
            cards[0].rank,
            cards[1].rank,
            cards[2].rank,
            cards[3].rank,
            cards[4].rank,
        );
    }
}

pub struct Game {
    pub players: Vec<Player>,
    pub table: Vec<Card>,
    pub deck: Vec<Card>,
    pub pot: u32,
    pub turn: (Round, usize),
    pub bet: u32,
}

impl Game {
    pub fn new(mut players: Vec<Player>, is_first_game: bool) -> Game {
        if is_first_game {
            let mut rng = rand::thread_rng();
            players.shuffle(&mut rng);
        }

        let mut new_game = Game {
            players,
            table: Vec::new(),
            deck: Vec::new(),
            pot: 0,
            turn: (Round::PreFlop, 0),
            bet: 0,
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

    fn winner(&self) -> &Player {
        &self.players[0]
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
