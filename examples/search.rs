#[macro_use]
extern crate graph;

use std::collections::HashMap;
use graph::algorithms::{bfs, dfs, DFSVisitor};
use graph::graph::{AdjListGraph, output_graphviz};

struct Visitor {
    dist: HashMap<usize, usize>
}

impl DFSVisitor for Visitor {
    fn visit(&mut self, node: usize, parent: Option<usize>) {
        match parent {
            None => { self.dist.insert(node, 0); },
            Some(p) => {
                let new_dist = *self.dist.get(&p).unwrap() + 1;
                self.dist.insert(node, new_dist);
            }
        }
    }
}

fn main() {
    // Construct graph
    let mut g: AdjListGraph = AdjListGraph::new(true);
    let e = edges!(0 => 3, 3 => 2, 3 => 1, 1 => 4, 4 => 2);
    g.add_edges(e);

    // Calculate distances with DFSp
    let mut v = Visitor { dist: HashMap::new() };
    dfs(&g, &mut v);
    println!("DFS");
    for (i, d) in v.dist.iter() {
        println!("{}: {}", i, d);
    }

    // Calculate distances with BFS
    let mut dist: HashMap<usize, usize> = HashMap::new();
    bfs(&g, |&mut: node: usize, parent: Option<usize>| {
        match parent {
            None => { dist.insert(node, 0); },
            Some(p) => {
                let new_dist = *dist.get(&p).unwrap() + 1;
                dist.insert(node, new_dist);
            }
        }
    }, 0);
    println!("BFS");
    for (i, d) in dist.iter() {
        println!("{}: {}", i, d);
    }

    // Output GraphViz Dot file
    output_graphviz(&g, "graph.dot");
}
