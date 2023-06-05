// hands.rs
use crate::poker::*;
use std::cmp::Ordering::{self, *};
use std::collections::HashMap;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum Hand {
    HighCard(Rank, Rank, Rank, Rank, Rank),
    Pair(Rank, Rank, Rank, Rank),
    TwoPair(Rank, Rank, Rank),
    ThreeOfAKind(Rank, Rank, Rank),
    Straight(Rank),
    Flush(Rank),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank, Rank),
    StraightFlush(Rank),
    RoyalFlush,
}

impl Hand {
    pub fn compare(&self, other: &Hand) -> Ordering {
        use Hand::*;
        match (self, other) {
            (RoyalFlush, RoyalFlush) => Equal,
            (StraightFlush(max), StraightFlush(o_max)) => max.cmp(o_max),
            (FourOfAKind(rank, k), FourOfAKind(o_rank, o_k)) => match rank.cmp(o_rank) {
                Greater => Greater,
                Less => Less,
                Equal => k.cmp(o_k),
            },
            (FullHouse(trio, pair), FullHouse(o_trio, o_pair)) => match trio.cmp(o_trio) {
                Greater => Greater,
                Less => Less,
                Equal => pair.cmp(o_pair),
            },
            (Flush(max), Flush(o_max)) => max.cmp(o_max),
            (Straight(max), Straight(o_max)) => max.cmp(o_max),
            (ThreeOfAKind(trio, k1, k2), ThreeOfAKind(o_trio, o_k1, o_k2)) => {
                match trio.cmp(o_trio) {
                    Greater => Greater,
                    Less => Less,
                    Equal => match k1.cmp(o_k1) {
                        Greater => Greater,
                        Less => Less,
                        Equal => k2.cmp(o_k2),
                    },
                }
            }
            (TwoPair(pair1, pair2, k), TwoPair(o_pair1, o_pair2, o_k)) => {
                match pair1.cmp(o_pair1) {
                    Greater => Greater,
                    Less => Less,
                    Equal => match pair2.cmp(o_pair2) {
                        Greater => Greater,
                        Less => Less,
                        Equal => k.cmp(o_k),
                    },
                }
            }
            (Pair(pair, k1, k2, k3), Pair(o_pair, o_k1, o_k2, o_k3)) => match pair.cmp(o_pair) {
                Greater => Greater,
                Less => Less,
                Equal => match k1.cmp(o_k1) {
                    Greater => Greater,
                    Less => Less,
                    Equal => match k2.cmp(o_k2) {
                        Greater => Greater,
                        Less => Less,
                        Equal => k3.cmp(o_k3),
                    },
                },
            },
            (HighCard(k1, k2, k3, k4, k5), HighCard(o_k1, o_k2, o_k3, o_k4, o_k5)) => {
                match k1.cmp(o_k1) {
                    Greater => Greater,
                    Less => Less,
                    Equal => match k2.cmp(o_k2) {
                        Greater => Greater,
                        Less => Less,
                        Equal => match k3.cmp(o_k3) {
                            Greater => Greater,
                            Less => Less,
                            Equal => match k4.cmp(o_k4) {
                                Greater => Greater,
                                Less => Less,
                                Equal => k5.cmp(o_k5),
                            },
                        },
                    },
                }
            }
            _ => self.cmp(other),
        }
    }
}

pub fn calculate_hand(cards: &[Card]) -> Hand {
    if let Some(hand) = is_straight_flush(cards) {
        if hand == Hand::StraightFlush(Rank::Ace) {
            return Hand::RoyalFlush;
        } else {
            return hand;
        }
    } else if let Some(hand) = is_four_of_a_kind(cards) {
        return hand;
    } else if let Some(hand) = is_full_house(cards) {
        return hand;
    } else if let Some(hand) = is_flush(cards) {
        return hand;
    } else if let Some(hand) = is_straight(cards) {
        return hand;
    } else if let Some(hand) = is_three_of_a_kind(cards) {
        return hand;
    } else if let Some(hand) = is_pairs(cards) {
        return hand;
    }
    let mut ranks = cards.iter().map(|c| c.rank).collect::<Vec<_>>();
    ranks.sort();
    let l = ranks.len();
    Hand::HighCard(
        ranks[l - 1],
        ranks[l - 2],
        ranks[l - 3],
        ranks[l - 4],
        ranks[l - 5],
    )
}

//
// Functions to check if a set of cards can be made into each hand
//
fn is_straight_flush(cards: &[Card]) -> Option<Hand> {
    // separate cards by suit:
    let mut suits: HashMap<Suit, Vec<Card>> = HashMap::new();
    for card in cards.iter() {
        suits.entry(card.suit).or_insert(Vec::new()).push(*card);
    }
    // check for straights in each suit
    for (_, card_vec) in suits.iter() {
        if card_vec.len() < 5 {
            continue;
        } else if let Some(Hand::Straight(rank)) = is_straight(card_vec.as_slice()) {
            return Some(Hand::StraightFlush(rank));
        }
    }
    None
}

fn is_four_of_a_kind(cards: &[Card]) -> Option<Hand> {
    let mut ranks = cards.iter().map(|c| c.rank).collect::<Vec<_>>();
    ranks.sort();
    // separate cards by rank:
    let mut counts: HashMap<Rank, usize> = HashMap::new();
    for rank in ranks.iter() {
        *counts.entry(*rank).or_insert(0) += 1;
    }
    for (rank, count) in counts.iter() {
        if *count == 4 {
            let mut other = ranks.clone();
            other.retain(|r| r != rank);
            other.sort();
            return Some(Hand::FourOfAKind(*rank, other[other.len() - 1]));
        }
    }
    None
}

fn is_full_house(cards: &[Card]) -> Option<Hand> {
    let mut ranks = cards.iter().map(|c| c.rank).collect::<Vec<_>>();
    ranks.sort();
    // separate cards by rank:
    let mut counts: HashMap<Rank, usize> = HashMap::new();
    for rank in ranks.iter() {
        *counts.entry(*rank).or_insert(0) += 1;
    }
    let mut trio: Option<Rank> = None;
    let mut pair: Option<Rank> = None;
    // find a trio and pair
    for (rank, count) in counts.iter() {
        if *count == 3 {
            trio = Some(*rank);
        } else if *count == 2 {
            match pair {
                None => pair = Some(*rank),
                // if there's already a pair, use new pair is higher
                Some(current_rank) => {
                    if *rank > current_rank {
                        pair = Some(*rank);
                    }
                }
            }
        }
    }
    if let (Some(trio), Some(pair)) = (trio, pair) {
        Some(Hand::FullHouse(trio, pair))
    } else {
        None
    }
}

fn is_flush(cards: &[Card]) -> Option<Hand> {
    // separate cards by suit:
    let mut suits: HashMap<Suit, Vec<Card>> = HashMap::new();
    for card in cards.iter() {
        suits.entry(card.suit).or_insert(Vec::new()).push(*card);
    }
    // check for flushes in each suit
    for (_, card_vec) in suits.iter() {
        if card_vec.len() < 5 {
            continue;
        }
        let mut ranks = card_vec.iter().map(|c| c.rank).collect::<Vec<_>>();
        ranks.sort();
        return Some(Hand::Flush(ranks[ranks.len() - 1]));
    }
    None
}

fn is_straight(cards: &[Card]) -> Option<Hand> {
    let mut ranks = cards.iter().map(|c| c.rank).collect::<Vec<_>>();
    ranks.sort();
    // remove duplicates
    ranks.dedup();
    if ranks.len() < 5 {
        return None;
    }
    // special case for ace low straight
    if ranks[0] == Rank::Two
        && ranks[1] == Rank::Three
        && ranks[2] == Rank::Four
        && ranks[3] == Rank::Five
        && ranks[ranks.len() - 1] == Rank::Ace
    {
        return Some(Hand::Straight(Rank::Five));
    }
    // check for straights in each subset of length 5
    for i in (0..=(ranks.len() - 5)).rev() {
        let mut straight = true;
        for j in i..(i + 5 - 1) {
            if (ranks[j] as u8 + 1) != (ranks[j + 1] as u8) {
                straight = false;
                break;
            }
        }
        if straight {
            return Some(Hand::Straight(ranks[i + 5 - 1]));
        }
    }
    None
}

fn is_three_of_a_kind(cards: &[Card]) -> Option<Hand> {
    let mut ranks = cards.iter().map(|c| c.rank).collect::<Vec<_>>();
    ranks.sort();
    // separate cards by rank:
    let mut counts: HashMap<Rank, usize> = HashMap::new();
    for rank in ranks.iter() {
        *counts.entry(*rank).or_insert(0) += 1;
    }
    for (rank, count) in counts.iter() {
        if *count == 3 {
            let mut other = ranks.clone();
            other.retain(|r| r != rank);
            other.sort();
            return Some(Hand::ThreeOfAKind(
                *rank,
                other[other.len() - 1],
                other[other.len() - 2],
            ));
        }
    }
    None
}

fn is_pairs(cards: &[Card]) -> Option<Hand> {
    let mut ranks = cards.iter().map(|c| c.rank).collect::<Vec<_>>();
    ranks.sort();
    // separate cards by rank:
    let mut counts: HashMap<Rank, usize> = HashMap::new();
    for rank in ranks.iter() {
        *counts.entry(*rank).or_insert(0) += 1;
    }
    let mut pairs = Vec::new();
    for (rank, count) in counts.iter() {
        if *count == 2 {
            pairs.push(*rank);
        }
    }
    // handle 3 pair case, select highest 2 pairs
    if pairs.len() == 3 {
        let mut other = ranks.clone();
        other.retain(|r| r != &pairs[0] && r != &pairs[1] && r != &pairs[2]);
        other.sort();
        pairs.sort();
        if pairs[0] > other[other.len() - 1] {
            Some(Hand::TwoPair(pairs[2], pairs[1], pairs[0]))
        } else {
            Some(Hand::TwoPair(pairs[2], pairs[1], other[other.len() - 1]))
        }
    } else if pairs.len() == 2 {
        let mut other = ranks.clone();
        other.retain(|r| r != &pairs[0] && r != &pairs[1]);
        other.sort();
        pairs.sort();
        Some(Hand::TwoPair(pairs[1], pairs[0], other[other.len() - 1]))
    } else if pairs.len() == 1 {
        let mut other = ranks.clone();
        other.retain(|r| r != &pairs[0]);
        other.sort();
        Some(Hand::Pair(
            pairs[0],
            other[other.len() - 1],
            other[other.len() - 2],
            other[other.len() - 3],
        ))
    } else {
        None
    }
}
