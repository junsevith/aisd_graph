use std::ops::Deref;

use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::{Rng, thread_rng};

use crate::graph::edge::Edge;
use crate::graph::{Graph, GraphTrait};
use crate::graph::sep_set::SepSet;

pub fn random_full(n: usize) -> Graph<(), f64> {
    let vertices = (0..n).map(|_| ()).collect::<Vec<_>>();
    let mut graph = Graph::<(), f64, false>::new_with_vertices(vertices);
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

pub fn kruskal<G, V: Clone, E: Clone + PartialOrd>(graph: &G) -> G
where
    G: GraphTrait<V, E>,
{
    let mut new_edges = Vec::with_capacity(graph.vertices().len() + 1);

    let mut set = SepSet::full(graph.vertices().len());

    let mut edges = graph.edges().collect::<Vec<_>>();
    edges.sort_unstable();


    for Edge { from, to, weight } in edges {
        let root1 = set.find(from);
        let root2 = set.find(to);
        if root1 != root2 {
            set.union(root1, root2);
            new_edges.push((from, to, weight));
        }
    }


    let mut new_graph = G::new_with_vertices(graph.vertices().clone());
    new_edges.iter().for_each(|(from, to, edge)| {
        new_graph.add_edge(*from, *to, edge.deref().clone());
    });

    new_graph
}

pub fn prim<G, V: Clone, E: Clone + PartialOrd>(graph: &G) -> G
where
    G: GraphTrait<V, E>,
{
    let vertices = graph.vertices().len();

    let mut new_graph = G::new_with_vertices(graph.vertices().clone());
    let mut new_edges = Vec::with_capacity(vertices + 1);

    let mut current = 0;
    let mut done = 1;
    let mut visited = vec![false; vertices];
    visited[current] = true;
    let mut distances = (0..vertices).map(|_| None).collect::<Vec<_>>();
    let mut min = 0;

    while done < vertices {
        for i in 0..vertices {
            if !visited[i] {
                //checking for new edge thet might be shorter
                if let Some(edge) = graph.get_edge(current, i) {
                    if let Some(distance) = &distances[i] {
                        if &edge < distance {
                            distances[i] = Some(edge);
                        }
                    } else {
                        distances[i] = Some(edge);
                    }
                }

                //updating min if possible
                match (&distances[i], &distances[min]) {
                    (Some(x), Some(y)) => {
                        if x < y {
                            min = i;
                        }
                    }
                    (Some(_), None) => {
                        min = i;
                    }
                    _ => {}
                }
            }
        }

        //adding min edge to MST
        let new_edge = distances[min].take().unwrap();
        current = new_edge.to;
        new_edges.push(new_edge);
        visited[current] = true;
        done += 1;
    }

    //adding edges to new graph
    new_edges.iter().for_each(|edge| {
        new_graph.add_edge(edge.from, edge.to, edge.weight.clone());
    });

    new_graph
}

pub fn broadcast_path<G, V: Clone, E: Clone + PartialOrd>(graph: &G, start: usize) -> (usize, Vec<Vec<(usize, usize)>>)
where
    G: GraphTrait<V, E>,
{
    let ver_count = graph.vertices().len();
    let mut visited = vec![false; ver_count];
    let mut order = vec![Vec::new(); ver_count];
    let stats = broadcast_helper(graph, start, &mut visited, &mut order);
    // println!("Full: {}", stats);
    (stats, order)
}

fn broadcast_helper<G, V: Clone, E: Clone + PartialOrd>(
    graph: &G,
    start: usize,
    visited: &mut Vec<bool>,
    order: &mut Vec<Vec<(usize, usize)>>,
)
    -> usize
where
    G: GraphTrait<V, E>,
{
    visited[start] = true;
    let mut children = Vec::new();
    for edge in graph.edges_from(start) {
        if !visited[edge.to] {
            let size = broadcast_helper(graph, edge.to, visited, order);
            children.push((edge.to, size));
        }
    }
    children.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    // let size = children.first().map(|x| x.1).unwrap_or(0) + 1; //subtree height
    // let size = children.iter().fold(1, |acc, x| acc + x.1); //subtree size
    let size = children.iter().enumerate().map(|(i, x)| x.1 + i).max().unwrap_or(0) + 1;

    order[start] = children;

    size
}







