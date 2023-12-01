use crate::util;

pub fn part1(input: &str) -> Result<String> {
    let lines = input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());

    let process = util::compose!(find_value);

    let score: u32 = lines.map(process).sum();

    Ok(format!("{}", score))
}

pub fn part2(input: &str) -> Result<String> {
    let lines = input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());

    let process = util::compose!(replace_digits, find_value);

    let score: u32 = lines.map(process).sum();

    Ok(format!("{}", score))
}

fn find_value(s: String) -> u32 {
    let digits = s
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>();

    10 * digits.first().unwrap() + digits.last().unwrap()
}

fn replace_digits(s: String) -> String {
    // Using the weird "one1one" format instead of just replacing with "1" handles
    // the edge cases like "twone", where the first digit should be "2" ("two|ne")
    // and the last should be "1" ("tw|one").
    //
    // Example:
    //    "twone" -> "2ne" -> (2, 2) => WRONG!
    //    "twone" -> "two2twone1one" -> (2, 1) => CORRECT!
    s.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six5six")
        .replace("seven", "seven7seven")
        .replace("eigth", "eigth8eigth")
        .replace("nine", "nine9nine")
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------

#[test]
fn test_part1() {
    assert_eq!("142", part1(TEST_INPUT).unwrap());
}

#[test]
fn test_part2() {
    assert_eq!("281", part2(TEST_INPUT_2).unwrap());
}

#[cfg(test)]
const TEST_INPUT: &str = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

#[cfg(test)]
const TEST_INPUT_2: &str = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
