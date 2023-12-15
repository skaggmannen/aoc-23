extern crate itertools;
extern crate num;

use std::{collections::HashMap, fmt};

use itertools::Itertools;

use crate::util;

pub fn part1(input: &str) -> Result<String> {
    let lines = util::non_empty_lines(input).collect_vec();

    let mut platform = Platform::from(&lines);
    platform.tilt_north();

    Ok(platform.northern_load().to_string())
}

#[test]
fn test_part1() {
    assert_eq!("136", part1(TEST_INPUT).unwrap());
}

pub fn part2(input: &str) -> Result<String> {
    let lines = util::non_empty_lines(input).collect_vec();

    let mut platform = Platform::from(&lines);
    let mut cache = HashMap::new();
    let mut iterations = Vec::new();
    let mut cycle = 0;

    loop {
        if cache.contains_key(&platform.board) {
            break;
        } else {
            cache.insert(platform.board.clone(), cycle);
        }

        cycle += 1;
        platform.tilt_north();
        platform.tilt_west();
        platform.tilt_south();
        platform.tilt_east();

        iterations.push(platform.board.clone());
    }

    let first_repetition = cache.get(&platform.board).unwrap();
    let repetition_len = cycle - first_repetition;
    let offset = (1_000_000_000 - first_repetition) % repetition_len;
    let final_board = &iterations[first_repetition + offset - 1];

    platform.board = final_board.clone();

    Ok(platform.northern_load().to_string())
}

#[test]
fn test_part2() {
    assert_eq!("64", part2(TEST_INPUT).unwrap());
}

#[cfg(test)]
const TEST_INPUT: &str = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

struct Platform {
    width: usize,
    height: usize,
    board: Vec<Vec<char>>,
}

impl Platform {
    fn from(input: &[String]) -> Platform {
        let mut board = Vec::new();
        for (i, row) in input.iter().enumerate() {
            for c in row.chars() {
                if i >= board.len() {
                    board.push(Vec::new());
                }

                board[i].push(c);
            }
        }

        Platform {
            width: board[0].len(),
            height: board.len(),
            board: board,
        }
    }

    fn tilt_north(&mut self) {
        for row in 1..self.height {
            for col in 0..self.width {
                if self.board[row][col] == 'O' {
                    self.roll_north(row, col);
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for col in (0..self.width - 1).rev() {
            for row in 0..self.height {
                if self.board[row][col] == 'O' {
                    self.roll_east(row, col);
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for row in (0..self.height - 1).rev() {
            for col in 0..self.width {
                if self.board[row][col] == 'O' {
                    self.roll_south(row, col);
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for col in 1..self.width {
            for row in 0..self.height {
                if self.board[row][col] == 'O' {
                    self.roll_west(row, col);
                }
            }
        }
    }

    fn roll_north(&mut self, row: usize, col: usize) {
        let mut stop = row;

        for i in (0..row).rev() {
            let c = self.board[i][col];
            if c == '#' || c == 'O' {
                break;
            }

            stop = i;
        }

        if stop == row {
            return;
        }

        self.board[row][col] = '.';
        self.board[stop][col] = 'O';
    }

    fn roll_east(&mut self, row: usize, col: usize) {
        let mut stop = col;

        for j in col + 1..self.width {
            let c = self.board[row][j];
            if c == '#' || c == 'O' {
                break;
            }

            stop = j;
        }

        self.board[row][col] = '.';
        self.board[row][stop] = 'O';
    }

    fn roll_south(&mut self, row: usize, col: usize) {
        let mut stop = row;

        for i in row + 1..self.height {
            let c = self.board[i][col];
            if c == '#' || c == 'O' {
                break;
            }

            stop = i;
        }

        self.board[row][col] = '.';
        self.board[stop][col] = 'O';
    }

    fn roll_west(&mut self, row: usize, col: usize) {
        let mut stop = col;

        for j in (0..col).rev() {
            let c = self.board[row][j];
            if c == '#' || c == 'O' {
                break;
            }

            stop = j;
        }

        self.board[row][col] = '.';
        self.board[row][stop] = 'O';
    }

    fn northern_load(&self) -> usize {
        let mut load = 0;

        for (i, row) in self.board.iter().enumerate() {
            for &c in row {
                if c == 'O' {
                    load += self.height - i;
                }
            }
        }

        load
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.board.iter() {
            for c in row {
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------
