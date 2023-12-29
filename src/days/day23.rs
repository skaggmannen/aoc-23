use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

use crate::util::{self};

extern crate itertools;
extern crate num;

pub fn part1(input: &str) -> Result<String> {
    let grid = util::non_empty_lines(input)
        .map(|s| s.chars().collect_vec())
        .collect_vec();

    let start = (0i32, 1i32);
    let end = ((grid.len() - 1) as i32, (grid[0].len() - 2) as i32);

    let graph = build_graph(&grid, &start, &end, true);

    Ok(format!(
        "{}",
        dfs(&graph, &start, &end, &mut HashSet::new())
    ))
}

#[test]
fn test_part1() {
    assert_eq!("94", part1(TEST_INPUT).unwrap());
}

pub fn part2(input: &str) -> Result<String> {
    let grid = util::non_empty_lines(input)
        .map(|s| s.chars().collect_vec())
        .collect_vec();

    let start = (0i32, 1i32);
    let end = ((grid.len() - 1) as i32, (grid[0].len() - 2) as i32);

    let graph = build_graph(&grid, &start, &end, false);

    Ok(format!(
        "{}",
        dfs(&graph, &start, &end, &mut HashSet::new())
    ))
}

#[test]
fn test_part2() {
    assert_eq!("154", part2(TEST_INPUT).unwrap());
}

#[cfg(test)]
const TEST_INPUT: &str = r"
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

type Pos = (i32, i32);
type Graph = HashMap<Pos, HashMap<Pos, i32>>;

fn build_graph(grid: &Vec<Vec<char>>, start: &Pos, end: &Pos, slippery_slopes: bool) -> Graph {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut points = vec![*start, *end];

    // Find all intersection points
    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == '#' {
                continue;
            }

            let mut neighbors = 0;
            for (next_row, next_col) in [
                (r as i32 - 1, c as i32),
                (r as i32 + 1, c as i32),
                (r as i32, c as i32 - 1),
                (r as i32, c as i32 + 1),
            ] {
                if next_row >= 0
                    && next_row < height
                    && next_col >= 0
                    && next_col < width
                    && grid[next_row as usize][next_col as usize] != '#'
                {
                    neighbors += 1;
                }
            }
            if neighbors >= 3 {
                points.push((r as i32, c as i32))
            }
        }
    }

    let mut graph: Graph = points.iter().map(|&p| (p, HashMap::new())).collect();

    // Build the adjacency list for the intersection points.
    for &(start_row, start_col) in points.iter() {
        let mut stack = vec![(0, start_row, start_col)];
        let mut seen = HashSet::new();
        seen.insert((start_row, start_col));

        while let Some((n, r, c)) = stack.pop() {
            if n != 0 && points.contains(&(r, c)) {
                graph
                    .get_mut(&(start_row, start_col))
                    .unwrap()
                    .insert((r, c), n);
                continue;
            }

            for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                match grid[r as usize][c as usize] {
                    '#' => continue,
                    '^' if slippery_slopes && (dr, dc) != (-1, 0) => continue,
                    'v' if slippery_slopes && (dr, dc) != (1, 0) => continue,
                    '<' if slippery_slopes && (dr, dc) != (0, -1) => continue,
                    '>' if slippery_slopes && (dr, dc) != (0, 1) => continue,
                    _ => {}
                }

                let next_row = r + dr;
                let next_col = c + dc;

                if seen.contains(&(next_row, next_col)) {
                    continue;
                }

                if next_row >= 0 && next_row < height && next_col >= 0 && next_col < width {
                    stack.push((n + 1, next_row, next_col));
                    seen.insert((next_row, next_col));
                }
            }
        }
    }

    graph
}

fn dfs(graph: &Graph, p: &Pos, end: &Pos, seen: &mut HashSet<Pos>) -> i32 {
    if p == end {
        return 0;
    }

    let mut m = i32::MIN;

    seen.insert(*p);
    for (next, n) in graph.get(p).unwrap().iter() {
        if !seen.contains(next) {
            m = max(m, n + dfs(graph, next, end, seen));
        }
    }
    seen.remove(p);

    m
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------
