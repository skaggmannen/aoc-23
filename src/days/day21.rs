use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use itertools::Itertools;

use crate::util;

extern crate itertools;
extern crate num;

pub fn part1(input: &str) -> Result<String> {
    let map: Map = input.parse()?;
    let result = map.walk(64);

    Ok(result.to_string())
}

#[test]
fn test_part1() {
    let map: Map = TEST_INPUT.parse().unwrap();
    let result = map.walk(6);
    assert_eq!(16, result);
}

pub fn part2(input: &str) -> Result<String> {
    let map: Map = input.parse()?;
    let steps = 26501365;
    let size = map.tiles.len();

    // This is the maximum number of maps we can traverse in a straight line
    // going up, down, left or right.
    //
    // This works because the input has an empty row and col in the exact middle
    // of the map, which allows straight traversal through each map.
    //
    // Since there is also an empty diagonal in the input between the
    // "entry points" on each side, this is also the exact number of grids we
    // can visit by going diagonally.
    let grid_width = steps / size - 1;

    // We can now calculate the number of "odd" grids we can fully visit. This
    // is easiest to see by drawing the grids and color the "odd" ones. It will
    // form a kind of diagonal square of colored grids.

    let odd_count = (grid_width / 2 * 2 + 1).pow(2);
    let even_count = ((grid_width + 1) / 2 * 2).pow(2);

    // So let's count the number of points we can visit in the odd and even
    // grids.
    let odd_points = map.walk_from(&map.start, size * 2 + 1);
    let even_points = map.walk_from(&map.start, size * 2);

    // Now we need to handle the "corner cases" at the outer points of the the
    // huge grid of grids. This is done by starting at the entry point of each
    // side of the grid and checking how many points we can reach from there.
    let steps = size - 1;
    let corners = [
        map.walk_from(&Pos(size as i32 - 1, map.start.1), steps), // Top
        map.walk_from(&Pos(map.start.0, 0), steps),               // Right
        map.walk_from(&Pos(0, map.start.1), steps),               // Bottom
        map.walk_from(&Pos(map.start.0, size as i32 - 1), steps), // Left
    ];

    // The last parts to cover are the "edges" of the reachable grids. They will
    // be partially covered in two different ways: one where we come from a
    // partially filled grid, giving a smaller amount of steps covering the edge
    // grid, and one where we come from a fully filled grid, giving us more
    // steps left to cover the edge.
    let steps = size / 2 - 1;
    let small_edges = [
        map.walk_from(&Pos(size as i32 - 1, 0), steps), // Top right
        map.walk_from(&Pos(size as i32 - 1, size as i32 - 1), steps), // Top left
        map.walk_from(&Pos(0, 0), steps),               // Bottom right
        map.walk_from(&Pos(0, size as i32 - 1), steps), // Bottom left
    ];

    let steps = (size * 3) / 2 - 1;
    let large_edges = [
        map.walk_from(&Pos(size as i32 - 1, 0), steps), // Top right
        map.walk_from(&Pos(size as i32 - 1, size as i32 - 1), steps), // Top left
        map.walk_from(&Pos(0, 0), steps),               // Bottom right
        map.walk_from(&Pos(0, size as i32 - 1), steps), // Bottom left
    ];

    let result = odd_count * odd_points
        + even_count * even_points
        + corners.iter().sum::<usize>()
        + (grid_width + 1) * small_edges.iter().sum::<usize>()
        + grid_width * large_edges.iter().sum::<usize>();

    Ok(result.to_string())
}

#[test]
fn test_part2() {
    let map: Map = TEST_INPUT.parse().unwrap();

    {
        let result = map.walk(6);
        assert_eq!(16, result);
    }
    {
        let result = map.walk(10);
        assert_eq!(50, result);
    }
    {
        let result = map.walk(50);
        assert_eq!(1594, result);
    }
    {
        let result = map.walk(100);
        assert_eq!(6536, result);
    }
    {
        let result = map.walk(500);
        assert_eq!(167004, result);
    }
    {
        let result = map.walk(1000);
        assert_eq!(668697, result);
    }
    {
        let result = map.walk(5000);
        assert_eq!(16733044, result);
    }
}

#[cfg(test)]
const TEST_INPUT: &str = r"
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

struct Map {
    start: Pos,
    tiles: Vec<Vec<char>>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Map> {
        let tiles = util::non_empty_lines(s)
            .map(|s| s.chars().collect_vec())
            .collect_vec();

        let mut start = Pos(0, 0);
        for (i, r) in tiles.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c == 'S' {
                    start = Pos(i as i32, j as i32);
                    break;
                }
            }
        }

        Ok(Map { start, tiles })
    }
}

impl Map {
    fn walk(&self, steps_left: usize) -> usize {
        self.walk_from(&self.start, steps_left)
    }

    fn walk_from(&self, pos: &Pos, steps_left: usize) -> usize {
        // If the number of steps is even we should count even tiles, otherwise
        // we count the odd ones.
        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();
        let mut result = 0;

        to_visit.push_back((*pos, steps_left));

        while let Some((pos, steps_left)) = to_visit.pop_front() {
            if visited.contains(&pos) {
                continue;
            }
            visited.insert(pos);

            if steps_left % 2 == 0 {
                result += 1;
            }

            if steps_left == 0 {
                continue;
            }

            let Pos(row, col) = pos;

            for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let i = row + dy;
                let j = col + dx;
                let size = self.tiles.len() as i32;

                if i < 0 || j < 0 || i >= size || j >= size {
                    // The gardener can't step outside the map
                    continue;
                }

                let c = self.tiles[i as usize][j as usize];
                if c != '#' {
                    to_visit.push_back((Pos(i, j), steps_left - 1));
                }
            }
        }

        result
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------
