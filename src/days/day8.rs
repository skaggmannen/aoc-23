extern crate itertools;
extern crate num;

use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;
use num::integer::lcm;

use crate::util;

pub fn part1(input: &str) -> Result<String> {
    let parts = input.split_once("\n\n").unwrap();

    let instructions = parts.0.trim().chars().collect_vec();
    let network: Network = parts.1.trim().parse().unwrap();

    let mut curr = "AAA";

    for step in 0.. {
        if curr == "ZZZ" {
            return Ok(format!("{}", step));
        }
        match instructions[step % instructions.len()] {
            'L' => {
                curr = &network.nodes.get(curr).unwrap().0;
            }
            'R' => {
                curr = &network.nodes.get(curr).unwrap().1;
            }
            _ => {}
        }
    }

    todo!()
}

#[test]
fn test_part1() {
    assert_eq!("2", part1(TEST_INPUT).unwrap());
    assert_eq!("6", part1(TEST_INPUT_2).unwrap());
}

pub fn part2(input: &str) -> Result<String> {
    let parts = input.split_once("\n\n").unwrap();

    let instructions = parts.0.trim().chars().collect_vec();
    let network: Network = parts.1.trim().parse().unwrap();

    let curr = network
        .nodes
        .keys()
        .filter(|k| k.ends_with("A"))
        .collect_vec();

    let mut moves = Vec::new();
    for i in 0..curr.len() {
        moves.push(network.get_moves(&instructions, curr[i]));
    }

    moves.sort();
    moves.reverse();

    let gcf = instructions.len();
    let mut lcm = (moves.pop().unwrap() * moves.pop().unwrap()) / gcf;

    if moves.len() == 0 {
        return Ok((lcm * gcf).to_string());
    }

    while moves.len() > 0 {
        lcm = (lcm * moves.pop().unwrap()) / gcf;
    }

    return Ok(lcm.to_string());
}

#[test]
fn test_part2() {
    assert_eq!("6", part2(TEST_INPUT_3).unwrap());
}

struct Network {
    nodes: HashMap<String, (String, String)>,
}

impl Network {
    fn get_moves<'a>(&'a self, instructions: &Vec<char>, start: &'a String) -> usize {
        let mut curr = start;

        let mut moves = 0;

        for step in 0.. {
            if curr.ends_with("Z") {
                moves = step;
                break;
            }

            match instructions[step % instructions.len()] {
                'L' => curr = &self.nodes[curr].0,
                'R' => curr = &self.nodes[curr].1,
                _ => {}
            }
        }

        moves
    }
}

impl FromStr for Network {
    type Err = Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut network = HashMap::new();

        let lines = util::non_empty_lines(s);
        for l in lines {
            let (name, dsts) = l.split_once(" = ").unwrap();

            let (left, right) = dsts
                .strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split_once(", ")
                .unwrap();

            network.insert(name.to_owned(), (left.to_owned(), right.to_owned()));
        }

        Ok(Network { nodes: network })
    }
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------

#[cfg(test)]
const TEST_INPUT: &str = "
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

#[cfg(test)]
const TEST_INPUT_2: &str = "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

#[cfg(test)]
const TEST_INPUT_3: &str = "
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
