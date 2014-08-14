#![feature(macro_rules)]

use std::collections::{Deque, HashMap, HashSet, RingBuf};
use std::io::File;

pub struct AdjListGraph<V, E> {
    adjList: HashMap<uint, HashMap<uint, E>>,
    nodes: HashMap<uint, V>,
}

impl<V, E> AdjListGraph<V, E> {
    pub fn new() -> AdjListGraph<V, E> {
        AdjListGraph { adjList: HashMap::new(), nodes: HashMap::new() }
    }

    pub fn add_vertex(&mut self, n: uint, v: V) {
        self.nodes.insert(n, v);
        self.adjList.insert(n, HashMap::new());
    }

    pub fn add_vertices(&mut self, vertices: Vec<(uint, V)>) {
        for (i, v) in vertices.move_iter() {
            self.add_vertex(i, v);
        }
    }

    pub fn add_edge(&mut self, from: uint, to: uint, e: E) {
        let adj = self.adjList.get_mut(&from);
        adj.insert(to, e);
    }

    pub fn add_edges(&mut self, edges: Vec<(uint, uint, E)>) {
        for (from, to, e) in edges.move_iter() {
            self.add_edge(from, to, e);
        }
    }

    pub fn output_graphviz(&self, filename: &str) {
        let path = Path::new(filename);
        let mut file = match File::create(&path) {
            Ok(f)  => f,
            Err(e) => fail!("Error opening file: {}", e)
        };
        file.write_str("digraph G {\n").ok();

        for (from, u) in self.nodes.iter() {
            for (to, _) in self.adjList[*from].iter() {
                file.write_str(
                    format!("\t{} -> {};\n", from, to).as_slice()).ok();
            }
        }

        file.write_str("}\n").ok();
    }

    // Algorithms

    pub fn dfs(&self, visit: |uint, Option<uint>|) {
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

    pub fn bfs(&self, visit: |uint, Option<uint>|) {
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

    pub fn prim() {

    }

    pub fn kruskal() {

    }

    pub fn djikstra() {

    }

    pub fn warshall() {

    }

    pub fn scc() {

    }

    pub fn ford_fulkerson() {

    }

    pub fn bellman_ford() {

    }
}
