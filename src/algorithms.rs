//use graph::AdjListGraph;
use std::collections::{Deque, HashSet, PriorityQueue, RingBuf};
use graph::AdjListGraph;
use std::cmp::{Ord, Ordering};
use disjoint_set::DisjointSet;
use std::default::Default;

struct PQElt<E>(uint, Option<uint>, Option<E>);

impl<E: Ord> Ord for PQElt<E> {
    fn cmp(&self, other: &PQElt<E>) -> Ordering {
        let &PQElt(_, _, ref edge) = other;
        let &PQElt(_, _, ref self_edge) = self;
        let (e, self_e) = match (edge, self_edge) {
            (&Some(ref e), &Some(ref self_e)) => (e, self_e),
            _ => return Less // Return anything; Nones shouldn't be compared
        };
        // Reverse the Ordering, because we're using a max heap, not a min heap
        self_e.cmp(e).reverse()
    }
}

impl<E: Ord> PartialOrd for PQElt<E> {
    fn partial_cmp(&self, other: &PQElt<E>) -> Option<Ordering>{
        Some(self.cmp(other))
    }
}

impl<E: PartialEq> Eq for PQElt<E> {}

impl<E: PartialEq> PartialEq for PQElt<E> {
    fn eq(&self, other: &PQElt<E>) -> bool {
        let &PQElt(_, _, ref edge) = other;
        let &PQElt(_, _, ref self_edge) = self;
        let (e, self_e) = match (edge, self_edge) {
            (&Some(ref e), &Some(ref self_e)) => (e, self_e),
            _ => return false // Return anything; Nones shouldn't be compared
        };
        self_e.eq(e)
    }
}

pub trait Weight {
    fn weight(&self) -> int;
}

pub fn dfs<V: Clone + Default,
           E: Clone + Default + Ord>(g: &AdjListGraph<V, E>,
                                  visit: |uint, Option<uint>|) {
    let mut visited: HashSet<uint> = HashSet::new();
    visited.insert(0);
    dfs_helper(g, 0, None, &mut visited, visit);

    fn dfs_helper<V: Clone + Default,
    E: Clone + Default + Ord>(g: &AdjListGraph<V, E>,
                    cur: uint,
                    parent: Option<uint>,
                    visited: &mut HashSet<uint>,
                    visit: |uint, Option<uint>|) {
        visit(cur, parent);
        visited.insert(cur);
        for to in g.adj_iter(cur) {
            if !visited.contains(to) {
                dfs_helper(g, *to, Some(cur), visited,
                           |to, parent| visit(to, parent));
            }
        }
    }
}

pub fn bfs<V: Clone + Default,
           E: Clone + Default + Ord>(g: &AdjListGraph<V, E>,
                                  visit: |uint, Option<uint>|) {
    let mut visited: HashSet<uint> = HashSet::new();
    let mut queue: RingBuf<(uint, Option<uint>)> = RingBuf::new();
    visited.insert(0);
    queue.push((0, None));

    while !queue.is_empty() {
        let (u, parent) = queue.pop_front().unwrap();
        visit(u, parent);
        for v in g.adj_iter(u) {
            if !visited.contains(v) {
                visited.insert(*v);
                queue.push((*v, Some(u)));
            }
        }
    }
}

pub fn prim<V: Clone + Default,
            E: Clone + Default + Ord + Weight>(g: &AdjListGraph<V, E>)
                                     -> AdjListGraph<V, E> {
    let mut mst = AdjListGraph::new();
    let mut pq: PriorityQueue<PQElt<E>> =
        PriorityQueue::new();
    let mut visited: HashSet<uint> = HashSet::new();

    pq.push(PQElt(0, None, None));
    mst.add_vertex_with_prop(0, g.node_prop(0));

    while mst.size() != g.size() {
        // Pick edge with minimal weight and add to graph
        let PQElt(u, parent, min_edge) = pq.pop().unwrap();

        match (parent, min_edge) {
            (None, None) => (),
            (Some(parent), Some(min_edge)) => {
                mst.add_vertex_with_prop(u, g.node_prop(u));
                mst.add_edge_with_prop(parent, u, min_edge);
            },
            (_, _) => fail!("Error")
        }

        // Push all adjacent edges on to priority queue
        visited.insert(u);
        for v in g.adj_iter(u) {
            if !visited.contains(v) {
                pq.push(PQElt(*v, Some(u), Some(g.edge_prop(u, *v))));
            }
        }
    }

    mst
}

pub fn kruskal<V: Clone + Default,
               E: Clone + Default + Ord + Weight>(g: &AdjListGraph<V, E>)
                                                  -> AdjListGraph<V, E>{
    let mut ds = DisjointSet::new();
    for v in g.nodes_iter() {
        ds.add_set(*v);
    }

    let mut edge_weights = Vec::new();
    for &(u, v) in g.edges_iter() {
        edge_weights.push((u, v, g.edge_prop(u, v)))
    }
    edge_weights.sort();

    let mut mst: AdjListGraph<V, E> = AdjListGraph::new();
    while !edge_weights.is_empty() {
        let (u, v, weight) = edge_weights.pop().unwrap();
        ds.find(&u);
        if ds.find(&u) != ds.find(&v) {
            mst.add_edge_with_prop(u, v, weight);
            ds.union(&u, &v);
        }
    }

    mst
}

pub fn djikstra() {

}

pub fn warshall() {

}

pub fn connected_components<V: Clone + Default,
                            E: Clone + Default + Ord>(g: &AdjListGraph<V, E>) {
    let mut ds = DisjointSet::new();
    for v in g.nodes_iter() {
        ds.add_set(v.clone());
    }

    for u in g.nodes_iter() {
        for v in g.adj_iter(*u) {
            if ds.find(u) != ds.find(v) {
                ds.union(u, v);
            }
        }
    }

    println!("{}", ds);
}

pub fn ford_fulkerson() {

}

pub fn bellman_ford() {

}
