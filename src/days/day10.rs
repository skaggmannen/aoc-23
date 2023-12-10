extern crate itertools;
extern crate num;

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;

use crate::util;

pub fn part1(input: &str) -> Result<String> {
    let map = input.parse::<Map>()?;

    let loop_size = map.iter().count();

    Ok((loop_size / 2).to_string())
}

#[test]
fn test_part1() {
    assert_eq!("8", part1(TEST_INPUT).unwrap());
}

pub fn part2(input: &str) -> Result<String> {
    let map = input.parse::<Map>()?;

    let loop_tiles = map.iter().collect::<HashSet<_>>();

    let mut inside_area = 0;
    let mut outside_area = 0;

    for row in 0..map.lines.len() {
        let mut inside = false;
        let mut first_loop_tile: Option<char> = None;
        let line = &map.lines[row];

        for col in 0..line.len() {
            let pos = Pos(row as i32, col as i32);
            let c = map.values[&pos];

            if loop_tiles.contains(&pos) {
                if let Some(f) = first_loop_tile {
                    match (f, c) {
                        ('F', '7') | ('L', 'J') => {
                            // We stepped off on the same side
                            first_loop_tile = None;
                            inside = !inside;
                        }
                        ('L', '7') | ('F', 'J') => {
                            // We stepped off on the other side
                            first_loop_tile = None;
                        }
                        ('F' | 'L', '-') => {
                            // We're still walking along the loop
                        }
                        _ => {
                            // We stepped of one part of the loop onto another
                            first_loop_tile = Some(c);
                            inside = !inside;
                        }
                    }
                } else {
                    // We stepped onto the loop
                    first_loop_tile = Some(c);
                    inside = !inside;
                }
            } else {
                if let Some(_) = first_loop_tile {
                    // We stepped of the loop
                    first_loop_tile = None;
                }

                if inside {
                    inside_area += 1;
                } else {
                    outside_area += 1;
                }
            }
        }
    }

    Ok(inside_area.to_string())
}

#[test]
fn test_part2() {
    assert_eq!("4", part2(TEST_INPUT_2).unwrap());
    assert_eq!("8", part2(TEST_INPUT_3).unwrap());
}

#[test]
fn test_part2_input_3() {
    assert_eq!("8", part2(TEST_INPUT_3).unwrap());
}

#[test]
fn test_part2_input_4() {
    assert_eq!("10", part2(TEST_INPUT_4).unwrap());
}

#[cfg(test)]
const TEST_INPUT: &str = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

#[cfg(test)]
const TEST_INPUT_2: &str = "
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

#[cfg(test)]
const TEST_INPUT_3: &str = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

#[cfg(test)]
const TEST_INPUT_4: &str = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

struct Map {
    values: HashMap<Pos, char>,
    start: Pos,
    lines: Vec<String>,
}

struct MapIterator<'a> {
    m: &'a Map,
    pos: Pos,
    delta: Pos,
    visited: HashSet<Pos>,
}

impl Iterator for MapIterator<'_> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        if self.visited.contains(&self.pos) {
            return None;
        }

        self.visited.insert(self.pos);

        let c = self.m.values[&self.pos];

        self.delta = next_delta(c, self.delta).unwrap();
        let Pos(dy, dx) = self.delta;
        let Pos(row, col) = self.pos;

        self.pos = Pos(row + dy, col + dx);

        Some(self.pos)
    }
}

impl Map {
    fn iter<'a>(&'a self) -> MapIterator<'a> {
        MapIterator {
            m: self,
            pos: self.start,
            delta: start_delta(&self.values[&self.start]),
            visited: HashSet::new(),
        }
    }

    fn start_char(&self) -> char {
        let Pos(row, col) = self.start;

        let north = self.values.get(&Pos(row - 1, col)).unwrap_or(&'.');
        let east = self.values.get(&Pos(row, col + 1)).unwrap_or(&'.');
        let south = self.values.get(&Pos(row + 1, col)).unwrap_or(&'.');
        let west = self.values.get(&Pos(row, col - 1)).unwrap_or(&'.');

        match (north, east, south, west) {
            ('|' | '7' | 'F', '-' | 'J' | '7', _, _) => 'L', // North to east
            ('|' | '7' | 'F', _, '|' | 'J' | 'L', _) => '|', // North to south
            ('|' | '7' | 'F', _, _, '-' | 'F' | 'L') => 'J', // North to west
            (_, '-' | 'J' | '7', '|' | 'J' | 'L', _) => 'F', // East to south
            (_, '-' | 'J' | '7', _, '-' | 'F' | 'L') => '-', // East to west
            (_, _, '|' | 'J' | 'L', '-' | 'F' | 'L') => '7', // South to west
            _ => '.', // There's no valid path to this tile...
        }
    }
}

fn start_delta(c: &char) -> Pos {
    match c {
        '|' => Pos(1, 0),             // Pretend we entered from north
        'L' | 'F' => Pos(0, -1),      // Pretend we entered from east
        '-' | 'J' | '7' => Pos(0, 1), // Pretend we entered from west
        _ => Pos(0, 0),
    }
}

struct AreaFinder<'a> {
    values: &'a HashMap<Pos, char>,
    loop_tiles: &'a HashSet<Pos>,
    visited: HashSet<Pos>,
}

impl AreaFinder<'_> {
    fn find(&mut self, pos: Pos) -> Option<(usize, bool)> {
        if self.visited.contains(&pos) {
            None
        } else if !self.values.contains_key(&pos) {
            Some((0, true))
        } else if self.loop_tiles.contains(&pos) {
            Some((0, false))
        } else {
            self.visited.insert(pos);

            let mut area = if self.values[&pos] == '.' { 1 } else { 0 };
            let mut outside = false;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if let Some((v, o)) = self.find(Pos(pos.0 + dy, pos.1 + dx)) {
                        area += v;
                        outside = outside || o;
                    }
                }
            }

            Some((area, outside))
        }
    }
}

fn next_delta(c: char, delta: Pos) -> Option<Pos> {
    match c {
        '|' if delta == Pos(1, 0) => Some(Pos(1, 0)),
        '|' if delta == Pos(-1, 0) => Some(Pos(-1, 0)),

        '-' if delta == Pos(0, 1) => Some(Pos(0, 1)),
        '-' if delta == Pos(0, -1) => Some(Pos(0, -1)),

        'L' if delta == Pos(1, 0) => Some(Pos(0, 1)),
        'L' if delta == Pos(0, -1) => Some(Pos(-1, 0)),

        'J' if delta == Pos(1, 0) => Some(Pos(0, -1)),
        'J' if delta == Pos(0, 1) => Some(Pos(-1, 0)),

        '7' if delta == Pos(-1, 0) => Some(Pos(0, -1)),
        '7' if delta == Pos(0, 1) => Some(Pos(1, 0)),

        'F' if delta == Pos(-1, 0) => Some(Pos(0, 1)),
        'F' if delta == Pos(0, -1) => Some(Pos(1, 0)),

        _ => None,
    }
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let lines = util::non_empty_lines(s).collect_vec();

        let mut values = HashMap::new();
        let mut start = Pos(0, 0);

        for row in 0..lines.len() {
            let line = &lines[row].chars().collect_vec();
            for col in 0..line.len() {
                let c = line[col];
                values.insert(Pos(row as i32, col as i32), line[col]);

                if c == 'S' {
                    start = Pos(row as i32, col as i32);
                }
            }
        }

        let mut map = Map {
            values,
            start,
            lines: lines,
        };

        // Replace the start char
        map.values.insert(map.start, map.start_char());

        Ok(map)
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Pos(i32, i32);

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------
