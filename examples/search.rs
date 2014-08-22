#![feature(macro_rules, default_type_params, phase)]

#[phase(plugin, link)] extern crate graph;

use std::collections::HashMap;
use graph::algorithms::dfs;
use graph::algorithms::bfs;
use graph::graph::AdjListGraph;
use graph::graph::output_graphviz;

fn main() {
    // Construct graph
    let mut g: AdjListGraph = AdjListGraph::new();
    let e = edges!(0 => 3, 3 => 2, 3 => 1, 1 => 4, 4 => 2);
    g.add_edges(e);

    // Calculate distances with DFS
    let mut dist: HashMap<uint, uint> = HashMap::new();
    dfs(&g, |node: uint, parent: Option<uint>| {
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
    bfs(&g, |node: uint, parent: Option<uint>| {
        match parent {
            None => { dist.insert(node, 0); },
            Some(p) => {
                let new_dist = dist.find(&p).unwrap() + 1;
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
