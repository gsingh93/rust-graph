//use graph::AdjListGraph;
use std::collections::{Deque, HashSet, PriorityQueue, RingBuf};
use graph::AdjListGraph;
use std::cmp::{Ord, Ordering};
use disjoint_set::DisjointSet;

struct PQElt<E>(uint, Option<uint>, Option<Option<E>>);

impl<E: Ord> Ord for PQElt<E> {
    fn cmp(&self, other: &PQElt<E>) -> Ordering {
        let &PQElt(_, _, ref edge) = other;
        let &PQElt(_, _, ref self_edge) = self;

        // Reverse the Ordering, because we're using a max heap, not a min heap
        self_edge.cmp(edge).reverse()
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
        self_edge.eq(edge)
    }
}

pub trait Weight {
    fn weight(&self) -> int;
    fn set_weight(&mut self, int);
}

pub fn dfs<V: Clone,
           E: Clone + Ord>(g: &AdjListGraph<V, E>,
                                     visit: |uint, Option<uint>|) {
    match g.nodes_iter().nth(1) {
        Some(source) => dfs_from(g, visit, *source),
        None         => ()
    }
}

pub fn dfs_from<V: Clone,
                E: Clone + Ord>(g: &AdjListGraph<V, E>,
                                          visit: |uint, Option<uint>|,
                                          source: uint) {
    let mut visited: HashSet<uint> = HashSet::new();

    visited.insert(source);
    dfs_helper(g, source, None, &mut visited, visit);

    fn dfs_helper<V: Clone,
    E: Clone + Ord>(g: &AdjListGraph<V, E>,
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

pub fn bfs<V: Clone,
           E: Clone + Ord>(g: &AdjListGraph<V, E>,
                                     visit: |uint, Option<uint>|,
                                     source: uint) {
    let mut visited: HashSet<uint> = HashSet::new();
    let mut queue: RingBuf<(uint, Option<uint>)> = RingBuf::new();
    visited.insert(source);
    queue.push((source, None));

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

pub fn prim<V: Clone,
            E: Clone + Ord + Weight>(g: &AdjListGraph<V, E>)
                                               -> AdjListGraph<V, E> {
    let mut mst = AdjListGraph::new();
    let mut pq: PriorityQueue<PQElt<E>> =
        PriorityQueue::new();
    let mut visited: HashSet<uint> = HashSet::new();

    // TODO: Should the user be allowed to choose the source node
    let source = match g.nodes_iter().nth(1) {
        Some(source) => *source,
        None         => return mst
    };

    pq.push(PQElt(source, None, None));
    g.copy_node_to(&mut mst, source);

    while mst.size() != g.size() {
        // Pick edge with minimal weight and add to graph
        let PQElt(u, parent, min_edge) = pq.pop().unwrap();

        match (parent, min_edge) {
            (None, None) => (),
            (Some(parent), Some(_)) => {
                g.copy_edge_to(&mut mst, parent, u);
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

pub fn kruskal<V: Clone,
               E: Clone + Ord + Weight>(g: &AdjListGraph<V, E>)
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
        let (u, v, _) = edge_weights.pop().unwrap();
        ds.find(&u);
        if ds.find(&u) != ds.find(&v) {
            g.copy_edge_to(&mut mst, u, v);
            ds.union(&u, &v);
        }
    }

    mst
}

pub fn djikstra() {

}

pub fn warshall() {

}

pub fn connected_components<V: Clone,
                            E: Clone + Ord>(g: &AdjListGraph<V, E>) {
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
