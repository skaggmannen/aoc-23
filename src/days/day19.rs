use core::panic;
use std::collections::HashMap;

use itertools::Itertools;
use std::ops::RangeInclusive;

use crate::util;

extern crate itertools;
extern crate num;

pub fn part1(input: &str) -> Result<String> {
    let parts = input.split_once("\n\n").unwrap();

    let workflows = parse_workflows(parts.0);
    let parts = parse_parts(parts.1);

    let start = workflows.get("in").unwrap();

    let score: u64 = parts
        .iter()
        .map(|p| {
            let mut next = Some(start);
            while let Some(wf) = next {
                match wf.evaluate(p).as_str() {
                    "A" => return Some(p),
                    "R" => return None,
                    w => next = workflows.get(&w.to_owned()),
                }
            }

            None
        })
        .filter_map(|r| r)
        .map(|p| p.categories.values().sum::<u64>())
        .sum();

    Ok(score.to_string())
}

#[test]
fn test_part1() {
    assert_eq!("19114", part1(TEST_INPUT).unwrap());
}

pub fn part2(input: &str) -> Result<String> {
    let parts = input.split_once("\n\n").unwrap();

    let workflows = parse_workflows(parts.0);

    let acceptable_ranges = acceptable_values(&workflows);

    let score: u64 = acceptable_ranges
        .iter()
        .map(|ranges| {
            (ranges.x.end() - ranges.x.start() + 1)
                * (ranges.m.end() - ranges.m.start() + 1)
                * (ranges.a.end() - ranges.a.start() + 1)
                * (ranges.s.end() - ranges.s.start() + 1)
        })
        .sum();

    Ok(score.to_string())
}

#[test]
fn test_part2() {
    assert_eq!("167409079868000", part2(TEST_INPUT).unwrap());
}

#[cfg(test)]
const TEST_INPUT: &str = r"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn evaluate(&self, part: &Part) -> String {
        for rule in self.rules.iter() {
            if let Some(result) = rule.match_part(part) {
                return result;
            }
        }

        panic!("no matching rules!")
    }

    fn acceptable_values(
        &self,
        workflows: &HashMap<String, Workflow>,
        input: &Ranges,
    ) -> Vec<Ranges> {
        let mut result = Vec::new();

        let mut ranges = input.clone();

        for rule in self.rules.iter() {
            match rule {
                Rule::LessThan(category, value, r) => {
                    if !ranges.contains(category, value) {
                        continue;
                    }

                    // Create a new restricted range that lets us pass this
                    // rule.
                    let restricted = ranges.less_than(&category, value);

                    // Update the input to no longer include the range that made
                    // us select this path since the different paths are mutually
                    // exclusive.
                    ranges = ranges.greater_than_or_equal(&category, value);

                    if r == "A" {
                        // We have reached a point where the part is accepted.
                        // Remember the range that led us here.
                        result.push(restricted);
                    } else if r != "R" {
                        // Follow the reference using the restricted range.
                        let wf = workflows.get(r).unwrap();
                        result.extend(wf.acceptable_values(workflows, &restricted));
                    }
                }
                Rule::GreaterThan(category, value, r) => {
                    if !ranges.contains(category, value) {
                        continue;
                    }

                    // Create a new restricted range that lets us pass this
                    // rule.
                    let restricted = ranges.greater_than(&category, value);

                    // Update the input to no longer include the range that made
                    // us select this path since the different paths are mutually
                    // exclusive.
                    ranges = ranges.less_than_or_equal(&category, value);

                    if r == "A" {
                        // We have reached a point where the part is accepted.
                        // Remember the range that led us here.
                        result.push(restricted);
                    } else if r != "R" {
                        // Follow the reference using the restricted range.
                        let wf = workflows.get(r).unwrap();
                        result.extend(wf.acceptable_values(workflows, &restricted));
                    }
                }
                Rule::Default(r) => {
                    if r == "A" {
                        // All other parts would be accepted, so remember that
                        // choice as well.
                        result.push(ranges.clone());
                    } else if r != "R" {
                        // Follow the reference using the left over range.
                        let wf = workflows.get(r).unwrap();
                        result.extend(wf.acceptable_values(workflows, &ranges));
                    }
                }
            }
        }

        result
    }
}

fn acceptable_values(workflows: &HashMap<String, Workflow>) -> Vec<Ranges> {
    let start = &workflows["in"];
    let input = Ranges {
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
    };

    start.acceptable_values(workflows, &input)
}

#[derive(Clone)]
struct Ranges {
    x: RangeInclusive<u64>,
    m: RangeInclusive<u64>,
    a: RangeInclusive<u64>,
    s: RangeInclusive<u64>,
}
impl Ranges {
    fn less_than(&self, category: &String, value: &u64) -> Ranges {
        let mut r = self.clone();

        match category.as_str() {
            "x" => r.x = *r.x.start()..=*std::cmp::min(r.x.end(), &(value - 1)),
            "m" => r.m = *r.m.start()..=*std::cmp::min(r.m.end(), &(value - 1)),
            "a" => r.a = *r.a.start()..=*std::cmp::min(r.a.end(), &(value - 1)),
            "s" => r.s = *r.s.start()..=*std::cmp::min(r.s.end(), &(value - 1)),
            _ => panic!("unexpected category: {category}"),
        };

        r
    }

    fn less_than_or_equal(&self, category: &String, value: &u64) -> Ranges {
        let mut r = self.clone();

        match category.as_str() {
            "x" => r.x = *r.x.start()..=*std::cmp::min(r.x.end(), &(value)),
            "m" => r.m = *r.m.start()..=*std::cmp::min(r.m.end(), &(value)),
            "a" => r.a = *r.a.start()..=*std::cmp::min(r.a.end(), &(value)),
            "s" => r.s = *r.s.start()..=*std::cmp::min(r.s.end(), &(value)),
            _ => panic!("unexpected category: {category}"),
        };

        r
    }

    fn greater_than(&self, category: &String, value: &u64) -> Ranges {
        let mut r = self.clone();

        match category.as_str() {
            "x" => r.x = *std::cmp::max(r.x.start(), &(value + 1))..=*r.x.end(),
            "m" => r.m = *std::cmp::max(r.m.start(), &(value + 1))..=*r.m.end(),
            "a" => r.a = *std::cmp::max(r.a.start(), &(value + 1))..=*r.a.end(),
            "s" => r.s = *std::cmp::max(r.s.start(), &(value + 1))..=*r.s.end(),
            _ => panic!("unexpected category: {category}"),
        };

        r
    }

    fn greater_than_or_equal(&self, category: &String, value: &u64) -> Ranges {
        let mut r = self.clone();

        match category.as_str() {
            "x" => r.x = *std::cmp::max(r.x.start(), &value)..=*r.x.end(),
            "m" => r.m = *std::cmp::max(r.m.start(), &value)..=*r.m.end(),
            "a" => r.a = *std::cmp::max(r.a.start(), &value)..=*r.a.end(),
            "s" => r.s = *std::cmp::max(r.s.start(), &value)..=*r.s.end(),
            _ => panic!("unexpected category: {category}"),
        };

        r
    }

    fn contains(&self, category: &String, value: &u64) -> bool {
        match category.as_str() {
            "x" => value >= self.x.start() && value <= self.x.end(),
            "m" => value >= self.m.start() && value <= self.m.end(),
            "a" => value >= self.a.start() && value <= self.a.end(),
            "s" => value >= self.s.start() && value <= self.s.end(),
            _ => panic!("unexpected category: {category}"),
        }
    }
}

fn parse_workflows(input: &str) -> HashMap<String, Workflow> {
    util::non_empty_lines(input)
        .map(|s| {
            let name = &s[..s.find("{").unwrap()];
            let rules = (&s[s.find("{").unwrap() + 1..s.find("}").unwrap()])
                .split(",")
                .map(|s| parse_rule(s))
                .collect_vec();

            (name.to_owned(), Workflow { rules })
        })
        .collect()
}

struct Part {
    categories: HashMap<String, u64>,
}

fn parse_parts(input: &str) -> Vec<Part> {
    util::non_empty_lines(input)
        .map(|s| {
            let categories = s
                .split(",")
                .map(|s| {
                    let (category, value) = s
                        .trim_start_matches("{")
                        .trim_end_matches("}")
                        .split_once("=")
                        .unwrap();

                    (category.to_owned(), value.parse::<u64>().unwrap())
                })
                .collect();

            Part { categories }
        })
        .collect_vec()
}

enum Rule {
    LessThan(String, u64, String),
    GreaterThan(String, u64, String),
    Default(String),
}

impl Rule {
    fn match_part(&self, p: &Part) -> Option<String> {
        match self {
            Rule::LessThan(category, value, result) if p.categories[category] < *value => {
                Some(result.clone())
            }
            Rule::GreaterThan(category, value, result) if p.categories[category] > *value => {
                Some(result.clone())
            }
            Rule::Default(result) => Some(result.clone()),
            _ => None,
        }
    }
}

fn parse_rule(input: &str) -> Rule {
    if input.contains(">") {
        let category = &input[..input.find(">").unwrap()];
        let value: u64 = (&input[input.find(">").unwrap() + 1..input.find(":").unwrap()])
            .parse()
            .unwrap();
        let result = &input[input.find(":").unwrap() + 1..];

        return Rule::GreaterThan(category.to_owned(), value, result.to_string());
    } else if input.contains("<") {
        let category = &input[..input.find("<").unwrap()];
        let value: u64 = (&input[input.find("<").unwrap() + 1..input.find(":").unwrap()])
            .parse()
            .unwrap();
        let result = &input[input.find(":").unwrap() + 1..];

        return Rule::LessThan(category.to_owned(), value, result.to_string());
    } else {
        return Rule::Default(input.to_owned());
    }
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------
