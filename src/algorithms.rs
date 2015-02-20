//use graph::AdjListGraph;
use std::collections::{HashSet, BinaryHeap, VecDeque};
use graph::AdjListGraph;
use std::cmp::{Ord, Ordering};
use disjoint_set::DisjointSet;

struct PQElt<E>(usize, Option<usize>, Option<Option<E>>);

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
    fn weight(&self) -> isize;
    fn set_weight(&mut self, isize);
}

pub trait DFSVisitor {
    #[allow(unused_variables)]
    fn visit(&mut self, node: usize, parent: Option<usize>) {}
}

pub fn dfs<T: DFSVisitor, V: Clone,
           E: Clone + Ord>(g: &AdjListGraph<V, E>,
                           visitor: &mut T) {
    match g.nodes_iter().nth(1) {
        Some(source) => dfs_from(g, visitor, *source),
        None         => ()
    }
}

pub fn dfs_from<T: DFSVisitor, V: Clone,
                E: Clone + Ord>(g: &AdjListGraph<V, E>,
                                visitor: &mut T,
                                source: usize) {
    let mut visited: HashSet<usize> = HashSet::new();

    visited.insert(source);
    dfs_helper(g, source, None, &mut visited, visitor);

    fn dfs_helper<T: DFSVisitor, V: Clone,
                  E: Clone + Ord>(g: &AdjListGraph<V, E>,
                                  cur: usize,
                                  parent: Option<usize>,
                                  visited: &mut HashSet<usize>,
                                  visitor: &mut T) {
        visitor.visit(cur, parent);
        visited.insert(cur);
        for to in g.adj_iter(cur) {
            if !visited.contains(to) {
                dfs_helper(g, *to, Some(cur), visited, visitor);
            }
        }
    }
}

pub fn bfs<V: Clone,
           E: Clone + Ord,
           F: FnMut(usize, Option<usize>)>(g: &AdjListGraph<V, E>,
                                           mut visit: F,
                                           source: usize) {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut queue: VecDeque<(usize, Option<usize>)> = VecDeque::new();
    visited.insert(source);
    queue.push_back((source, None));

    while !queue.is_empty() {
        let (u, parent) = queue.pop_front().unwrap();
        visit(u, parent);
        for v in g.adj_iter(u) {
            if !visited.contains(v) {
                visited.insert(*v);
                queue.push_back((*v, Some(u)));
            }
        }
    }
}

pub fn prim<V: Clone,
            E: Clone + Ord + Weight>(g: &AdjListGraph<V, E>)
                                     -> Result<AdjListGraph<V, E>, &'static str> {
    if g.is_directed() {
        return Err("Prim's algorithm only works with an undirected graph");
    }

    let mut mst = AdjListGraph::new(false);
    let mut pq: BinaryHeap<PQElt<E>> =
        BinaryHeap::new();
    let mut visited: HashSet<usize> = HashSet::new();

    // TODO: Should the user be allowed to choose the source node
    let source = match g.nodes_iter().nth(1) {
        Some(source) => *source,
        None         => return Ok(mst)
    };

    pq.push(PQElt(source, None, None));

    while mst.size() != g.size() {
        if pq.is_empty() {
            return Err("Graph is not connected, no MST found");
        }

        // Pick edge with minimal weight and add to graph
        let PQElt(u, parent, min_edge) = pq.pop().unwrap();
        if mst.contains_node(u) {
            continue;
        }

        match (parent, min_edge) {
            (None, None) => g.copy_node_to(&mut mst, source),
            (Some(parent), Some(_)) => {
                g.copy_edge_to(&mut mst, parent, u);
            },
            (_, _) => panic!("Error")
        }

        // Push all adjacent edges on to priority queue
        visited.insert(u);
        for v in g.adj_iter(u) {
            if !visited.contains(v) {
                pq.push(PQElt(*v, Some(u), Some(g.edge_prop(u, *v))));
            }
        }
    }

    Ok(mst)
}

pub fn kruskal<V: Clone,
               E: Clone + Ord + Weight>(g: &AdjListGraph<V, E>)
                                        -> Result<AdjListGraph<V, E>, &'static str> {
    if g.is_directed() {
        return Err("Kruskal's algorithm only works with an undirected graph");
    }

    let mut ds = DisjointSet::new();
    for v in g.nodes_iter() {
        ds.add_set(*v);
    }

    let mut edge_weights = Vec::new();
    for &(u, v) in g.edges_iter() {
        edge_weights.push((u, v, g.edge_prop(u, v)))
    }
    edge_weights.sort_by(|&(_, _, ref prop1), &(_, _, ref prop2)|
                         prop2.cmp(prop1));

    let mut mst: AdjListGraph<V, E> = AdjListGraph::new(false);
    while !edge_weights.is_empty() {
        let (u, v, _) = edge_weights.pop().unwrap();
        ds.find(&u);
        if ds.find(&u) != ds.find(&v) {
            g.copy_edge_to(&mut mst, u, v);
            ds.union(&u, &v);
        }
    }

    assert!(mst.num_edges() < mst.size());
    if mst.num_edges() != mst.size() - 1 {
        return Err("Graph is not connected, no MST found");
    }

    Ok(mst)
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

    println!("{:?}", ds);
}

pub fn ford_fulkerson() {

}

pub fn bellman_ford() {

}

#[cfg(test)]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Edge {
    weight: isize
}

#[cfg(test)]
impl Edge {
    fn new(weight: isize) -> Edge {
        Edge { weight: weight }
    }
}

#[cfg(test)]
impl Weight for Edge {
    fn weight(&self) -> isize {
        self.weight
    }

    fn set_weight(&mut self, weight: isize) {
        self.weight = weight;
    }
}

#[test]
fn mst_test() {
    let mut g: AdjListGraph<(), Edge> = AdjListGraph::new(false);
    g.add_edge_with_prop(0, 1, Edge::new(4));
    g.add_edge_with_prop(1, 2, Edge::new(8));
    g.add_edge_with_prop(2, 3, Edge::new(7));
    g.add_edge_with_prop(2, 5, Edge::new(5));
    g.add_edge_with_prop(3, 4, Edge::new(9));
    g.add_edge_with_prop(3, 5, Edge::new(20));
    g.add_edge_with_prop(4, 5, Edge::new(19));
    g.add_edge_with_prop(5, 6, Edge::new(3));
    g.add_edge_with_prop(6, 7, Edge::new(1));
    g.add_edge_with_prop(7, 8, Edge::new(18));
    g.add_edge_with_prop(6, 8, Edge::new(17));
    g.add_edge_with_prop(8, 2, Edge::new(2));
    g.add_edge_with_prop(7, 0, Edge::new(16));
    g.add_edge_with_prop(1, 7, Edge::new(11));

    let mut mst: AdjListGraph<(), Edge> = AdjListGraph::new(false);
    mst.add_edge_with_prop(0, 1, Edge::new(4));
    mst.add_edge_with_prop(2, 3, Edge::new(7));
    mst.add_edge_with_prop(2, 5, Edge::new(5));
    mst.add_edge_with_prop(3, 4, Edge::new(9));
    mst.add_edge_with_prop(5, 6, Edge::new(3));
    mst.add_edge_with_prop(6, 7, Edge::new(1));
    mst.add_edge_with_prop(8, 2, Edge::new(2));
    mst.add_edge_with_prop(1, 2, Edge::new(8));

    let pmst = prim(&g).unwrap();
    let kmst = kruskal(&g).unwrap();
    assert_eq!(mst, pmst);
    assert_eq!(mst, kmst);
}

#[test]
fn mst_error_test() {
    let directed_graph: AdjListGraph<(), Edge> = AdjListGraph::new(true);
    prim(&directed_graph).err();
    kruskal(&directed_graph).err();

    let mut disconnected_graph = AdjListGraph::new(false);
    disconnected_graph.add_edge_with_prop(0, 1, Edge::new(1));
    disconnected_graph.add_edge_with_prop(0, 2, Edge::new(2));
    disconnected_graph.add_edge_with_prop(1, 2, Edge::new(3));
    disconnected_graph.add_node_with_prop(3, ());
    prim(&disconnected_graph).err();
    kruskal(&disconnected_graph).err();
}
