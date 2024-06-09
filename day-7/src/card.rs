use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;
use std::{fs::File, io, io::prelude::*, io::BufReader};

#[derive(PartialEq, PartialOrd)]
pub enum CardType {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Hand {
    bid: i32,
    hand_type: HandType,
    hand_type_two: HandType,
    card_counts: Vec<(CardType, i32)>,
    cards_str: String,
}

impl CardType {
    pub fn new(c: char) -> Self {
        match c {
            '2' => CardType::Two,
            '3' => CardType::Three,
            '4' => CardType::Four,
            '5' => CardType::Five,
            '6' => CardType::Six,
            '7' => CardType::Seven,
            '8' => CardType::Eight,
            '9' => CardType::Nine,
            'T' => CardType::T,
            'J' => CardType::J,
            'Q' => CardType::Q,
            'K' => CardType::K,
            'A' => CardType::A,
            _ => panic!("invalid card type!"),
        }
    }
}

impl Hand {
    fn new(hand: &str) -> Self {
        let bid;
        let hand_type;
        let card_counts;
        let cards_str;
        let hand_type_two;
        if let Some((str_cards, str_bid)) = hand.split_once(' ') {
            cards_str = str_cards.to_string();
            (hand_type, hand_type_two, card_counts) = Hand::parse_cards(str_cards);
            bid = str_bid
                .parse::<i32>()
                .expect("bid is not in the expected format!");
        } else {
            panic!("unexpected hand format!");
        }
        Self {
            bid,
            hand_type,
            hand_type_two,
            card_counts,
            cards_str,
        }
    }

    fn parse_cards(cards: &str) -> (HandType, HandType, Vec<(CardType, i32)>) {
        let mut dict: HashMap<char, i32> = HashMap::new();
        for c in cards.chars() {
            if let Some(value) = dict.get(&c) {
                dict.insert(c, value + 1);
            } else {
                dict.insert(c, 1);
            }
        }

        let mut card_counts: Vec<(CardType, i32)> = dict
            .into_iter()
            .map(|(k, v)| (CardType::new(k), v))
            .collect::<Vec<(CardType, i32)>>();
        card_counts.sort_by(|a, b| Hand::card_count_ordering(a, b).unwrap());

        let first_count = &card_counts[0];
        let second_count;
        if first_count.1 == 5 {
            second_count = None;
        } else {
            second_count = Some(&card_counts[1])
        }
        let hand_type = Hand::hand_type_one(first_count, second_count);
        let hand_type_two = Hand::hand_type_two(&card_counts);

        return (hand_type, hand_type_two, card_counts);
    }

    fn hand_type_one(
        first_card_count: &(CardType, i32),
        second_card_count: Option<&(CardType, i32)>,
    ) -> HandType {
        match first_card_count {
            (_, 5) => HandType::FiveOfAKind,
            (_, 4) => HandType::FourOfAKind,
            (_, 3) => {
                if second_card_count.unwrap().1 == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            (_, 2) => {
                if second_card_count.unwrap().1 == 2 {
                    HandType::TwoPair
                } else {
                    HandType::Pair
                }
            }
            (_, 1) => HandType::HighCard,
            _ => panic!("unknown hand type!"),
        }
    }

    fn hand_type_two(card_counts: &Vec<(CardType, i32)>) -> HandType {
        if card_counts[0].1 == 5 {
            return HandType::FiveOfAKind;
        }
        let types: Vec<&(CardType, i32)> = card_counts
            .into_iter()
            .filter(|x| x.0 == CardType::J)
            .collect();
        if types.len() < 1 {
            return Hand::hand_type_one(&card_counts[0], Some(&card_counts[1]));
        }
        // need the second value to be the second highest non-joker value
        let no_joker: Vec<&(CardType, i32)> = card_counts
            .into_iter()
            .filter(|x| x.0 != CardType::J)
            .collect();
        let second_count;
        if no_joker.len() < 2 {
            second_count = None;
        } else {
            second_count = Some(no_joker[1].1);
        }
        return match (no_joker[0].1, second_count, types[0].1) {
            (4, _, 1) | (3, _, 2) | (2, _, 3) | (1, _, 4) => HandType::FiveOfAKind,
            (3, _, 1) | (2, _, 2) | (1, _, 3) => HandType::FourOfAKind,
            (2, Some(2), 1) => HandType::FullHouse,
            (2, _, 1) | (1, _, 2) => HandType::ThreeOfAKind,
            (1, _, 1) => HandType::Pair,
            _ => Hand::hand_type_one(&card_counts[0], Some(&card_counts[1])),
        };
    }

    fn card_count_ordering(a: &(CardType, i32), b: &(CardType, i32)) -> Option<Ordering> {
        if a.1 < b.1 {
            return Some(Ordering::Greater);
        }
        if a.1 > b.1 {
            return Some(Ordering::Less);
        }
        // otherwise the counts are equal
        // there can be no ties
        if a.0 > b.0 {
            return Some(Ordering::Less);
        }
        return Some(Ordering::Greater);
    }

    fn hand_order(a: &Hand, b: &Hand) -> Option<Ordering> {
        // if a is better hand type, return Less
        // if b is better, retrun Greater
        if a.hand_type_two != b.hand_type_two {
            if a.hand_type_two > b.hand_type_two {
                return Some(Ordering::Less);
            }
            return Some(Ordering::Greater);
        }

        // otherwise the hand type is the same
        // for 5, 4, 3, 2, 1 of a kind, compare the set then compare the other
        // singles
        let m: Vec<char> = a.cards_str.chars().collect();
        let n: Vec<char> = b.cards_str.chars().collect();
        for (x, y) in zip(m, n) {
            let left = CardType::new(x);
            let right = CardType::new(y);
            if left != right {
                if left > right {
                    return Some(Ordering::Less);
                } else {
                    return Some(Ordering::Greater);
                }
            }
        }
        return Some(Ordering::Equal);
    }
}

#[test]
fn five_of_kind() {
    let hand_1 = Hand::new("AAAAA 1234");
    assert!(hand_1.bid == 1234);
    assert!(hand_1.hand_type == HandType::FiveOfAKind);
    assert!(hand_1.card_counts[0] == (CardType::A, 5));
}

#[test]
fn two_pair() {
    let hand_2 = Hand::new("87687 11111");
    assert!(hand_2.hand_type == HandType::TwoPair);
    assert!(hand_2.card_counts[0].0 == CardType::Eight);
}

#[test]
fn one_pair() {
    let hand_2 = Hand::new("85687 11111");
    assert!(hand_2.hand_type == HandType::Pair);

    let hand_4 = Hand::new("5AJ85 1");
    assert!(hand_4.hand_type == HandType::Pair);
}

#[test]
fn full_house() {
    let hand_3 = Hand::new("33232 1");
    assert!(hand_3.hand_type == HandType::FullHouse);

    let hand_6 = Hand::new("68686 123");
    assert!(hand_6.hand_type == HandType::FullHouse);
}

#[test]
fn hand_order() {
    let hand_1 = Hand::new("AAAAA 1234");
    let hand_2 = Hand::new("87687 11111");
    let hand_3 = Hand::new("85687 11111");
    let hand_5 = Hand::new("77234 123");
    let hand_4 = Hand::new("5AJ85 1");
    let hand_6 = Hand::new("68686 123");
    let hand_7 = Hand::new("J4K3A 22");
    let hand_8 = Hand::new("7A7AA 336");

    assert!(Hand::hand_order(&hand_1, &hand_2) == Some(Ordering::Less));
    assert!(Hand::hand_order(&hand_3, &hand_5) == Some(Ordering::Less));
    assert!(Hand::hand_order(&hand_6, &hand_4) == Some(Ordering::Less));
    assert!(Hand::hand_order(&hand_8, &hand_7) == Some(Ordering::Less));
}

#[test]
fn hand_types() {
    assert!(HandType::TwoPair > HandType::Pair);
}

pub fn part_one(file: File) -> i32 {
    let reader = BufReader::new(file);
    let mut vec: Vec<Hand> = vec![];
    for l in reader.lines().map(|x| x.expect("error reading line")) {
        vec.push(Hand::new(&l));
    }

    vec.sort_by(|x, y| Hand::hand_order(x, y).unwrap());
    let rev_vec: Vec<Hand> = vec.into_iter().rev().collect();

    let mut tot = 0;
    for (i, hand) in rev_vec.into_iter().enumerate() {
        tot += ((i + 1) as i32) * hand.bid;
        if hand.cards_str.chars().any(|x| x == 'J') {
            println!(
                "rank: {}; card: {}; bid: {}; type: {:?}",
                i + 1,
                hand.cards_str,
                hand.bid,
                hand.hand_type_two
            );
        }
    }

    return tot;
}
