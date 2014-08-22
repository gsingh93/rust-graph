#![macro_escape]

use std::collections::HashMap;
use std::collections::hashmap::Keys;
use std::default::Default;
use std::fmt::Show;
use std::io::File;
use std::slice::Items;

#[macro_export]
macro_rules! edges (
    ($($u:expr => $v:expr),+) => ({
        let mut edges: Vec<(uint, uint)> = Vec::new();
        $(
            edges.push(($u, $v));
        )+
        edges
    });
    ($($u:expr => $v:expr),+,) => (edges!($($u => $v),+));
    ($($u:expr => $v:expr => $e:expr),+) => ({
        let mut edges: Vec<(uint, uint, Edge)> = Vec::new();
        $(
            edges.push(($u, $v, $e));
        )+
        edges
    });
    ($($u:expr => $v:expr => $e:expr),+,) => (edges!($($u => $v => $e),+));
)

pub struct AdjListGraph<V = (), E = ()> {
    adjList: HashMap<uint, Vec<uint>>,
    nodes: HashMap<uint, V>,
    edges: HashMap<(uint, uint), E>,
}

impl<V: Clone + Default, E: Clone + Default + Ord> AdjListGraph<V, E> {
    pub fn new() -> AdjListGraph<V, E> {
        AdjListGraph { adjList: HashMap::new(), nodes: HashMap::new(),
                       edges: HashMap::new() }
    }

    pub fn size(&self) -> uint {
        self.nodes.len()
    }

    pub fn add_node(&mut self, n: uint) {
        self.add_node_with_prop(n, Default::default());
    }

    pub fn add_node_with_prop(&mut self, n: uint, v: V) {
        self.nodes.insert(n, v);
        self.adjList.insert(n, Vec::new());
    }

    pub fn add_nodes(&mut self, vertices: Vec<uint>) {
        for i in vertices.move_iter() {
            self.add_node(i);
        }
    }

    pub fn add_nodes_with_prop(&mut self, vertices: Vec<(uint, V)>) {
        for (i, v) in vertices.move_iter() {
            self.add_node_with_prop(i, v);
        }
    }

    pub fn add_edge(&mut self, from: uint, to: uint) {
        self.add_edge_with_prop(from, to, Default::default());
    }

    pub fn add_edge_with_prop(&mut self, from: uint, to: uint, e: E) {
        if !self.nodes.contains_key(&from) {
            self.add_node(from);
        }
        if !self.nodes.contains_key(&to) {
            self.add_node(to);
        }

        self.adjList.get_mut(&from).push(to);
        self.edges.insert((from, to), e);
    }

    pub fn add_edges(&mut self, edges: Vec<(uint, uint)>) {
        for (from, to) in edges.move_iter() {
            self.add_edge(from, to);
        }
    }

    pub fn add_edges_with_prop(&mut self, edges: Vec<(uint, uint, E)>) {
        for (from, to, e) in edges.move_iter() {
            self.add_edge_with_prop(from, to, e);
        }
    }

    pub fn node_prop(&self, node: uint) -> V {
        self.nodes[node].clone()
    }

    pub fn edge_prop(&self, from: uint, to: uint) -> E {
        self.edges[(from, to)].clone()
    }

    pub fn nodes_iter<'a>(&'a self) -> Keys<'a, uint, V> {
        self.nodes.keys()
    }

    pub fn edges_iter<'a>(& 'a self) -> Keys<'a, (uint, uint), E> {
        self.edges.keys()
    }

    pub fn adj_iter<'a>(&'a self, from: uint) -> Items<'a, uint> {
        self.adjList[from].iter()
    }
}

pub fn output_graphviz<V: Clone + Default,
                       E: Clone + Default + Ord + Show>(g: &AdjListGraph<V, E>,
                                                        filename: &str) {
    let path = Path::new(filename);
    let mut file = match File::create(&path) {
        Ok(f)  => f,
        Err(e) => fail!("Error opening file: {}", e)
    };
    file.write_str("digraph G {\n").ok();

    for from in g.nodes_iter() {
        for to in g.adj_iter(*from) {
            file.write_str(
                format!("\t{} -> {};\n", from, to).as_slice()).ok();
        }
    }

    file.write_str("}\n").ok();
}
