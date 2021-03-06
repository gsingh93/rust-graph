#[macro_use]
extern crate rust_graph as graph;

use graph::algorithms::connected_components;
use graph::graph::AdjListGraph;

fn main() {
    // Construct graph
    let mut g: AdjListGraph<(), ()> = AdjListGraph::new(false);
    let e = edges!(0 => 3, 3 => 2, 3 => 1, 1 => 4, 4 => 2);
    g.add_edges(e);

    connected_components(&g);
    //mst.output_graphviz("prim-mst.dot");

    // TODO: Strongly connected_components
}
