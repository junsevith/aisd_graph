use crate::graph::edge::Edge;

pub struct Graph<V, E> {
    pub vertices: Vec<V>,
    pub edges: Vec<Vec<Option<E>>>,
}

impl<V, E> Graph<V, E> {
    pub fn new() -> Self {
        Graph {
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn new_with_vertices(vertices: Vec<V>) -> Self {
        let edges = (0..vertices.len()).map(|_| (0..vertices.len()).map(|_| None).collect()).collect();
        Graph {
            vertices,
            edges,
        }
    }

    pub fn add_vertex(&mut self, vertex: V) {
        self.vertices.push(vertex);
        self.edges.iter_mut().for_each(|edges| edges.push(None));
        self.edges.push((0..self.vertices.len()).map(|_| None).collect());
    }

    fn add_edge_directed(&mut self, from: usize, to: usize, edge: E) {
        self.edges[from][to] = Some(edge);
    }

    pub fn add_edge(&mut self, from: usize, to: usize, edge: E) {
        if from < to {
            self.edges[from][to] = Some(edge);
        } else {
            self.edges[to][from] = Some(edge);
        }
    }

    fn get_edge_directed(&self, from: usize, to: usize) -> Option<Edge<&E>> {
        self.edges[from][to].as_ref().map(|edge| Edge::new(from, to, edge))
    }

    pub fn get_edge(&self, from: usize, to: usize) -> Option<Edge<&E>> {
        if from < to {
            self.edges[from][to].as_ref().map(|edge| Edge::new(from, to, edge))
        } else {
            self.edges[to][from].as_ref().map(|edge| Edge::new(from, to, edge))
        }
    }

    fn edges_from_directed(&self, index: usize) -> impl Iterator<Item=Edge<&E>> {
        self.edges[index].iter().enumerate().filter_map(move |(i, edge)| {
            match edge {
                None => None,
                Some(value) => Some(Edge::new(index, i, value))
            }
        })
    }

    pub fn edges_from(&self, index: usize) -> impl Iterator<Item=Edge<&E>> {
        (0..self.vertices.len()).filter_map(
            move |i| {
                match self.get_edge(index, i) {
                    None => None,
                    Some(value) => Some(value)
                }
            }
        )
    }

    pub fn edges(&self) -> impl Iterator<Item=Edge<&E>> {
        self.edges.iter().enumerate().flat_map(|(i, edges)| {
            edges.iter().enumerate().filter_map(move |(j, edge)| {
                match edge {
                    None => None,
                    Some(value) => Some(Edge::new(i, j, value))
                }
            })
        })
    }
}



