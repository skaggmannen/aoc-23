extern crate itertools;
extern crate num;

use std::{num::ParseIntError, str::FromStr};

use itertools::Itertools;

use crate::util;

pub fn part1(input: &str) -> Result<String> {
    let lines = util::non_empty_lines(input);

    let history = lines.map(|s| s.parse::<History>());

    let extrapolations: Result<i32> = history.map_ok(|h| h.extrapolate()).sum();

    Ok(extrapolations?.to_string())
}

#[test]
fn test_part1() {
    assert_eq!("114", part1(TEST_INPUT).unwrap());
}

pub fn part2(input: &str) -> Result<String> {
    let lines = util::non_empty_lines(input);

    let history = lines.map(|s| s.parse::<History>());

    let extrapolations: Result<i32> = history.map_ok(|h| h.extrapolate_history()).sum();

    Ok(extrapolations?.to_string())
}

#[test]
fn test_part2() {
    assert_eq!("2", part2(TEST_INPUT).unwrap());
}

struct History {
    values: Vec<i32>,
}

impl History {
    fn extrapolate(&self) -> i32 {
        return do_it(&self.values);
    }
    fn extrapolate_history(&self) -> i32 {
        return do_it_backwards(&self.values);
    }
}

fn do_it(vec: &Vec<i32>) -> i32 {
    if vec.iter().all(|&v| v == 0) {
        return 0;
    }

    let mut next_level = Vec::new();
    for i in 0..vec.len() - 1 {
        let left = vec[i];
        let right = vec[i + 1];

        next_level.push(right - left);
    }

    let incr = do_it(&next_level);

    vec[vec.len() - 1] + incr
}

fn do_it_backwards(vec: &Vec<i32>) -> i32 {
    if vec.iter().all(|&v| v == 0) {
        return 0;
    }

    let mut next_level = Vec::new();
    for i in 0..vec.len() - 1 {
        let left = vec[i];
        let right = vec[i + 1];

        next_level.push(right - left);
    }

    let incr = do_it_backwards(&next_level);

    vec[0] - incr
}

impl FromStr for History {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let values: Result<Vec<_>> = s
            .trim()
            .split(" ")
            .map(|s| s.parse().map_err(|err: ParseIntError| err.to_string()))
            .collect();

        Ok(History { values: values? })
    }
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------

#[cfg(test)]
const TEST_INPUT: &str = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
