#[macro_use]
extern crate graph;

use graph::algorithms::kruskal;
use graph::algorithms::prim;
use graph::algorithms::Weight;
use graph::graph::AdjListGraph;
use graph::graph::output_graphviz;
use std::cmp::Ord;
use std::cmp::Ordering::{self, Greater, Less, Equal};
use std::default::Default;

#[derive(Clone, Eq, PartialEq, PartialOrd, Show)]
struct Edge {
    weight: isize
}

impl Edge {
    fn new(weight: isize) -> Edge {
        Edge { weight: weight }
    }
}

impl Weight for Edge {
    fn weight(&self) -> isize {
        self.weight
    }

    fn set_weight(&mut self, weight: isize) {
        self.weight = weight;
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
    let mut g: AdjListGraph<(), Edge> = AdjListGraph::new(false);
    let e = edges!(0 => 3 => Edge::new(4), 3 => 2 => Edge::new(2),
                   3 => 1 => Edge::new(-1), 1 => 4 => Edge::new(1),
                   4 => 2 => Edge::new(5));
    g.add_edges_with_prop(e);

    // Prim's
    let mst = match prim(&g) {
        Err(e) => panic!(e),
        Ok(mst) => mst
    };
    output_graphviz(&mst, "prim-mst.dot");

    // Kruskal's
    let mst = match kruskal(&g) {
        Err(e) => panic!(e),
        Ok(mst) => mst
    };
    output_graphviz(&mst, "kruskal-mst.dot");
}
