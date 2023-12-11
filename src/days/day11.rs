extern crate itertools;
extern crate num;

use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::util;

pub fn part1(input: &str) -> Result<String> {
    let map = Map::parse(input, 1);

    let mut distances = HashMap::<(Pos, Pos), usize>::new();

    for &pos in map.galaxies.iter() {
        for &other in map.galaxies.iter() {
            if pos == other {
                continue;
            }

            if distances.contains_key(&(pos, other)) || distances.contains_key(&(other, pos)) {
                continue;
            }

            let Pos(row, col) = pos;
            let Pos(other_row, other_col) = other;

            let row_distance = (other_row as i32 - row as i32).abs() as usize;
            let col_distance = (other_col as i32 - col as i32).abs() as usize;

            distances.insert((pos, other), row_distance + col_distance);
        }
    }

    Ok(distances.values().sum::<usize>().to_string())
}

#[test]
fn test_part1() {
    assert_eq!("374", part1(TEST_INPUT).unwrap());
}

pub fn part2(input: &str) -> Result<String> {
    let map = Map::parse(input, 1_000_000);

    Ok(map.distances().values().sum::<usize>().to_string())
}

#[test]
fn test_part2() {
    let map = Map::parse(TEST_INPUT, 10);
    assert_eq!("1030", map.distances().values().sum::<usize>().to_string());
}

#[test]
fn test_part2_2() {
    let map = Map::parse(TEST_INPUT, 100);
    assert_eq!("8410", map.distances().values().sum::<usize>().to_string());
}

#[cfg(test)]
const TEST_INPUT: &str = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

struct Map {
    galaxies: HashSet<Pos>,
}

impl Map {
    fn parse(s: &str, expansion_factor: usize) -> Map {
        let lines = util::non_empty_lines(s).collect_vec();

        let mut rows_with_galaxies = HashSet::new();
        let mut cols_with_galaxies = HashSet::new();

        for (i, row) in lines.iter().enumerate() {
            for (j, c) in row.chars().enumerate() {
                if c == '#' {
                    rows_with_galaxies.insert(i);
                    cols_with_galaxies.insert(j);
                }
            }
        }

        let mut row_expansion = HashMap::new();
        let mut col_expansion = HashMap::new();

        let mut curr_row_expansion = 0;

        for (i, row) in lines.iter().enumerate() {
            let mut curr_col_expansion = 0;

            for (j, _) in row.chars().enumerate() {
                if !cols_with_galaxies.contains(&j) {
                    curr_col_expansion += expansion_factor - 1;
                }

                col_expansion.insert(j, curr_col_expansion);
            }

            if !rows_with_galaxies.contains(&i) {
                curr_row_expansion += expansion_factor - 1;
            }

            row_expansion.insert(i, curr_row_expansion);
        }

        let mut galaxies = HashSet::new();
        for (i, row) in lines.into_iter().enumerate() {
            for (j, c) in row.chars().enumerate() {
                if c == '#' {
                    galaxies.insert(Pos(i + row_expansion[&i], j + col_expansion[&j]));
                }
            }
        }

        Map { galaxies }
    }

    fn distances(&self) -> HashMap<(Pos, Pos), usize> {
        let mut distances = HashMap::<(Pos, Pos), usize>::new();

        for &pos in self.galaxies.iter() {
            for &other in self.galaxies.iter() {
                if pos == other {
                    continue;
                }

                if distances.contains_key(&(pos, other)) || distances.contains_key(&(other, pos)) {
                    continue;
                }

                let Pos(row, col) = pos;
                let Pos(other_row, other_col) = other;

                let row_distance = (other_row as i64 - row as i64).abs() as usize;
                let col_distance = (other_col as i64 - col as i64).abs() as usize;

                distances.insert((pos, other), row_distance + col_distance);
            }
        }

        distances
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Pos(usize, usize);

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------
