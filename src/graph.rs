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
        // Only construct a new adjacency list if the node did not already exist
        if self.nodes.insert(n, v) {
            self.adjList.insert(n, Vec::new());
        }
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

    pub fn copy_node_to(&self, other: &mut AdjListGraph<V, E>, v: uint) {
        other.add_node_with_prop(v, self.node_prop(v));
    }

    pub fn copy_edge_to(&self, other: &mut AdjListGraph<V, E>, from: uint,
                     to: uint) {
        self.copy_node_to(other, from);
        self.copy_node_to(other, to);
        other.add_edge_with_prop(from, to, self.edge_prop(from, to));
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

macro_rules! add_node (
    ($m:ident, $g:ident, $n:expr, $p:expr) => ({
        $m.insert($n, $p);
        $g.add_node_with_prop($n, $p);
    });
    ($m:ident, $g:ident, $n:expr) => ({
        $m.insert($n, 0);
        $g.add_node($n);
    })
)

macro_rules! add_edge (
    ($em:ident, $nm: ident, $g:ident, $f:expr, $t:expr, $p:expr) => ({
        $em.insert(($f, $t), $p);
        $g.add_edge_with_prop($f, $t, $p);
    });
    ($em:ident, $nm:ident, $g:ident, $f:expr, $t:expr) => ({
        if !$nm.contains_key(&$f) {
            $nm.insert($f, 0);
        }
        if !$nm.contains_key(&$t) {
            $nm.insert($t, 0);
        }
        $em.insert(($f, $t), 0);
        $g.add_edge($f, $t);
    })
)

#[cfg(test)]
fn check<V: Clone + Default + Ord + Show,
         E: Clone + Default + Ord + Show>(g: &AdjListGraph<V, E>,
                                          nodes: HashMap<uint, V>,
                                          edges: HashMap<(uint, uint), E>) {
    assert_eq!(nodes.len(), g.size());

    assert_eq!(nodes.len(), g.nodes_iter().count());
    for n in g.nodes_iter() {
        assert!(nodes.contains_key(n));
        assert_eq!(nodes[*n], g.node_prop(*n));
    }

    assert_eq!(edges.len(), g.edges_iter().count());
    for e in g.edges_iter() {
        let (u, v) = *e;
        assert!(edges.contains_key(e));
        assert_eq!(edges[*e], g.edge_prop(u, v));
    }
    // TODO: Adjacency
}

#[test]
fn graph_creation_test() {
    let mut nodes = HashMap::new();
    let mut edges = HashMap::new();
    let mut g = AdjListGraph::new();
    add_node!(nodes, g, 0, 1u);
    check(&g, nodes.clone(), edges.clone());
    add_node!(nodes, g, 2, 2);
    check(&g, nodes.clone(), edges.clone());
    add_node!(nodes, g, 3);
    check(&g, nodes.clone(), edges.clone());
    add_edge!(edges, nodes, g, 0, 2, 6u);
    check(&g, nodes.clone(), edges.clone());
    add_edge!(edges, nodes, g, 2, 0);
    check(&g, nodes.clone(), edges.clone());
    add_edge!(edges, nodes, g, 3, 0, 7);
    check(&g, nodes.clone(), edges.clone());
    add_edge!(edges, nodes, g, 0, 3);
    check(&g, nodes.clone(), edges.clone());
    add_edge!(edges, nodes, g, 1, 2);
    check(&g, nodes.clone(), edges.clone());

    // Test duplicate additions
    add_edge!(edges, nodes, g, 3, 0, 7);
    check(&g, nodes.clone(), edges.clone());

    *edges.get_mut(&(3, 0)) = 6;
    add_edge!(edges, nodes, g, 3, 0, 6);
    check(&g, nodes.clone(), edges.clone());

    add_edge!(edges, nodes, g, 0, 1);
    check(&g, nodes.clone(), edges.clone());

    *nodes.get_mut(&0) = 2;
    add_node!(nodes, g, 0, 2);
    check(&g, nodes.clone(), edges.clone());
}

