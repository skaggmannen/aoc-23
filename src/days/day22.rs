use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

use crate::util;

extern crate itertools;
extern crate num;

pub fn part1(input: &str) -> Result<String> {
    let mut bricks = util::non_empty_lines(input)
        .enumerate()
        .map(|(i, s)| (i, s.parse::<Brick>().unwrap()))
        .collect_vec();

    settle(&mut bricks);

    let mut count = 0;

    for (id, _) in bricks.iter() {
        let mut is_required = false;
        for (_, b) in bricks.iter() {
            if b.supported_by.contains(id) && b.supported_by.len() == 1 {
                is_required = true;
            }
        }

        if !is_required {
            count += 1
        }
    }

    Ok(format!("{count}"))
}

#[test]
fn test_part1() {
    assert_eq!("5", part1(TEST_INPUT).unwrap());
}

pub fn part2(input: &str) -> Result<String> {
    let mut bricks = util::non_empty_lines(input)
        .enumerate()
        .map(|(i, s)| (i, s.parse::<Brick>().unwrap()))
        .collect_vec();

    settle(&mut bricks);

    let count = bricks
        .iter()
        .map(|(id, _)| find_falling_bricks(id, &bricks))
        .sum::<usize>();

    Ok(format!("{count}"))
}

#[test]
fn test_part2() {
    assert_eq!("7", part2(TEST_INPUT).unwrap());
}

#[cfg(test)]
const TEST_INPUT: &str = r"
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

#[derive(Clone, Debug)]
struct Brick {
    start: Pos,
    stop: Pos,
    supported_by: HashSet<usize>,
}

impl<'a> Brick {
    fn collides_with(&self, other: &Self) -> bool {
        let overlap_x = self.stop.x >= other.start.x && self.start.x <= other.stop.x;
        let overlap_y = self.stop.y >= other.start.y && self.start.y <= other.stop.y;

        overlap_x && overlap_y
    }
}

impl<'a> FromStr for Brick {
    type Err = Error;

    fn from_str(s: &str) -> Result<Brick> {
        let (s1, s2) = s.split_once("~").unwrap();
        let (start, stop): (Pos, Pos) = (s1.parse()?, s2.parse()?);

        assert!(start.x <= stop.x);
        assert!(start.y <= stop.y);
        assert!(start.z <= stop.z);

        Ok(Brick {
            start,
            stop,
            supported_by: HashSet::new(),
        })
    }
}

fn find_falling_bricks(id: &usize, bricks: &[(usize, Brick)]) -> usize {
    let mut disintegrated = HashSet::new();
    disintegrated.insert(*id);

    for (i, b) in bricks
        .iter()
        .sorted_by(|(_, a), (_, b)| a.start.z.cmp(&b.start.z))
    {
        if b.supported_by.intersection(&disintegrated).count() > 0
            && b.supported_by.difference(&disintegrated).count() == 0
        {
            disintegrated.insert(*i);
        }
    }

    disintegrated.len() - 1
}

#[derive(Copy, Clone, Debug)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl FromStr for Pos {
    type Err = Error;

    fn from_str(s: &str) -> Result<Pos> {
        let (x, y, z) = s
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        Ok(Pos { x, y, z })
    }
}

fn settle(bricks: &mut [(usize, Brick)]) {
    let mut settled: Vec<(usize, Brick)> = Vec::new();

    for (i, b) in bricks
        .iter_mut()
        .sorted_by(|(_, a), (_, b)| a.start.z.cmp(&b.start.z))
    {
        let mut stop_z = None;

        for (id, o) in settled
            .iter()
            .sorted_by(|(_, a), (_, b)| a.stop.z.cmp(&b.stop.z))
            .rev()
        {
            if o.collides_with(&b) {
                if let Some(z) = stop_z {
                    if o.stop.z == z {
                        // We found another support on the same z-level as the
                        // previous one.
                        b.supported_by.insert(*id);
                    } else {
                        break;
                    }
                } else {
                    stop_z = Some(o.stop.z);
                    b.supported_by.insert(*id);
                }
            }
        }

        let old_z = b.start.z;
        b.start = Pos {
            z: if let Some(z) = stop_z { z + 1 } else { 1 },
            ..b.start
        };
        b.stop = Pos {
            z: if let Some(z) = stop_z {
                b.stop.z - (old_z - (z + 1))
            } else {
                b.stop.z - (old_z - 1)
            },
            ..b.stop
        };

        settled.push((*i, b.clone()));
    }
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------
