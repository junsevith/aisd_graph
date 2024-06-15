use crate::graph::{GraphTrait, UndirectedGraph};
use crate::graph::algorithms::random_full;
use crate::graph::sep_set::SepSet;

#[test]
fn test_graph() {
    let mut graph = UndirectedGraph::new();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    graph.add_edge(0, 1, 10);
    graph.add_edge(1, 2, 20);
    graph.add_edge(2, 0, 30);
    assert_eq!(graph.get_edge(0, 1).unwrap().weight, &10);
    assert_eq!(graph.get_edge(1, 2).unwrap().weight, &20);
    assert_eq!(graph.get_edge(2, 0).unwrap().weight, &30);
    assert_eq!(graph.get_edge(1, 0), None);
    assert_eq!(graph.get_edge(2, 1), None);
    assert_eq!(graph.get_edge(0, 2), None);
}

#[test]
fn set_test() {
    let mut set = SepSet::empty(10);
    for i in 0..10 {
        set.make_set(i);
    }
    set.union(0, 1);
    set.union(2, 3);
    set.union(4, 5);
    set.union(6, 7);
    set.union(8, 9);
    set.union(0, 2);
    set.union(4, 6);
    set.union(8, 0);
    println!("{}", set.find(1));
    println!("{}", set.find(3));
    println!("{}", set.find(5));
    println!("{}", set.find(7));
    println!("{}", set.find(9));
    assert_eq!(set.find(1), set.find(3));
    assert_eq!(set.find(5), set.find(7));
    assert_eq!(set.find(9), set.find(1));
}

#[test]
fn test_full_graph() {
    let graph = random_full(50);
    let edges = graph.edges().collect::<Vec<_>>();
    println!("{:?}", edges);

}