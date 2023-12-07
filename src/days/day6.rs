extern crate itertools;

use std::ops::Range;

use itertools::Itertools;

use crate::util;

pub fn part1(input: &str) -> Result<String> {
    let (times, distances) = util::non_empty_lines(input)
        .map(|s| parse_line(&s))
        .collect_tuple()
        .unwrap();

    let races = times.into_iter().zip(distances.into_iter());

    let score = races
        .map(|(time, record_distance)| {
            println!("({}, {})", time, record_distance);
            let mut results = Vec::new();
            for i in 1..time {
                let distance = calc_distance(time, i);
                if distance > record_distance {
                    println!("  -> {}", i);
                    results.push(distance);
                }
            }

            results.len()
        })
        .fold(1, |acc, v| acc * v);

    Ok(format!("{}", score))
}

#[test]
fn test_part1() {
    assert_eq!("288", part1(TEST_INPUT).unwrap());
}

pub fn part2(input: &str) -> Result<String> {
    let lines = util::non_empty_lines(input).collect_vec();

    let time = lines[0]
        .strip_prefix("Time:")
        .unwrap()
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();

    let distance = lines[1]
        .strip_prefix("Distance:")
        .unwrap()
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();

    let (left, right) = calc_breakpoints(time, distance);

    Ok(format!("{}", right - left + 1))
}

#[test]
fn test_part2() {
    assert_eq!("71503", part2(TEST_INPUT).unwrap());
}

fn parse_line(s: &str) -> Vec<i64> {
    s.split(" ")
        .filter_map(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        })
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect()
}

fn calc_distance(time_limit: i64, windup: i64) -> i64 {
    let speed = windup;
    let time_left = time_limit - windup;

    speed * time_left
}

#[test]
fn test_calc_distance() {
    assert_eq!(0, calc_distance(7, 0));
    assert_eq!(6, calc_distance(7, 1));
    assert_eq!(10, calc_distance(7, 2));
    assert_eq!(12, calc_distance(7, 3));
    assert_eq!(12, calc_distance(7, 4));
    assert_eq!(10, calc_distance(7, 5));
    assert_eq!(6, calc_distance(7, 6));
    assert_eq!(0, calc_distance(7, 7));
}

// d = w * (t - w) = w*t - w^2
// [d > D] => [w*t - w^2 > D] =>[ w*t - w^2 - D > 0]
// w*t - w^2 - D = 0 => w^2 - w*t + D = 0 => w = t/2 +- sqrt(4*D - t^2)/2
fn calc_breakpoints(time: i64, distance: i64) -> (i64, i64) {
    let max = find_max(time, 1..time);
    let left = bisect_left(time, distance, 1..max);
    let right = bisect_right(time, distance, max..time);

    return (left, right);
}

fn find_max(time: i64, r: Range<i64>) -> i64 {
    if r.start == r.end - 1 {
        return r.start;
    }

    let w = (r.start + r.end) / 2;
    let d = calc_distance(time, w);
    if w > 0 {
        let before = calc_distance(time, w - 1);
        if before > d {
            return find_max(time, r.start..w);
        }
    }
    if w < r.end - 1 {
        let after = calc_distance(time, w + 1);
        if after > d {
            return find_max(time, w..r.end);
        }
    }

    return w;
}

fn bisect_left(time: i64, distance: i64, r: Range<i64>) -> i64 {
    if r.start == r.end - 1 {
        let d = calc_distance(time, r.start);
        if d > distance {
            return r.start;
        } else {
            return r.start + 1;
        }
    }

    let w = (r.start + r.end) / 2;
    let d = w * (time - w);

    if d > distance {
        return bisect_left(time, distance, r.start..w);
    } else {
        return bisect_left(time, distance, w..r.end);
    }
}

fn bisect_right(time: i64, distance: i64, r: Range<i64>) -> i64 {
    if r.start == r.end - 1 {
        let d = calc_distance(time, r.start);
        if d > distance {
            return r.start;
        } else {
            return r.start - 1;
        }
    }

    let w = (r.start + r.end) / 2;
    let d = w * (time - w);

    if d > distance {
        return bisect_right(time, distance, w..r.end);
    } else {
        return bisect_right(time, distance, r.start..w);
    }
}

#[test]
fn test_breakpoints() {
    assert_eq!((2, 5), calc_breakpoints(7, 9));
    assert_eq!((4, 11), calc_breakpoints(15, 40));
    assert_eq!((11, 19), calc_breakpoints(30, 200));
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------

#[cfg(test)]
const TEST_INPUT: &str = "
Time:      7  15   30
Distance:  9  40  200
";
