use std::collections::HashSet;

use itertools::Itertools;

use crate::util;

extern crate itertools;
extern crate num;

pub fn part1(input: &str) -> Result<String> {
    let mut tiles = util::non_empty_lines(input)
        .map(|l| {
            l.chars()
                .map(|c| Tile {
                    visited: HashSet::new(),
                    c,
                })
                .collect_vec()
        })
        .collect_vec();

    trace_beam(&mut tiles, Pos(0, 0), Direction::Right);

    let score = tiles
        .iter()
        .fold(Vec::new(), |mut acc, row| {
            acc.extend(row);
            acc
        })
        .iter()
        .filter(|t| !t.visited.is_empty())
        .count();

    Ok(score.to_string())
}

#[test]
fn test_part1() {
    assert_eq!("46", part1(TEST_INPUT).unwrap());
}

pub fn part2(input: &str) -> Result<String> {
    let tiles = util::non_empty_lines(input)
        .map(|l| {
            l.chars()
                .map(|c| Tile {
                    visited: HashSet::new(),
                    c,
                })
                .collect_vec()
        })
        .collect_vec();

    let mut scores = Vec::new();

    for row in 0..tiles.len() {
        {
            let mut tiles = tiles.clone();
            trace_beam(&mut tiles, Pos(row as i32, 0), Direction::Right);
            scores.push(
                tiles
                    .iter()
                    .fold(Vec::new(), |mut acc, row| {
                        acc.extend(row);
                        acc
                    })
                    .iter()
                    .filter(|t| !t.visited.is_empty())
                    .count(),
            );
        }
        {
            let col = tiles[0].len() as i32 - 1;
            let mut tiles = tiles.clone();
            trace_beam(&mut tiles, Pos(row as i32, col), Direction::Left);
            scores.push(
                tiles
                    .iter()
                    .fold(Vec::new(), |mut acc, row| {
                        acc.extend(row);
                        acc
                    })
                    .iter()
                    .filter(|t| !t.visited.is_empty())
                    .count(),
            );
        }
    }

    for col in 0..tiles.len() {
        {
            let mut tiles = tiles.clone();
            trace_beam(&mut tiles, Pos(0, col as i32), Direction::Down);
            scores.push(
                tiles
                    .iter()
                    .fold(Vec::new(), |mut acc, row| {
                        acc.extend(row);
                        acc
                    })
                    .iter()
                    .filter(|t| !t.visited.is_empty())
                    .count(),
            );
        }
        {
            let row = tiles.len() as i32 - 1;
            let mut tiles = tiles.clone();
            trace_beam(&mut tiles, Pos(row, col as i32), Direction::Up);
            scores.push(
                tiles
                    .iter()
                    .fold(Vec::new(), |mut acc, row| {
                        acc.extend(row);
                        acc
                    })
                    .iter()
                    .filter(|t| !t.visited.is_empty())
                    .count(),
            );
        }
    }

    let score = scores.iter().max().unwrap();

    Ok(score.to_string())
}

#[test]
fn test_part2() {
    assert_eq!("51", part2(TEST_INPUT).unwrap());
}

#[cfg(test)]
const TEST_INPUT: &str = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

#[derive(Clone)]
struct Tile {
    c: char,
    visited: HashSet<Direction>,
}

struct Pos(i32, i32);

impl Pos {
    fn walk(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Pos(self.0 - 1, self.1),
            Direction::Left => Pos(self.0, self.1 - 1),
            Direction::Right => Pos(self.0, self.1 + 1),
            Direction::Down => Pos(self.0 + 1, self.1),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn trace_beam(tiles: &mut Vec<Vec<Tile>>, pos: Pos, direction: Direction) {
    let Pos(row, col) = pos;

    if row < 0 || col < 0 {
        return;
    }

    if row >= tiles.len() as i32 || col >= tiles[0].len() as i32 {
        return;
    }

    if tiles[row as usize][col as usize]
        .visited
        .contains(&direction)
    {
        return;
    }

    tiles[row as usize][col as usize].visited.insert(direction);

    use Direction::*;

    match tiles[row as usize][col as usize].c {
        '\\' => match direction {
            Up => trace_beam(tiles, pos.walk(Left), Left),
            Down => trace_beam(tiles, pos.walk(Right), Right),
            Left => trace_beam(tiles, pos.walk(Up), Up),
            Right => trace_beam(tiles, pos.walk(Down), Down),
        },
        '/' => match direction {
            Up => trace_beam(tiles, pos.walk(Right), Right),
            Down => trace_beam(tiles, pos.walk(Left), Left),
            Left => trace_beam(tiles, pos.walk(Down), Down),
            Right => trace_beam(tiles, pos.walk(Up), Up),
        },
        '|' => match direction {
            Left | Right => {
                trace_beam(tiles, pos.walk(Up), Up);
                trace_beam(tiles, pos.walk(Down), Down);
            }
            _ => trace_beam(tiles, pos.walk(direction), direction),
        },
        '-' => match direction {
            Up | Down => {
                trace_beam(tiles, pos.walk(Left), Left);
                trace_beam(tiles, pos.walk(Right), Right);
            }
            _ => trace_beam(tiles, pos.walk(direction), direction),
        },
        _ => trace_beam(tiles, pos.walk(direction), direction),
    }
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------
