#![feature(macro_rules)]

use std::collections::{Deque, HashMap, HashSet, RingBuf};
use std::io::File;

macro_rules! edges (
    ($($u:expr => $v:expr),+) => ({
        let mut edges: Vec<(uint, uint, Edge)> = Vec::new();
        $(
            edges.push(($u, $v, Edge));
        )+
        edges
    });
    ($($u:expr => $v:expr),+,) => (edges!($($u => $v),+))
)

fn main() {
    let mut g: AdjListGraph<Node, Edge> = AdjListGraph::new();
    g.add_vertices(&[0, 1, 2, 3, 4], Node);
    let e = edges!(0 => 3, 3 => 2, 3 => 1, 1 => 4, 4 => 2);
    g.add_edges(e);
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
    println!("dfs");
    for (i, d) in dist.iter() {
        println!("(i, d): {}", (i, d));
    }

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
    println!("bfs");
    for (i, d) in dist.iter() {
        println!("(i, d): {}", (i, d));
    }

    g.output_graphviz("graph.dot");
}

#[deriving(Clone)]
struct Node;

// impl Show for Node {
//     fn fmt(&self, f: &mut Formatter) -> Result {
//         write!(f, "{}", )
//     }
// }

#[deriving(Clone)]
struct Edge;

// impl Edge {
//     fn new(u: uint, v: uint) -> Edge {
//         Edge { u: Node { label: u }, v: Node { label: v } }
//     }
// }

struct AdjListGraph<V, E> {
    adjList: HashMap<uint, HashMap<uint, (V, E)>>,
    nodes: HashMap<uint, V>,
}

impl<V: Clone, E> AdjListGraph<V, E> {
    fn new() -> AdjListGraph<V, E> {
        AdjListGraph { adjList: HashMap::new(), nodes: HashMap::new() }
    }

    fn add_vertex(&mut self, n: uint, v: &V) {
        self.nodes.insert(n, v.clone());
        self.adjList.insert(n, HashMap::new());
    }

    fn add_vertices(&mut self, vertices: &[uint], v: V) {
        for i in vertices.iter() {
            self.add_vertex(*i, &v);
        }
    }

    fn add_edge(&mut self, from: uint, to: uint, e: E) {
        let adj = self.adjList.get_mut(&from);
        adj.insert(to, (self.nodes[to].clone(), e));
    }

    fn add_edges(&mut self, edges: Vec<(uint, uint, E)>) {
        for (from, to, e) in edges.move_iter() {
            self.add_edge(from, to, e);
        }
    }

    fn output_graphviz(&self, filename: &str) {
        let path = Path::new(filename);
        let mut file = match File::create(&path) {
            Ok(f)  => f,
            Err(e) => fail!("Error opening file: {}", e)
        };
        file.write_str("digraph G {\n").ok();

        for (from, u) in self.nodes.iter() {
            for (to, &(ref v, _)) in self.adjList[*from].iter() {
                file.write_str(
                    format!("\t{} -> {};\n", from, to).as_slice()).ok();
            }
        }

        file.write_str("}\n").ok();
    }

    fn dfs(&self, visit: |uint, Option<uint>|) {
        let mut visited: HashSet<uint> = HashSet::new();
        visited.insert(0);
        dfs_helper(self, 0, None, &mut visited, visit);

        fn dfs_helper<V, E>(g: &AdjListGraph<V, E>, cur: uint,
                            parent: Option<uint>, visited: &mut HashSet<uint>,
                            visit: |uint, Option<uint>|) {
            visit(cur, parent);
            visited.insert(cur);
            for (to, _) in g.adjList[cur].iter() {
                if !visited.contains(to) {
                    dfs_helper(g, *to, Some(cur), visited,
                               |to, parent| visit(to, parent));
                }
            }
        }
    }

    fn bfs(&self, visit: |uint, Option<uint>|) {
        let mut visited: HashSet<uint> = HashSet::new();
        let mut queue: RingBuf<(uint, Option<uint>)> = RingBuf::new();
        visited.insert(0);
        queue.push((0, None));

        while !queue.is_empty() {
            let (u, parent) = queue.pop_front().unwrap();
            visit(u, parent);
            for (v, _) in self.adjList[u].iter() {
                if !visited.contains(v) {
                    visited.insert(*v);
                    queue.push((*v, Some(u)));
                }
            }
        }
    }
}

// trait Graph {
//     //fn nodes(&'a self) -> &'a [int];
// }

// struct AdjMatrixGraph {
//     adjMatrix: Vec<Vec<int>>
// }

// impl AdjMatrixGraph {
//     fn new() -> AdjMatrixGraph {
//         let adjMatrix: Vec<Vec<int>> = vec!();
//         AdjMatrixGraph { adjMatrix: adjMatrix }
//     }
// }

// impl Graph for AdjListGraph {
//     //fn nodes(&'a self) -> &'a [int] { self.nodes.as_slice() }
// }
