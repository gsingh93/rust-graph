#![feature(macro_rules, default_type_params, phase)]

#[phase(plugin, link)] extern crate graph;

use graph::algorithms::scc;
use graph::graph::AdjListGraph;

fn main() {
    // Construct graph
    let mut g: AdjListGraph<(), ()> = AdjListGraph::new();
    g.add_vertices(vec![0, 1, 2, 3, 4]);
    let e = edges!(0 => 3, 3 => 2, 3 => 1, 1 => 4, 4 => 2);
    g.add_edges(e);

    scc(&g);
    //mst.output_graphviz("prim-mst.dot");
}
