extern crate itertools;

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::util;

pub fn part1(input: &str) -> Result<String> {
    let lines = util::non_empty_lines(input).collect::<Vec<_>>();

    let cards = lines.iter().map(parse_card);

    let score: u32 = cards.map(|c| check_score(&c)).sum();

    Ok(format!("{}", score))
}

pub fn part2(input: &str) -> Result<String> {
    let lines = util::non_empty_lines(input).collect::<Vec<_>>();

    let cards = lines.iter().map(parse_card).collect_vec();
    let mut copies = HashMap::<usize, u32>::new();

    let mut card_count = 0;

    for i in 0..cards.len() {
        let card = &cards[i];
        let score = winning_nbrs(card);
        let multiplier = *copies.get(&i).unwrap_or(&1);

        for j in i + 1..i + 1 + score as usize {
            let e = copies.entry(j).or_insert(1);
            *e += 1 * multiplier
        }

        card_count += multiplier;
    }

    Ok(format!("{}", card_count))
}

struct Card {
    winning_nbrs: HashSet<u32>,
    nbrs: Vec<u32>,
}

fn parse_card(s: &String) -> Card {
    let (_, numbers) = s.split_once(":").unwrap();

    let (first, second) = numbers.split_once("|").unwrap();

    Card {
        winning_nbrs: parse_nbrs(first).collect(),
        nbrs: parse_nbrs(second).collect(),
    }
}

fn parse_nbrs(s: &str) -> impl Iterator<Item = u32> + '_ {
    s.trim()
        .split(" ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
}

fn check_score(c: &Card) -> u32 {
    let mut score = 0;
    for n in c.nbrs.iter() {
        if c.winning_nbrs.contains(n) {
            score = if score == 0 { 1 } else { score * 2 }
        }
    }

    return score;
}

fn winning_nbrs(c: &Card) -> usize {
    c.nbrs.iter().filter(|n| c.winning_nbrs.contains(n)).count()
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------

#[test]
fn test_part1() {
    assert_eq!("13", part1(TEST_INPUT).unwrap());
}

#[test]
fn test_part2() {
    assert_eq!("30", part2(TEST_INPUT).unwrap());
}

#[cfg(test)]
const TEST_INPUT: &str = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
