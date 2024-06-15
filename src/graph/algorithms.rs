use std::ops::Deref;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use crate::graph::Graph;
use crate::graph::sep_set::SepSet;

pub fn random_full(n: usize) -> Graph<usize, f64> {
    let vertices = (0..n).collect::<Vec<_>>();
    let mut graph = Graph::new_with_vertices(vertices);
    let range = Uniform::new(0., 1.).unwrap();
    let ref mut rng = rand::thread_rng();
    for i in 0..n {
        for j in i + 1..n {
            let weight = range.sample(rng);
            graph.add_edge(i, j, weight);
        }
    }
    graph
}

pub fn kruskal<V: Clone, E: Clone + PartialOrd>(graph: &Graph<V, E>) -> Graph<V, E> {
    let mut new_edges = Vec::with_capacity(graph.vertices.len() + 1);

    let mut set = SepSet::full(graph.vertices.len());

    let mut edges = graph.edges().collect::<Vec<_>>();
    edges.sort_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap());

    // println!("{:?}", edges);

    for (from, to, edge) in edges {
        let root1 = set.find(from);
        let root2 = set.find(to);
        if root1 != root2 {
            set.union(root1, root2);
            new_edges.push((from, to, edge));
        }
    }


    let mut new_graph = Graph::new_with_vertices(graph.vertices.clone());
    new_edges.iter().for_each(|(from, to, edge)| {
        new_graph.add_undirected_edge(*from, *to, edge.deref().clone());
    });

    new_graph
}