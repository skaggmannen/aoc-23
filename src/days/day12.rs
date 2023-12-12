extern crate itertools;
extern crate num;

use std::collections::HashMap;

use itertools::Itertools;

use crate::util;

pub fn part1(input: &str) -> Result<String> {
    let lines = util::non_empty_lines(input).collect_vec();
    let mut cache = HashMap::new();

    let score = lines
        .iter()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(conditions, s)| {
            (
                conditions,
                s.split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect_vec(),
            )
        })
        .map(|(conditions, groups)| count_alternatives(&conditions, &groups, &mut cache))
        .sum::<usize>();

    Ok(score.to_string())
}

#[test]
fn test_part1() {
    assert_eq!("21", part1(TEST_INPUT).unwrap());
}

pub fn part2(input: &str) -> Result<String> {
    let lines = util::non_empty_lines(input).collect_vec();
    let mut cache = HashMap::new();

    let score = lines
        .iter()
        .map(|l| l.split_once(" ").unwrap())
        .map(|(conditions, s)| {
            return (
                [conditions].repeat(5).join("?").to_string(),
                [s].repeat(5).join(",").to_string(),
            );
        })
        .map(|(conditions, s)| {
            (
                conditions,
                s.split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect_vec(),
            )
        })
        .map(|(conditions, groups)| count_alternatives(&conditions, &groups, &mut cache))
        .sum::<usize>();

    Ok(score.to_string())
}

#[test]
fn test_part2() {
    assert_eq!("525152", part2(TEST_INPUT).unwrap());
}

#[cfg(test)]
const TEST_INPUT: &str = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

fn count_alternatives(
    conditions: &str,
    groups: &[usize],
    cache: &mut HashMap<(String, Vec<usize>), usize>,
) -> usize {
    if conditions.is_empty() {
        // We're out of springs so there should be no more groups left
        if groups.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    if groups.is_empty() {
        // There should be no more damaged springs left
        if conditions.contains('#') {
            return 0;
        } else {
            return 1;
        }
    }

    let cache_key = (conditions.to_string(), groups.to_vec());

    // Check if there's already a result for this setup in the cache
    if let Some(&result) = cache.get(&cache_key) {
        return result;
    }

    let mut result = 0;
    if conditions.starts_with(".") || conditions.starts_with("?") {
        // Assume that the spring was undamaged and count the number of valid
        // alternatives.
        result += count_alternatives(&conditions[1..], groups, cache)
    }

    if conditions.starts_with("#") || conditions.starts_with("?") {
        // Assume that the spring was damaged and count the number of valid
        // alternatives.
        if groups[0] <= conditions.len() && !(&conditions[..groups[0]]).contains(".") {
            if groups[0] == conditions.len() {
                // The group consumes the remaining string, do a recursive call
                // to check the end condition.
                result += count_alternatives("", &groups[1..], cache);
            } else if conditions.chars().nth(groups[0]).unwrap() != '#' {
                // The group successfully matches the start of the string.
                // Consume and check the remainder.
                result += count_alternatives(&conditions[groups[0] + 1..], &groups[1..], cache)
            }
        }
    }

    // Update the cache with the result so we do not have to calculate it again
    cache.insert(cache_key, result);

    result
}

#[test]
fn test_count_alternatives() {
    assert_eq!(
        1,
        count_alternatives("???.###", &[1, 1, 3], &mut HashMap::new())
    );
    assert_eq!(
        4,
        count_alternatives(".??..??...?##.", &[1, 1, 3], &mut HashMap::new())
    );
    assert_eq!(
        1,
        count_alternatives("?#?#?#?#?#?#?#?", &[1, 3, 1, 6], &mut HashMap::new())
    );
    assert_eq!(
        1,
        count_alternatives("????.#...#...", &[4, 1, 1], &mut HashMap::new())
    );
    assert_eq!(
        4,
        count_alternatives("????.######..#####.", &[1, 6, 5], &mut HashMap::new())
    );
    assert_eq!(
        10,
        count_alternatives("?###????????", &[3, 2, 1], &mut HashMap::new())
    );
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------
