use itertools::Itertools;

use crate::util;

extern crate itertools;
extern crate num;

pub fn part1(input: &str) -> Result<String> {
    let lines = util::non_empty_lines(input).map(|s| parse_part1(&s));

    let (mut x, mut y, mut l) = (0, 0, 2);
    let mut vertices = vec![(x, y)];

    for (direction, length) in lines {
        match direction.as_str() {
            "U" => y += length,
            "D" => y -= length,
            "L" => x -= length,
            "R" => x += length,
            _ => {}
        }

        l += length;
        vertices.push((x, y))
    }

    vertices.push(vertices[0]);

    // This is the "shoelace formula" for calculating the area of a polygon
    //
    // 2A = (x1*y2 - x2*y1) + (x2*y3 - x3*y2) + ... + (xn*y1 - x1*yn)
    let area: i64 = vertices
        .windows(2)
        .map(|v| v[0].0 * v[1].1 - v[1].0 * v[0].1)
        .sum();

    Ok(((area.abs() + l) / 2).to_string())
}

#[test]
fn test_part1() {
    assert_eq!("62", part1(TEST_INPUT).unwrap());
}

pub fn part2(input: &str) -> Result<String> {
    let lines = util::non_empty_lines(input).map(|s| parse_part2(&s));

    let (mut x, mut y, mut l) = (0, 0, 2);
    let mut vertices = vec![(x, y)];

    for (direction, length) in lines {
        match direction.as_str() {
            "U" => y += length,
            "D" => y -= length,
            "L" => x -= length,
            "R" => x += length,
            _ => {}
        }

        l += length;
        vertices.push((x, y))
    }

    vertices.push(vertices[0]);

    // This is the "shoelace formula" for calculating the area of a polygon
    //
    // 2A = (x1*y2 - x2*y1) + (x2*y3 - x3*y2) + ... + (xn*y1 - x1*yn)
    let area = vertices
        .windows(2)
        .map(|v| v[0].0 * v[1].1 - v[1].0 * v[0].1)
        .sum::<i64>();

    Ok(((area.abs() + l) / 2).to_string())
}

#[test]
fn test_part2() {
    assert_eq!("952408144115", part2(TEST_INPUT).unwrap());
}

#[cfg(test)]
const TEST_INPUT: &str = r"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

fn parse_part1(s: &str) -> (String, i64) {
    let parts = s.split(" ").collect_vec();

    (parts[0].to_string(), parts[1].parse().unwrap())
}

fn parse_part2(s: &str) -> (String, i64) {
    let encoded = s.split(" ").collect_vec()[2]
        .trim_start_matches("(#")
        .trim_end_matches(")");

    (
        match &encoded[5..] {
            "0" => "R".to_string(),
            "1" => "D".to_string(),
            "2" => "L".to_string(),
            "3" => "U".to_string(),
            _ => "".to_string(),
        },
        i64::from_str_radix(&encoded[..5], 16).unwrap(),
    )
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------
