extern crate itertools;

use std::{collections::HashMap, fmt::Display, hash::Hash, str::FromStr};

use itertools::Itertools;

use crate::util;

pub fn part1(input: &str) -> Result<String> {
    let lines = util::non_empty_lines(input);

    let mut hands = lines.map(|s| parse_hand(&s)).collect_vec();
    hands.sort();

    let score: usize = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.2)
        .sum();

    Ok(format!("{}", score))
}

#[test]
fn test_part1() {
    assert_eq!("6440", part1(TEST_INPUT).unwrap());
}

pub fn part2(input: &str) -> Result<String> {
    let lines = util::non_empty_lines(input);

    let mut hands = lines.map(|s| parse_joker_hand(&s)).collect_vec();
    hands.sort();

    let score: usize = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.2)
        .sum();

    Ok(format!("{}", score))
}

#[test]
fn test_part2() {
    assert_eq!("5905", part2(TEST_INPUT).unwrap());
}

struct Hand(Kind, String, u32);

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Hand(kind, cards, bid) = self;

        write!(f, "{} => {} {}", kind, cards, bid)
    }
}

fn parse_hand(s: &str) -> (Kind, String, usize) {
    let (cards_str, bid) = s.split_once(" ").unwrap();

    let cards = cards_str
        .chars()
        .map(|c| match c {
            'A' => 'E',
            'K' => 'D',
            'Q' => 'C',
            'J' => 'B',
            'T' => 'A',
            _ => c,
        })
        .collect::<String>();

    (cards.parse().unwrap(), cards, bid.parse().unwrap())
}

fn parse_joker_hand(s: &str) -> (Kind, String, usize) {
    let (cards_str, bid) = s.split_once(" ").unwrap();

    let cards = cards_str
        .chars()
        .map(|c| match c {
            'A' => 'E',
            'K' => 'D',
            'Q' => 'C',
            'J' => 'B',
            'T' => 'A',
            _ => c,
        })
        .collect::<String>();

    (
        replace_joker(cards.clone()).parse().unwrap(),
        cards.replace('B', "0"), // The joker is the weakest card
        bid.parse().unwrap(),
    )
}

fn replace_joker(hand: String) -> String {
    if !hand.contains('B') {
        return hand;
    }

    if hand == "BBBBB" {
        return String::from("EEEEE");
    }

    let candidates: Vec<char> = hand.chars().filter(|&c| c != 'B').collect();

    let mut counts: Vec<(usize, char)> = count_items(&candidates)
        .iter()
        .map(|(&c, &count)| (count, c))
        .collect();

    counts.sort();

    hand.replace('B', &counts.last().unwrap().1.to_string())
}

#[test]
fn test_kind() {
    assert_eq!(Kind::TwoPairs, Kind::from_str("2JJTT").unwrap());
    assert_eq!(Kind::FiveOfKind, Kind::from_str("AAAAA").unwrap());
    assert_eq!(Kind::FourOfKind, Kind::from_str("AAAAK").unwrap());
    assert_eq!(Kind::ThreeOfKind, Kind::from_str("AAAKQ").unwrap());
    assert_eq!(Kind::TwoPairs, Kind::from_str("AAKKQ").unwrap());
    assert_eq!(Kind::Pair, Kind::from_str("AAKQJ").unwrap());
    assert_eq!(Kind::HighCard, Kind::from_str("AKQJT").unwrap());
    assert_eq!(Kind::HighCard, Kind::from_str("2658J").unwrap());
}

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq)]
enum Kind {
    HighCard,
    Pair,
    TwoPairs,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl FromStr for Kind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let counts = count_items(&s.chars().collect());

        Ok(match counts.len() {
            1 => Kind::FiveOfKind,
            2 if counts.values().any(|&c| c == 4) => Kind::FourOfKind,
            2 if counts.values().any(|&c| c == 3) => Kind::FullHouse,
            3 if counts.values().any(|&c| c == 3) => Kind::ThreeOfKind,
            3 if counts.values().any(|&c| c == 2) => Kind::TwoPairs,
            4 => Kind::Pair,
            _ => Kind::HighCard,
        })
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn count_items<T: Copy + Hash + Eq + PartialEq>(v: &Vec<T>) -> HashMap<T, usize> {
    let mut counts = HashMap::<T, usize>::new();

    for &c in v.iter() {
        let e = counts.entry(c).or_insert(0);
        *e += 1;
    }

    counts
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------

#[cfg(test)]
const TEST_INPUT: &str = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
