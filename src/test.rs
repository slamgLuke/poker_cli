// test.rs
use crate::poker::{Rank::*, Suit::*, *};
use crate::hands::{Hand::*, *};
use std::cmp::Ordering::*;

//
// HAND ORDERING TESTS:
// 
#[test]
fn hand_cmp_1() {
    let hand1_1 = Pair(Ten, Nine, Eight, Two);
    let hand1_2 = TwoPair(Three, Two, Eight);
    assert_eq!(hand1_1.compare(&hand1_2), Less);
    assert_eq!(hand1_2.compare(&hand1_1), Greater);
}

#[test]
fn hand_cmp_2() {
    let hand2_1 = TwoPair(Jack, Eight, Four);
    let hand2_2 = ThreeOfAKind(Jack, Nine, Four);
    assert_eq!(hand2_2.compare(&hand2_1), Greater);
    assert_eq!(hand2_1.compare(&hand2_2), Less);
}

#[test]
fn hand_cmp_3() {
    let hand3_1 = FourOfAKind(Seven, Two);
    let hand3_2 = FourOfAKind(Seven, Five);
    let hand3_3 = FourOfAKind(Nine, Four);
    assert_eq!(hand3_1.compare(&hand3_2), Less);
    assert_eq!(hand3_1.compare(&hand3_3), Less);
    assert_eq!(hand3_2.compare(&hand3_3), Less);
    assert_eq!(hand3_2.compare(&hand3_1), Greater);
    assert_eq!(hand3_3.compare(&hand3_1), Greater);
    assert_eq!(hand3_3.compare(&hand3_2), Greater);
}

#[test]
fn hand_cmp_4() {
    assert_eq!(RoyalFlush.compare(&RoyalFlush), Equal);
    assert_eq!(RoyalFlush.compare(&StraightFlush(King)), Greater);
    assert_eq!(StraightFlush(King).compare(&RoyalFlush), Less);

    let hand4_1 = StraightFlush(Ten);
    let hand4_2 = Flush(Jack);
    assert_eq!(hand4_1.compare(&hand4_2), Greater);
    assert_eq!(hand4_2.compare(&hand4_1), Less);
}

#[test]
fn hand_cmp_5() {
    let hand5_1 = Straight(Ten);
    let hand5_2 = Straight(Ten);
    assert_eq!(hand5_1.compare(&hand5_2), Equal);
    assert_eq!(hand5_2.compare(&hand5_1), Equal);
}

#[test]
fn hand_cmp_6() {
    let hand6_1 = FullHouse(Ten, Two);
    let hand6_2 = FullHouse(Ten, Three);
    assert_eq!(hand6_1.compare(&hand6_2), Less);
    assert_eq!(hand6_2.compare(&hand6_1), Greater);
}

#[test]
fn hand_cmp_7() {
    let hand7_1 = HighCard(Ten, Nine, Eight, Two, Three);
    let hand7_2 = HighCard(Ten, Nine, Eight, Two, Four);
    assert_eq!(hand7_1.compare(&hand7_2), Less);
    assert_eq!(hand7_2.compare(&hand7_1), Greater);
    assert_eq!(hand7_1.compare(&hand7_1), Equal);
}

//
// HAND CALCULATION TESTS:
//
#[test]
fn hand_calc_1() {
    let hand_1 = vec![
        Card { rank: Ten, suit: Hearts },
        Card { rank: Nine, suit: Spades },
        Card { rank: Eight, suit: Clubs },
        Card { rank: Two, suit: Hearts },
        Card { rank: Three, suit: Spades },
        Card { rank: King, suit: Hearts },
        Card { rank: Four, suit: Clubs },
    ];
    assert_eq!(calculate_hand(&hand_1), HighCard(King, Ten, Nine, Eight, Four));
}

#[test]
fn hand_calc_2() {
    let hand_2 = vec![
        Card { rank: Ten, suit: Hearts },
        Card { rank: Nine, suit: Spades },
        Card { rank: Ace, suit: Hearts },
        Card { rank: Eight, suit: Clubs },
        Card { rank: Two, suit: Hearts },
        Card { rank: Three, suit: Spades },
        Card { rank: Ace, suit: Clubs },
    ];
    assert_eq!(calculate_hand(&hand_2), Pair(Ace, Ten, Nine, Eight));
}

#[test]
fn hand_calc_3_1() {
    let hand_3 = vec![
        Card { rank: Ten, suit: Hearts },
        Card { rank: Nine, suit: Spades },
        Card { rank: Ten, suit: Diamonds },
        Card { rank: Eight, suit: Clubs },
        Card { rank: Two, suit: Hearts },
        Card { rank: Ace, suit: Clubs },
        Card { rank: Ace, suit: Diamonds },
    ];
    assert_eq!(calculate_hand(&hand_3), TwoPair(Ace, Ten, Nine));
}

#[test]
fn hand_calc_3_2() {
    let hand_3 = vec![
        Card { rank: Ten, suit: Hearts },
        Card { rank: Nine, suit: Spades },
        Card { rank: Ten, suit: Diamonds },
        Card { rank: Eight, suit: Clubs },
        Card { rank: Two, suit: Hearts },
        Card { rank: Ace, suit: Clubs },
        Card { rank: Ace, suit: Diamonds },
    ];
    assert_eq!(calculate_hand(&hand_3), TwoPair(Ace, Ten, Nine));
}

#[test]
fn hand_calc_3_3() {
    let hand_3 = vec![
        Card { rank: Ace, suit: Clubs },
        Card { rank: Nine, suit: Spades },
        Card { rank: Eight, suit: Clubs },
        Card { rank: Eight, suit: Hearts },
        Card { rank: Ace, suit: Diamonds },
        Card { rank: Ten, suit: Diamonds },
        Card { rank: Ten, suit: Hearts },
    ];
    assert_eq!(calculate_hand(&hand_3), TwoPair(Ace, Ten, Nine));
}

#[test]
fn hand_calc_4() {
    let hand_4 = vec![
        Card { rank: Nine, suit: Spades },
        Card { rank: Queen, suit: Hearts },
        Card { rank: Two, suit: Hearts },
        Card { rank: Three, suit: Spades },
        Card { rank: Ace, suit: Clubs },
        Card { rank: Ace, suit: Diamonds },
        Card { rank: Ace, suit: Spades },
    ];
    assert_eq!(calculate_hand(&hand_4), ThreeOfAKind(Ace, Queen, Nine));
}

#[test]
fn hand_calc_5() {
    let hand_5 = vec![
        Card { rank: King, suit: Clubs },
        Card { rank: Nine, suit: Spades },
        Card { rank: Queen, suit: Hearts },
        Card { rank: King, suit: Spades },
        Card { rank: Three, suit: Spades },
        Card { rank: King, suit: Diamonds },
        Card { rank: King, suit: Hearts },
    ];
    assert_eq!(calculate_hand(&hand_5), FourOfAKind(King, Queen));
}

#[test]
fn hand_calc_6() {
    let hand_6 = vec![
        Card { rank: Three, suit: Hearts },
        Card { rank: Seven, suit: Hearts },
        Card { rank: Jack, suit: Hearts },
        Card { rank: Ten, suit: Diamonds },
        Card { rank: Two, suit: Hearts },
        Card { rank: Three, suit: Spades },
        Card { rank: Nine, suit: Hearts },
    ];
    assert_eq!(calculate_hand(&hand_6), Flush(Jack));
}

#[test]
fn hand_calc_7_1() {
    let hand_7 = vec![
        Card { rank: Ten, suit: Diamonds },
        Card { rank: Eight, suit: Diamonds },
        Card { rank: Seven, suit: Diamonds },
        Card { rank: Three, suit: Spades },
        Card { rank: Nine, suit: Diamonds },
        Card { rank: Six, suit: Hearts },
        Card { rank: Jack, suit: Diamonds },
    ];
    assert_eq!(calculate_hand(&hand_7), StraightFlush(Jack));
}

#[test]
fn hand_calc_7_2() {
    let hand_7 = vec![
        Card { rank: Queen, suit: Clubs },
        Card { rank: Jack, suit: Clubs },
        Card { rank: Ten, suit: Clubs },
        Card { rank: Four, suit: Spades },
        Card { rank: Nine, suit: Clubs },
        Card { rank: Ace, suit: Diamonds },
        Card { rank: King, suit: Clubs },
    ];
    assert_eq!(calculate_hand(&hand_7), StraightFlush(King));
}

#[test]
fn hand_calc_8_1() {
    let hand_8 = vec![
        Card { rank: Three, suit: Spades },
        Card { rank: Seven, suit: Hearts },
        Card { rank: Two, suit: Diamonds },
        Card { rank: Four, suit: Diamonds },
        Card { rank: King, suit: Hearts },
        Card { rank: Five, suit: Diamonds },
        Card { rank: Ace, suit: Spades },
    ];
    assert_eq!(calculate_hand(&hand_8), Straight(Five));
}

#[test]
fn hand_calc_8_2() {
    let hand_8 = vec![
        Card { rank: Queen, suit: Clubs },
        Card { rank: Queen, suit: Clubs },
        Card { rank: Jack, suit: Clubs },
        Card { rank: Four, suit: Spades },
        Card { rank: Ten, suit: Clubs },
        Card { rank: Ace, suit: Clubs },
        Card { rank: King, suit: Clubs },
    ];
    assert_eq!(calculate_hand(&hand_8), RoyalFlush);
}

#[test]
fn hand_calc_9() {
    let hand_9 = vec![
        Card { rank: Queen, suit: Clubs },
        Card { rank: Jack, suit: Clubs },
        Card { rank: Ten, suit: Clubs },
        Card { rank: Four, suit: Spades },
        Card { rank: Nine, suit: Clubs },
        Card { rank: Ace, suit: Clubs },
        Card { rank: King, suit: Clubs },
    ];
    assert_eq!(calculate_hand(&hand_9), RoyalFlush);
}



#[test]
fn hand_calc_10() {
    let hand_10 = vec![
        Card { rank: Six, suit: Clubs },
        Card { rank: Nine, suit: Clubs },
        Card { rank: Eight, suit: Clubs },
        Card { rank: Five, suit: Clubs },
        Card { rank: Three, suit: Clubs },
        Card { rank: Seven, suit: Clubs },
        Card { rank: Ten, suit: Spades },
    ];
    assert_eq!(calculate_hand(&hand_10), StraightFlush(Nine));

}

#[test]
fn hand_calc_11() {
    let hand_11 = vec![
        Card { rank: Two, suit: Clubs },
        Card { rank: Three, suit: Clubs },
        Card { rank: Two, suit: Hearts },
        Card { rank: Three, suit: Hearts },
        Card { rank: Ace, suit: Spades },
        Card { rank: Ace, suit: Spades },
        Card { rank: Ace, suit: Spades },
    ];
    assert_eq!(calculate_hand(&hand_11), FullHouse(Ace, Three));
}


//
// RANK ORD TESTS:
//
#[test]
fn rank_ord_1() {
    assert_eq!(Ace.cmp(&King), Greater);
    assert_eq!(King.cmp(&Ace), Less);
    assert_eq!(Ace.cmp(&Ace), Equal);
}

#[test]
fn rank_ord_2() {
    assert_eq!(Ten.cmp(&Jack), Less);
    assert_eq!(Ten.cmp(&Nine), Greater);
    assert_eq!(Nine.cmp(&Jack), Less);
}

#[test]
fn rank_ord_3() {
    assert_eq!(Two.cmp(&Ace), Less);
    assert_eq!(Ace.cmp(&Two), Greater);
}

