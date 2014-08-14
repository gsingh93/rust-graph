#![feature(macro_rules)]

extern crate graph;

use std::collections::HashMap;
use graph::AdjListGraph;

#[deriving(Clone)]
struct Node;

#[deriving(Clone)]
struct Edge;

macro_rules! edges (
    ($($u:expr => $v:expr => $e:expr),+) => ({
        let mut edges: Vec<(uint, uint, Edge)> = Vec::new();
        $(
            edges.push(($u, $v, $e));
        )+
        edges
    });
    ($($u:expr => $v:expr => $e:expr),+,) => (edges!($($u => $v => $e),+));
)

fn main() {
    // Construct graph
    let mut g: AdjListGraph<Node, Edge> = AdjListGraph::new();
    g.add_vertices(vec![(0, Node), (1, Node), (2, Node), (3, Node), (4, Node)]);
    let e = edges!(0 => 3 => Edge, 3 => 2 => Edge, 3 => 1 => Edge,
                   1 => 4 => Edge, 4 => 2 => Edge);
    g.add_edges(e);

    // Calculate distances with DFS
    let mut dist: HashMap<uint, uint> = HashMap::new();
    g.dfs(|node: uint, parent: Option<uint>| {
        match parent {
            None => { dist.insert(node, 0); },
            Some(p) => {
                let new_dist = dist.find(&p).unwrap() + 1;
                dist.insert(node, new_dist);
            }
        }
    });
    println!("DFS");
    for (i, d) in dist.iter() {
        println!("{}: {}", i, d);
    }

    // Calculate distances with BFS
    let mut dist: HashMap<uint, uint> = HashMap::new();
    g.bfs(|node: uint, parent: Option<uint>| {
        match parent {
            None => { dist.insert(node, 0); },
            Some(p) => {
                let new_dist = dist.find(&p).unwrap() + 1;
                dist.insert(node, new_dist);
            }
        }
    });
    println!("BFS");
    for (i, d) in dist.iter() {
        println!("{}: {}", i, d);
    }

    // Output GraphViz Dot file
    g.output_graphviz("graph.dot");
}
