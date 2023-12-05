extern crate itertools;

use std::ops::Range;

use itertools::Itertools;

pub fn part1(input: &str) -> Result<String> {
    let parts = input
        .split("\n\n")
        .filter_map(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        })
        .collect_vec();

    let almanac = parse_input(parts);

    let lowest = almanac
        .seeds
        .into_iter()
        .map(|v| almanac.seed_to_soil.resolve(v))
        .map(|v| almanac.soil_to_fertilizer.resolve(v))
        .map(|v| almanac.fertilizer_to_water.resolve(v))
        .map(|v| almanac.water_to_light.resolve(v))
        .map(|v| almanac.light_to_temp.resolve(v))
        .map(|v| almanac.temp_to_humidity.resolve(v))
        .map(|v| almanac.humidity_to_location.resolve(v))
        .min()
        .unwrap();

    Ok(format!("{}", lowest))
}

pub fn part2(input: &str) -> Result<String> {
    let parts = input
        .split("\n\n")
        .filter_map(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed)
            }
        })
        .collect_vec();

    let almanac = parse_input(parts);

    let loc_ranges = almanac
        .seed_ranges
        .into_iter()
        .map(|r| vec![r])
        .map(|rs| almanac.seed_to_soil.resolve_ranges(rs))
        .map(|rs| almanac.soil_to_fertilizer.resolve_ranges(rs))
        .map(|rs| almanac.fertilizer_to_water.resolve_ranges(rs))
        .map(|rs| almanac.water_to_light.resolve_ranges(rs))
        .map(|rs| almanac.light_to_temp.resolve_ranges(rs))
        .map(|rs| almanac.temp_to_humidity.resolve_ranges(rs))
        .map(|rs| almanac.humidity_to_location.resolve_ranges(rs))
        .fold(Vec::new(), |mut acc, mut rs| {
            acc.append(&mut rs);
            acc
        });

    let lowest = loc_ranges.iter().map(|r| r.start).min().unwrap();

    Ok(format!("{}", lowest))
}

fn parse_input(parts: Vec<&str>) -> Almanac {
    let seeds: Vec<u64> = parts[0]
        .split(" ")
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut seed_ranges = Vec::new();
    for i in 0..seeds.len() / 2 {
        let start = seeds[i * 2];
        let end = start + seeds[i * 2 + 1];
        seed_ranges.push(Range { start, end })
    }

    Almanac {
        seeds: seeds,
        seed_ranges: seed_ranges,
        seed_to_soil: parse_map(&parts[1]),
        soil_to_fertilizer: parse_map(&parts[2]),
        fertilizer_to_water: parse_map(&parts[3]),
        water_to_light: parse_map(&parts[4]),
        light_to_temp: parse_map(&parts[5]),
        temp_to_humidity: parse_map(&parts[6]),
        humidity_to_location: parse_map(&parts[7]),
    }
}

fn parse_map(input: &str) -> Map {
    let mut map = Map::new();

    for line in input.lines().skip(1) {
        let mut nbrs = line.split(" ").map(|s| s.parse().unwrap());

        let dst_start = nbrs.next().unwrap();
        let src_start = nbrs.next().unwrap();
        let count = nbrs.next().unwrap();

        map.entries.push(MapEntry {
            src: Range {
                start: src_start,
                end: src_start + count,
            },
            dst: Range {
                start: dst_start,
                end: dst_start + count,
            },
        })
    }

    return map;
}

struct Almanac {
    seeds: Vec<u64>,
    seed_ranges: Vec<Range<u64>>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temp: Map,
    temp_to_humidity: Map,
    humidity_to_location: Map,
}

struct Map {
    entries: Vec<MapEntry>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            entries: Vec::new(),
        }
    }

    pub fn resolve(&self, v: u64) -> u64 {
        for e in self.entries.iter() {
            if let Some(r) = e.map(v) {
                return r;
            }
        }

        return v;
    }

    pub fn resolve_ranges(&self, srcs: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut to_resolve = srcs.into_iter().collect_vec();
        to_resolve.sort_by(|a, b| a.start.cmp(&b.start));

        let mut resolved = Vec::new();

        while let Some(src) = to_resolve.pop() {
            let mut found_match = false;
            for e in self.entries.iter() {
                if let Some((r, mut non_match)) = e.map_range(&src) {
                    resolved.push(r);
                    to_resolve.append(&mut non_match);
                    found_match = true;
                    break;
                }
            }
            if !found_match {
                resolved.push(src);
            }
        }

        resolved.sort_by(|a, b| a.start.cmp(&b.start));

        return combine_ranges(resolved);
    }
}

#[derive(Debug)]
struct MapEntry {
    src: Range<u64>,
    dst: Range<u64>,
}

impl MapEntry {
    pub fn map_range(&self, r: &Range<u64>) -> Option<(Range<u64>, Vec<Range<u64>>)> {
        if let Some(overlapping) = overlapping(&self.src, &r) {
            let mut non_matching = Vec::new();

            if r.start != overlapping.start {
                non_matching.push(Range {
                    start: r.start,
                    end: overlapping.start,
                });
            }

            if r.end != overlapping.end {
                non_matching.push(Range {
                    start: overlapping.end,
                    end: r.end,
                });
            }

            Some((
                Range {
                    start: self.map(overlapping.start).unwrap(),
                    end: self.map(overlapping.end).unwrap(),
                },
                non_matching,
            ))
        } else {
            None
        }
    }

    fn map(&self, v: u64) -> Option<u64> {
        if v >= self.src.start && v <= self.src.end {
            let offset = v - self.src.start;
            Some(self.dst.start + offset)
        } else {
            None
        }
    }
}

fn overlapping(first: &Range<u64>, second: &Range<u64>) -> Option<Range<u64>> {
    if first.start >= second.end || first.end <= second.start {
        None
    } else {
        Some(Range {
            start: if first.start > second.start {
                first.start
            } else {
                second.start
            },
            end: if first.end < second.end {
                first.end
            } else {
                second.end
            },
        })
    }
}

fn combine_ranges(rs: Vec<Range<u64>>) -> Vec<Range<u64>> {
    let mut result = Vec::new();

    let mut curr: Option<Range<u64>> = None;
    for r in rs {
        if let Some(c) = curr {
            if r.start <= c.end {
                curr = Some(c.start..r.end);
            } else {
                result.push(c);
                curr = Some(r);
            }
        } else {
            curr = Some(r);
        }
    }

    if let Some(c) = curr {
        result.push(c);
    }

    return result;
}
// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------

#[test]
fn test_part1() {
    assert_eq!("35", part1(TEST_INPUT).unwrap());
}

#[test]
fn test_part2() {
    assert_eq!("46", part2(TEST_INPUT).unwrap());
}

#[cfg(test)]
const TEST_INPUT: &str = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
