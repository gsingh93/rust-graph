#![feature(macro_rules, default_type_params, phase)]

#[phase(plugin, link)] extern crate graph;

use graph::algorithms::Weight;
use graph::algorithms::prim;
use graph::graph::AdjListGraph;
use graph::graph::output_graphviz;
use std::cmp::{Ord, Ordering};
use std::default::Default;

#[deriving(Clone, Eq, PartialEq, PartialOrd, Show)]
struct Edge {
    weight: int
}

impl Edge {
    fn new(weight: int) -> Edge {
        Edge { weight: weight }
    }
}

impl Weight for Edge {
    fn weight(&self) -> int {
        self.weight
    }
}

impl Default for Edge {
    fn default() -> Edge {
        Edge { weight: Default::default() }
    }
}

impl Ord for Edge {
    // Flip the ordering so it works with a max heap
    fn cmp(&self, other: &Edge) -> Ordering {
        if self.weight > other.weight {
            Greater
        } else if self.weight < other.weight {
            Less
        } else {
            Equal
        }
    }
}

fn main() {
    // Construct graph
    let mut g: AdjListGraph<(), Edge> = AdjListGraph::new();
    g.add_vertices(vec![0, 1, 2, 3, 4]);
    let e = edges!(0 => 3 => Edge::new(4), 3 => 2 => Edge::new(2),
                   3 => 1 => Edge::new(-1), 1 => 4 => Edge::new(1),
                   4 => 2 => Edge::new(5));
    g.add_edges_with_prop(e);

    // Prim's Algorithm
    let mst = prim(&g);
    output_graphviz(&mst, "prim-mst.dot");
}
