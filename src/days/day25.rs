use rand::{thread_rng, Rng};
use std::collections::HashSet;

use itertools::Itertools;

use crate::util::{self};

extern crate itertools;
extern crate num;

pub fn part1(input: &str) -> Result<String> {
    let lines = util::non_empty_lines(input).collect_vec();
    let graph = parse_graph(&lines);

    // This uses Karger's Algorithm to find a minimal cut.
    //
    // Since this is a Monte Carlo algorithm, it will sometimes not find the
    // most optimal cut, which means we might have to run it a few times to
    // find a cut that only removes 3 edges.
    let mut contracted = karger_min_cut(&graph);
    while contracted.edges.len() > 3 {
        println!(
            "Contracted graph has {} edges, retrying...",
            contracted.edges.len()
        );
        contracted = karger_min_cut(&graph);
    }

    let score = contracted
        .vertices
        .iter()
        .fold(1, |acc, v| v.split(",").count() * acc);

    Ok(format!("{}", score))
}

#[test]
fn test_part1() {
    assert_eq!("54", part1(TEST_INPUT).unwrap());
}

pub fn part2(_input: &str) -> Result<String> {
    todo!()
}

#[test]
fn test_part2() {
    assert_eq!("2", part1(TEST_INPUT).unwrap());
}

#[cfg(test)]
const TEST_INPUT: &str = r"
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";

fn parse_graph<'a>(lines: &Vec<String>) -> Graph {
    let mut vertices = HashSet::new();
    let mut edges = HashSet::new();

    for line in lines {
        let (from, dsts) = line.split_once(": ").unwrap();

        vertices.insert(from.to_string());
        for d in dsts.split(" ") {
            vertices.insert(d.to_string());
            edges.insert(Edge::new(from, d));
        }
    }

    Graph {
        vertices: HashSet::from_iter(vertices),
        edges: Vec::from_iter(edges),
    }
}

// a class to represent a unweighted edge in graph
#[derive(Clone, Eq, PartialEq, Hash)]
struct Edge {
    src: String,
    dst: String,
}

impl Edge {
    fn new<'a>(src: &str, dst: &str) -> Edge {
        if src > dst {
            Edge {
                src: dst.to_string(),
                dst: src.to_string(),
            }
        } else {
            Edge {
                src: src.to_string(),
                dst: dst.to_string(),
            }
        }
    }
}

#[derive(Clone)]
struct Graph {
    vertices: HashSet<String>,
    edges: Vec<Edge>,
}

fn karger_min_cut(graph: &Graph) -> Graph {
    let mut graph = graph.clone();

    while graph.vertices.len() > 2 {
        // Choose a random edge to contract.
        let i = thread_rng().gen_range(0..graph.edges.len());
        let edge = &graph.edges[i].clone();

        // Create a new super node.
        let super_node = format!("{},{}", edge.src, edge.dst);
        graph.vertices.insert(super_node.clone());

        // Remove the old nodes.
        graph.vertices.remove(&edge.src);
        graph.vertices.remove(&edge.dst);

        graph.edges = graph
            .edges
            .iter_mut()
            // Redirect all edges to the new super node.
            .map(|e| {
                if e.src == edge.src || e.src == edge.dst {
                    e.src = super_node.clone();
                }
                if e.dst == edge.src || e.dst == edge.dst {
                    e.dst = super_node.clone();
                }

                e.clone()
            })
            // Eliminate any self references.
            .filter(|e| e.src == e.dst)
            .collect_vec();
    }

    graph
}

// -------------------------------------

type Error = String;
type Result<T> = std::result::Result<T, Error>;

// -------------------------------------
