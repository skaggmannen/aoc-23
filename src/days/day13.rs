extern crate itertools;
extern crate num;

use itertools::Itertools;

use crate::util;

pub fn part1(input: &str) -> Result<String> {
    let patterns = input.split("\n\n");

    let mut score = 0;

    for p in patterns {
        let lines = util::non_empty_lines(p)
            .map(|l| l.chars().collect())
            .collect_vec();

        let v_count = find_reflection(&lines, 99999999);
        let h_count = find_reflection(&transpose(&lines), 99999999);

        score += h_count + 100 * v_count;
    }

    Ok(score.to_string())
}

#[test]
fn test_part1() {
    assert_eq!("405", part1(TEST_INPUT).unwrap());
}

pub fn part2(input: &str) -> Result<String> {
    let patterns = input.split("\n\n");

    let mut score = 0;

    for p in patterns {
        println!("{p}");
        println!();

        let mut lines = util::non_empty_lines(p)
            .map(|l| l.chars().collect())
            .collect_vec();

        let h_a = find_reflection(&mut lines, 99999999);
        let h_b = find_reflection_with_smudge(&mut lines, h_a);

        if h_b != 0 {
            score += 100 * h_b;
        } else {
            let v_a = find_reflection(&mut transpose(&lines), 99999999);
            let v_b = find_reflection_with_smudge(&mut transpose(&lines), v_a);

            assert!(v_b != 0);

            score += v_b;
        };
    }

    Ok(score.to_string())
}

#[test]
fn test_part2() {
    assert_eq!("400", part2(TEST_INPUT).unwrap());
}

#[cfg(test)]
const TEST_INPUT: &str = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

fn find_reflection_with_smudge(lines: &mut [Vec<char>], ignore: usize) -> usize {
    for row in 0..lines.len() {
        for col in 0..lines[row].len() {
            // Try and change this tile
            if lines[row][col] == '#' {
                lines[row][col] = '.';
            } else {
                lines[row][col] = '#';
            }

            // Check if there is a reflection
            let reflection = find_reflection(lines, ignore);

            // Restore the smudge
            if lines[row][col] == '#' {
                lines[row][col] = '.';
            } else {
                lines[row][col] = '#';
            }

            if reflection != 0 && reflection != ignore {
                return reflection;
            }
        }
    }

    0
}

fn find_reflection(lines: &[Vec<char>], ignore: usize) -> usize {
    for (i, _) in lines.iter().enumerate() {
        if i+1 == ignore {
            continue;
        }

        if i == lines.len() - 1 {
            break;
        }

        let mut left = i;
        let mut right = i + 1;

        loop {
            if lines[left] != lines[right] {
                // There was a mismatch, so this was not a reflection point.
                break;
            }

            if left == 0 {
                // We reached the end on the left hand side without finding a
                // mismatch. This was a reflection point!
                return i + 1;
            } else if right == lines.len() - 1 {
                // We reached the end on the right hand side without finding a
                // mismatch. This was a reflection point!
                return i + 1;
            }

            left -= 1;
            right += 1;
        }
    }

    0
}

#[test]
fn test_find_reflection() {
    assert_eq!(
        4,
        find_reflection(
            &[
                "#...##..#".chars().collect(),
                "#....#..#".chars().collect(),
                "..##..###".chars().collect(),
                "#####.##.".chars().collect(),
                "#####.##.".chars().collect(),
                "..##..###".chars().collect(),
                "#....#..#".chars().collect(),
            ],
            9999999
        )
    );

    assert_eq!(
        5,
        find_reflection(
            &transpose(&vec![
                "#.##..##.".chars().collect(),
                "..#.##.#.".chars().collect(),
                "##......#".chars().collect(),
                "##......#".chars().collect(),
                "..#.##.#.".chars().collect(),
                "..##..##.".chars().collect(),
                "#.#.##.#.".chars().collect(),
            ]),
            9999999
        )
    );
}

fn transpose(v: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut transposed = Vec::new();

    for s in v {
        for (i, &c) in s.iter().enumerate() {
            if i >= transposed.len() {
                transposed.push(Vec::new());
            }

            transposed[i].push(c);
        }
    }

    transposed
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------
