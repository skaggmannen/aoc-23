use std::collections::HashMap;

use itertools::Itertools;

use crate::util;

extern crate itertools;
extern crate num;

pub fn part1(input: &str) -> Result<String> {
    let tiles = util::non_empty_lines(input)
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let map = Map { tiles };

    let starting_point = Crucible {
        pos: Pos(0, 0),
        direction: Direction::None,
        heat_loss: 0,
        max_steps: 3,
        min_steps: 1,
    };

    let mut cache = HashMap::new();

    let winner = map.move_crucible(starting_point, &mut cache).unwrap();

    Ok(winner.heat_loss.to_string())
}

#[test]
fn test_part1() {
    assert_eq!("102", part1(TEST_INPUT).unwrap());
}

pub fn part2(input: &str) -> Result<String> {
    let tiles = util::non_empty_lines(input)
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let map = Map { tiles };

    let starting_point = Crucible {
        pos: Pos(0, 0),
        direction: Direction::None,
        heat_loss: 0,
        max_steps: 10,
        min_steps: 4,
    };

    let mut cache = HashMap::new();

    let winner = map.move_crucible(starting_point, &mut cache).unwrap();

    Ok(winner.heat_loss.to_string())
}

#[test]
fn test_part2() {
    assert_eq!("94", part2(TEST_INPUT).unwrap());
}

#[test]
fn test_part2_2() {
    assert_eq!("71", part2(TEST_INPUT_2).unwrap());
}

#[cfg(test)]
const TEST_INPUT: &str = r"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

#[cfg(test)]
const TEST_INPUT_2: &str = r"
111111111111
999999991991
999999991991
999999991991
999999999991
";

struct Map {
    tiles: Vec<Vec<u32>>,
}

impl Map {
    fn move_crucible(
        &self,
        starting_point: Crucible,
        cache: &mut HashMap<CacheKey, u32>,
    ) -> Option<Crucible> {
        let mut paths = Vec::new();
        paths.push(starting_point);

        while let Some(c) = paths.pop() {
            let cache_key = CacheKey(c.pos, c.direction);
            if let Some(&heat_loss) = cache.get(&cache_key) {
                if heat_loss <= c.heat_loss {
                    // If we have already found a cheaper path to this destination we
                    // can safely skip this one.
                    continue;
                }
            }

            cache.insert(cache_key, c.heat_loss);

            let Pos(row, col) = c.pos;
            if row == self.tiles.len() - 1 && col == self.tiles[0].len() - 1 {
                if c.can_stop() {
                    return Some(c);
                }
            }

            for d in c.valid_directions() {
                if let Some(next_pos) = c.pos.walk(d) {
                    if next_pos.0 >= self.tiles.len() || next_pos.1 >= self.tiles[0].len() {
                        continue;
                    }

                    paths.push(Crucible {
                        pos: next_pos,
                        direction: d,
                        heat_loss: c.heat_loss + self.heat_loss(&next_pos),
                        max_steps: c.max_steps,
                        min_steps: c.min_steps,
                    })
                }
            }

            paths.sort_by(|a, b| b.heat_loss.cmp(&a.heat_loss));
        }

        None
    }

    fn heat_loss(&self, pos: &Pos) -> u32 {
        self.tiles[pos.0][pos.1]
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Crucible {
    pos: Pos,
    direction: Direction,
    heat_loss: u32,
    max_steps: u32,
    min_steps: u32,
}
impl Crucible {
    fn valid_directions(&self) -> Vec<Direction> {
        let mut directions = Vec::new();

        use Direction::*;
        match self.direction {
            None => {
                directions.push(Up(1));
                directions.push(Down(1));
                directions.push(Left(1));
                directions.push(Right(1));
            }
            Up(steps) => {
                if steps >= self.min_steps {
                    directions.push(Left(1));
                    directions.push(Right(1));
                }

                if steps < self.max_steps {
                    directions.push(Up(steps + 1));
                }
            }
            Down(steps) => {
                if steps >= self.min_steps {
                    directions.push(Left(1));
                    directions.push(Right(1));
                }
                if steps < self.max_steps {
                    directions.push(Down(steps + 1));
                }
            }
            Left(steps) => {
                if steps >= self.min_steps {
                    directions.push(Up(1));
                    directions.push(Down(1));
                }
                if steps < self.max_steps {
                    directions.push(Left(steps + 1));
                }
            }
            Right(steps) => {
                if steps >= self.min_steps {
                    directions.push(Up(1));
                    directions.push(Down(1));
                }
                if steps < self.max_steps {
                    directions.push(Right(steps + 1));
                }
            }
        };

        directions
    }

    fn can_stop(&self) -> bool {
        use Direction::*;
        match self.direction {
            None => true,
            Up(steps) => steps >= self.min_steps,
            Down(steps) => steps >= self.min_steps,
            Left(steps) => steps >= self.min_steps,
            Right(steps) => steps >= self.min_steps,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    None,
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pos(usize, usize);

impl Pos {
    fn walk(self, d: Direction) -> Option<Pos> {
        let Pos(row, col) = self;

        use Direction::*;
        match d {
            None => Some(Pos(row, col)),
            Down(_) => Some(Pos(row + 1, col)),
            Right(_) => Some(Pos(row, col + 1)),
            Up(_) => {
                if row > 0 {
                    Some(Pos(row - 1, col))
                } else {
                    Option::None
                }
            }
            Left(_) => {
                if col > 0 {
                    Some(Pos(row, col - 1))
                } else {
                    Option::None
                }
            }
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct CacheKey(Pos, Direction);

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------
