use crate::graph::edge::Edge;


mod edge;
mod sep_set;
pub mod algorithms;

#[cfg(test)]
mod tests;

pub type UndirectedGraph<V, E> = Graph<V, E, false>;
pub struct Graph<V, E, const DIRECTED: bool = false> {
    pub vertices: Vec<V>,
    pub edges: Vec<Vec<Option<E>>>,
}

pub trait GraphTrait<V, E> {
    fn new() -> Self;
    fn new_with_vertices(vertices: Vec<V>) -> Self;
    fn add_vertex(&mut self, vertex: V);
    fn add_edge(&mut self, from: usize, to: usize, edge: E);
    fn get_edge(&self, from: usize, to: usize) -> Option<Edge<&E>>;
    fn edges_from<'a>(&'a self, index: usize) -> impl Iterator<Item=Edge<&'a E>>
    where
        E: 'a;
    fn edges<'a>(&'a self) -> impl Iterator<Item=Edge<&'a E>>
    where
        E: 'a;
    fn vertices(&self) -> &Vec<V>;
}


impl<V, E> GraphTrait<V, E> for Graph<V, E, false> {
    fn new() -> Self {
        Graph {
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn new_with_vertices(vertices: Vec<V>) -> Self {
        let edges = (0..vertices.len()).map(|_| (0..vertices.len()).map(|_| None).collect()).collect();
        Graph {
            vertices,
            edges,
        }
    }

    fn add_vertex(&mut self, vertex: V) {
        self.vertices.push(vertex);
        self.edges.iter_mut().for_each(|edges| edges.push(None));
        self.edges.push((0..self.vertices.len()).map(|_| None).collect());
    }


    fn add_edge(&mut self, from: usize, to: usize, edge: E) {
        if from < to {
            self.edges[from][to] = Some(edge);
        } else {
            self.edges[to][from] = Some(edge);
        }
    }

    fn get_edge(&self, from: usize, to: usize) -> Option<Edge<&E>> {
        if from < to {
            self.edges[from][to].as_ref().map(|edge| Edge::new(from, to, edge))
        } else {
            self.edges[to][from].as_ref().map(|edge| Edge::new(from, to, edge))
        }
    }

    fn edges_from<'a>(&'a self, index: usize) -> impl Iterator<Item=Edge<&'a E>>
    where
        E: 'a,
    {
        (0..self.vertices.len()).filter_map(
            move |i| {
                match self.get_edge(index, i) {
                    None => None,
                    Some(value) => Some(value)
                }
            }
        )
    }

    fn edges<'a>(&'a self) -> impl Iterator<Item=Edge<&'a E>>
    where
        E: 'a,
    {
        self.edges.iter().enumerate().flat_map(|(i, edges)| {
            edges.iter().enumerate().filter_map(move |(j, edge)| {
                match edge {
                    None => None,
                    Some(value) => Some(Edge::new(i, j, value))
                }
            })
        })
    }

    fn vertices(&self) -> &Vec<V> {
        self.vertices.as_ref()
    }
}

impl<V, E> GraphTrait<V, E> for Graph<V, E, true> {
    fn new() -> Self {
        Graph {
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn new_with_vertices(vertices: Vec<V>) -> Self {
        let edges = (0..vertices.len()).map(|_| (0..vertices.len()).map(|_| None).collect()).collect();
        Graph {
            vertices,
            edges,
        }
    }

    fn add_vertex(&mut self, vertex: V) {
        self.vertices.push(vertex);
        self.edges.iter_mut().for_each(|edges| edges.push(None));
        self.edges.push((0..self.vertices.len()).map(|_| None).collect());
    }

    fn add_edge(&mut self, from: usize, to: usize, edge: E) {
        self.edges[from][to] = Some(edge);
    }


    fn get_edge(&self, from: usize, to: usize) -> Option<Edge<&E>> {
        self.edges[from][to].as_ref().map(|edge| Edge::new(from, to, edge))
    }


    fn edges_from<'a>(&'a self, index: usize) -> impl Iterator<Item=Edge<&'a E>>
    where
        E: 'a,
    {
        (0..self.vertices.len()).filter_map(
            move |i| {
                match self.get_edge(index, i) {
                    None => None,
                    Some(value) => Some(value)
                }
            }
        )
    }


    fn edges<'a>(&'a self) -> impl Iterator<Item=Edge<&'a E>>
    where
        E: 'a,
    {
        self.edges.iter().enumerate().flat_map(|(i, edges)| {
            edges.iter().enumerate().filter_map(move |(j, edge)| {
                match edge {
                    None => None,
                    Some(value) => Some(Edge::new(i, j, value))
                }
            })
        })
    }

    fn vertices(&self) -> &Vec<V> {
        self.vertices.as_ref()
    }
}


