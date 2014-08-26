#![macro_escape]

use std::collections::HashMap;
use std::collections::hashmap::Keys;
use std::fmt::{mod, Formatter, Show};
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
    adj_list: HashMap<uint, Vec<uint>>,
    nodes: HashMap<uint, Option<V>>,
    edges: HashMap<(uint, uint), Option<E>>,
    is_directed: bool
}

impl<V: Clone + PartialEq,
     E: Clone + PartialEq + Ord> PartialEq for AdjListGraph<V, E> {
    fn eq(&self, other: &AdjListGraph<V, E>) -> bool {
                use std::hash::Hash;
        if self.is_directed {
            if self.edges != other.edges {
                return false
            }
        } else {
            if self.edges.len() != other.edges.len() {
                return false
            }
            for &(u, v) in self.edges_iter() {
                let mut prop: Option<E>;
                if other.edges.contains_key(&(u, v)) {
                    prop = other.edge_prop(u, v);
                } else if other.edges.contains_key(&(v, u)) {
                    prop = other.edge_prop(u, v);
                } else {
                    return false;
                }
                if self.edge_prop(u, v) != prop {
                    return false
                }
            }
        }
        for u in self.nodes_iter() {
            if !vec_eq(self.adj_list[*u].as_slice(),
                       other.adj_list[*u].as_slice()) {
                return false;
            }
        }

        return self.nodes == other.nodes
            && self.is_directed == other.is_directed;

        fn vec_eq<T: Clone + Eq + Hash + PartialEq>(v1: &[T], v2: &[T]) -> bool {
            use std::collections::HashSet;
            let mut hm = HashSet::new();

            if v1.len() != v2.len() {
                return false
            }
            for e in v1.iter() {
                hm.insert((*e).clone());
            }
            for e in v2.iter() {
                if !hm.contains(e) {
                    return false;
                }
            }

            true
        }
    }
}

impl<V: Clone + Eq, E: Clone + Eq + Ord> Eq for AdjListGraph<V, E> {}

impl<V: Clone, E: Clone + Ord + Show> Show for AdjListGraph<V, E> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", graphviz(self))
    }
}

impl<V: Clone, E: Clone + Ord> AdjListGraph<V, E> {
    pub fn new(is_directed: bool) -> AdjListGraph<V, E> {
        AdjListGraph { adj_list: HashMap::new(), nodes: HashMap::new(),
                       edges: HashMap::new(), is_directed: is_directed }
    }

    pub fn size(&self) -> uint {
        self.nodes.len()
    }

    pub fn is_directed(&self) -> bool {
        self.is_directed
    }

    pub fn add_node(&mut self, n: uint) {
        self.add_node_internal(n, None);
    }

    pub fn add_node_with_prop(&mut self, n: uint, v: V) {
        self.add_node_internal(n, Some(v))
    }

    fn add_node_internal(&mut self, n: uint, v: Option<V>) {
        // Only construct a new adjacency list if the node did not already exist
        if self.nodes.insert(n, v) {
            self.adj_list.insert(n, Vec::new());
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

    pub fn contains_node(&self, node: uint) -> bool {
        self.nodes.contains_key(&node)
    }

    pub fn copy_node_to(&self, other: &mut AdjListGraph<V, E>, v: uint) {
        other.add_node_internal(v, self.node_prop(v));
    }

    pub fn add_edge(&mut self, from: uint, to: uint) {
        self.add_edge_internal(from, to, None);
    }

    pub fn add_edge_with_prop(&mut self, from: uint, to: uint, e: E) {
        self.add_edge_internal(from, to, Some(e));
    }

    fn add_edge_internal(&mut self, from: uint, to: uint, e: Option<E>) {
        if !self.nodes.contains_key(&from) {
            self.add_node(from);
        }
        if !self.nodes.contains_key(&to) {
            self.add_node(to);
        }

        assert!(self.nodes.contains_key(&from));
        assert!(self.nodes.contains_key(&to));

        self.adj_list.get_mut(&from).push(to);
        if !self.is_directed {
            self.adj_list.get_mut(&to).push(from);
        }
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

    pub fn contains_edge(&self, from: uint, to: uint) -> bool {
        self.edges.contains_key(&(from, to))
    }

    pub fn copy_edge_to(&self, other: &mut AdjListGraph<V, E>, from: uint,
                     to: uint) {
        self.copy_node_to(other, from);
        self.copy_node_to(other, to);
        other.add_edge_internal(from, to, self.edge_prop(from, to));
    }

    pub fn node_prop(&self, node: uint) -> Option<V> {
        if self.nodes.contains_key(&node) {
            self.nodes[node].clone()
        } else {
            fail!("Node doesn't exist, can't get property");
        }
    }

    pub fn edge_prop(&self, from: uint, to: uint) -> Option<E> {
        if self.edges.contains_key(&(from, to)) {
            self.edges[(from, to)].clone()
        } else if self.edges.contains_key(&(to, from)) {
            self.edges[(to, from)].clone()
        } else {
            fail!("Edge doesn't exist, can't get property");
        }
    }

    pub fn nodes_iter<'a>(&'a self) -> Keys<'a, uint, Option<V>> {
        self.nodes.keys()
    }

    pub fn edges_iter<'a>(& 'a self) -> Keys<'a, (uint, uint), Option<E>> {
        self.edges.keys()
    }

    pub fn adj_iter<'a>(&'a self, from: uint) -> Items<'a, uint> {
        if self.adj_list.contains_key(&from) {
            self.adj_list[from].iter()
        } else {
            assert!(!self.nodes.contains_key(&from));
            fail!("Node doesn't exist, can't get adjacency list")
        }
    }
}

pub fn graphviz<V: Clone, E: Clone + Ord + Show>(g: &AdjListGraph<V, E>)
                                                 -> String {
    let (s, arrow) = if g.is_directed {
        ("digraph", "->")
    } else {
        ("graph", "--")
    };
    let mut s = s.to_string();

    s.push_str(" G {\n");
    for from in g.nodes_iter() {
        for to in g.adj_iter(*from) {
            let label = match g.edge_prop(*from, *to) {
                None    => "".to_string(),
                Some(l) => format!("{}", l)
            };
            s.push_str(format!("\t{} {} {} [label='{}'];\n", from, arrow, to,
                               label).as_slice());
        }
    }
    s.push_str("}\n");

    s
}

pub fn output_graphviz<V: Clone,
                       E: Clone + Ord + Show>(g: &AdjListGraph<V, E>,
                                              filename: &str) {
    let path = Path::new(filename);
    let mut file = match File::create(&path) {
        Ok(f)  => f,
        Err(e) => fail!("Error opening file: {}", e)
    };
    file.write_str(graphviz(g).as_slice()).ok();
}

macro_rules! add_node (
    ($m:ident, $adj:ident, $g:ident, $n:expr, $p:expr) => ({
        if !$m.contains_key(&$n) {
            $adj.insert($n, Vec::new());
        }
        $m.insert($n, Some($p));
        $g.add_node_with_prop($n, $p);
    });
    ($m:ident, $adj:ident, $g:ident, $n:expr) => ({
        if !$m.contains_key(&$n) {
            $adj.insert($n, Vec::new());
        }
        $m.insert($n, None);
        $g.add_node($n);
    })
)

macro_rules! add_edge (
    ($em:ident, $nm: ident, $adj:ident, $g:ident, $f:expr, $t:expr,
     $p:expr) => ({
        if !$nm.contains_key(&$f) {
            $adj.insert($f, Vec::new());
            $nm.insert($f, None);
        }
        if !$nm.contains_key(&$t) {
            $adj.insert($t, Vec::new());
            $nm.insert($t, None);
        }
        $adj.get_mut(&$f).push($t);
        $em.insert(($f, $t), Some($p));
        $g.add_edge_with_prop($f, $t, $p);
    });
    ($em:ident, $nm:ident, $adj:ident, $g:ident, $f:expr, $t:expr) => ({
        if !$nm.contains_key(&$f) {
            $adj.insert($f, Vec::new());
            $nm.insert($f, None);
        }
        if !$nm.contains_key(&$t) {
            $adj.insert($t, Vec::new());
            $nm.insert($t, None);
        }
        $adj.get_mut(&$f).push($t);
        $em.insert(($f, $t), None);
        $g.add_edge($f, $t);
    })
)

#[cfg(test)]
fn check<V: Clone + Ord + Show,
         E: Clone + Ord + Show>(g: &AdjListGraph<V, E>,
                                nodes: &HashMap<uint, Option<V>>,
                                edges: &HashMap<(uint, uint), Option<E>>,
                                adj_list: &HashMap<uint, Vec<uint>>) {
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

    assert_eq!(g.nodes_iter().count(), adj_list.len());
    for u in g.nodes_iter() {
        assert_eq!(adj_list[*u], g.adj_iter(*u).map(|x| *x).collect());
    }
}

#[test]
fn graph_creation_test() {
    let mut nodes = HashMap::new();
    let mut edges = HashMap::new();
    let mut adj_list: HashMap<uint, Vec<uint>> = HashMap::new();
    let mut g = AdjListGraph::new(true);

    macro_rules! check_node(
        ($n:expr, $p:expr) => ({
            add_node!(nodes, adj_list, g, $n, $p);
            check(&g, &nodes, &edges, &adj_list);
        });
        ($n:expr) => ({
            add_node!(nodes, adj_list, g, $n);
            check(&g, &nodes, &edges, &adj_list);
        })
    );
    macro_rules! check_edge(
        ($f:expr, $t:expr, $p:expr) => ({
            add_edge!(edges, nodes, adj_list, g, $f, $t, $p);
            check(&g, &nodes, &edges, &adj_list);
        });
        ($f:expr, $t:expr) => ({
            add_edge!(edges, nodes, adj_list, g, $f, $t);
            check(&g, &nodes, &edges, &adj_list);
        })
    );

    check_node!(0, 1u);
    check_node!(2, 2);
    check_node!(3);

    check_edge!(0, 2, 6u);
    check_edge!(2, 0);
    check_edge!(3, 0, 7);
    check_edge!(0, 3);
    check_edge!(1, 2);

    // Duplicate edge
    check_edge!(3, 0, 7);

    // Change edge data
    *edges.get_mut(&(3, 0)) = Some(6);
    check_edge!(3, 0, 6);

    // Duplicate node
    check_node!(0, 1);

    // Change node data
    *nodes.get_mut(&0) = Some(2);
    check_node!(0, 2);
}

#[test]
fn graph_copy_test() {
    let mut nodes = HashMap::new();
    let mut edges = HashMap::new();
    let mut adj_list = HashMap::new();
    let mut g = AdjListGraph::new(true);
    let mut copy: AdjListGraph<uint, uint> = AdjListGraph::new(true);

    g.add_node_with_prop(0, 1u);
    g.copy_node_to(&mut copy, 0);
    nodes.insert(0u, Some(1u));
    adj_list.insert(0, Vec::new());
    check(&g, &nodes, &edges, &adj_list);
    check(&copy, &nodes, &edges, &adj_list);

    g.add_edge_with_prop(0, 1, 2u);
    g.copy_edge_to(&mut copy, 0, 1);
    nodes.insert(1, None);
    edges.insert((0, 1), Some(2u));
    adj_list.insert(0, vec!(1));
    adj_list.insert(1, Vec::new());
    check(&g, &nodes, &edges, &adj_list);
    check(&copy, &nodes, &edges, &adj_list);

    g.add_node(2);
    g.copy_node_to(&mut copy, 2);
    nodes.insert(2, None);
    adj_list.insert(2, Vec::new());
    check(&g, &nodes, &edges, &adj_list);
    check(&copy, &nodes, &edges, &adj_list);

    g.add_edge(2, 3);
    g.copy_edge_to(&mut copy, 2, 3);
    nodes.insert(3, None);
    edges.insert((2, 3), None);
    adj_list.insert(2, vec!(3));
    adj_list.insert(3, Vec::new());
    check(&g, &nodes, &edges, &adj_list);
    check(&copy, &nodes, &edges, &adj_list);

    // Overwrite property
    copy.add_node_with_prop(0, 2);
    copy.copy_node_to(&mut g, 0);
    nodes.insert(0, Some(2));
    check(&g, &nodes, &edges, &adj_list);
    check(&copy, &nodes, &edges, &adj_list);

    copy.add_edge_with_prop(0, 1, 3u);
    copy.copy_edge_to(&mut g, 0, 1);
    edges.insert((0, 1), Some(3));
    adj_list.insert(0, vec!(1, 1));
    check(&g, &nodes, &edges, &adj_list);
    check(&copy, &nodes, &edges, &adj_list);
}

#[test]
fn undirected_graph_test() {
    let mut g: AdjListGraph<(), ()> = AdjListGraph::new(false);
    let mut nodes = HashMap::new();
    let mut edges = HashMap::new();
    let mut adj_list = HashMap::new();

    g.add_edge(0, 1);
    nodes.insert(0, None);
    nodes.insert(1, None);
    edges.insert((0, 1), None);
    adj_list.insert(0, vec!(1));
    adj_list.insert(1, vec!(0));
    check(&g, &nodes, &edges, &adj_list);
}
