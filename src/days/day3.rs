extern crate itertools;

use std::collections::HashMap;

use crate::util;

pub fn part1(input: &str) -> Result<String> {
    let lines = util::non_empty_lines(input).collect::<Vec<_>>();

    let board = parse_input(lines);

    let part_numbers = board.part_numbers();

    Ok(format!("{}", part_numbers.numbers.iter().sum::<u32>()))
}

pub fn part2(input: &str) -> Result<String> {
    let lines = util::non_empty_lines(input).collect::<Vec<_>>();

    let board = parse_input(lines);
    let part_numbers = board.part_numbers();

    let score: u32 = part_numbers
        .gears
        .values()
        .filter(|vs| vs.len() > 1)
        .map(|vs| vs.iter().fold(1, |acc, v| acc * v))
        .sum();

    Ok(format!("{}", score))
}

fn parse_input(lines: Vec<String>) -> Board {
    let mut board = Board { rows: Vec::new() };

    for l in lines {
        let mut cols = Vec::new();
        for c in l.chars() {
            cols.push(c);
        }

        board.rows.push(cols);
    }

    board
}

struct Board {
    rows: Vec<Vec<char>>,
}

impl Board {
    pub fn part_numbers(&self) -> PartNumbers {
        let mut part_numbers = PartNumbers {
            numbers: Vec::new(),
            gears: HashMap::new(),
        };

        for y in 0..self.rows.len() as i32 {
            let mut symbol = Symbol::None;
            let mut curr_nbr = String::new();

            let row = &self.rows[y as usize];

            for x in 0..row.len() as i32 {
                let c = row[x as usize];

                if c.is_digit(10) {
                    if let Symbol::None = symbol {
                        symbol = self.check_for_symbol(x, y);
                    }

                    curr_nbr.push(c);
                } else {
                    if curr_nbr.len() > 0 {
                        let nbr = curr_nbr.parse().unwrap();

                        match symbol {
                            Symbol::Gear { pos } => {
                                part_numbers.numbers.push(nbr);
                                part_numbers
                                    .gears
                                    .entry(pos)
                                    .or_insert(Vec::new())
                                    .push(nbr);
                            }
                            Symbol::Other => {
                                part_numbers.numbers.push(nbr);
                            }
                            Symbol::None => {}
                        }

                        curr_nbr = String::new();
                        symbol = Symbol::None;
                    }
                }
            }

            if curr_nbr.len() > 0 {
                let nbr = curr_nbr.parse().unwrap();

                match symbol {
                    Symbol::Gear { pos } => {
                        part_numbers.numbers.push(nbr);
                        part_numbers
                            .gears
                            .entry(pos)
                            .or_insert(Vec::new())
                            .push(nbr);
                    }
                    Symbol::Other => {
                        part_numbers.numbers.push(nbr);
                    }
                    Symbol::None => {}
                }
            }
        }

        return part_numbers;
    }

    fn check_for_symbol(&self, x: i32, y: i32) -> Symbol {
        for dy in [-1, 0, 1] {
            let pos_y = (y + dy) as usize;
            let Some(row) = self.rows.get(pos_y) else {
                continue;
            };

            for dx in [-1, 0, 1] {
                let pos_x = (x + dx) as usize;
                let Some(&c) = row.get((x + dx) as usize) else {
                    continue;
                };

                if c.is_digit(10) || c == '.' {
                    continue;
                }

                if c == '*' {
                    return Symbol::Gear {
                        pos: (pos_x, pos_y),
                    };
                }

                return Symbol::Other;
            }
        }

        return Symbol::None;
    }
}

enum Symbol {
    None,
    Gear { pos: (usize, usize) },
    Other,
}

struct PartNumbers {
    numbers: Vec<u32>,
    gears: HashMap<(usize, usize), Vec<u32>>,
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------

#[test]
fn test_part1() {
    assert_eq!("4361", part1(TEST_INPUT).unwrap());
}

#[test]
fn test_part2() {
    assert_eq!("467835", part2(TEST_INPUT).unwrap());
}

#[cfg(test)]
const TEST_INPUT: &str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
