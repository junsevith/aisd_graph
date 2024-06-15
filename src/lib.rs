pub mod graph;
pub mod chart;
pub mod indicator;


#[cfg(test)]
mod tests {
    use crate::graph::algorithms::{broadcast_path, kruskal, prim, random_full};
    use crate::graph::GraphTrait;

    #[test]
    fn kruskal_test() {
        let graph = random_full(50);
        let new_graph = kruskal(&graph);
        let edges = new_graph.edges().collect::<Vec<_>>();
        edges.iter().for_each(|edge| {
            println!("{:?}", edge);
        });
    }

    #[test]
    fn prim_test() {
        let graph = random_full(50);
        let new_graph = prim(&graph);
        let edges = new_graph.edges().collect::<Vec<_>>();
        edges.iter().for_each(|edge| {
            println!("{:?}", edge);
        });
    }

    #[test]
    fn both_test() {
        let graph = random_full(50);
        let kruskal_graph = kruskal(&graph);
        let prim_graph = prim(&graph);
        let kruskal_edges = kruskal_graph.edges().collect::<Vec<_>>();
        let prim_edges = prim_graph.edges().collect::<Vec<_>>();
        println!("Kruskal:");
        kruskal_edges.iter().for_each(|edge| {
            println!("{:?}", edge);
        });
        println!("{}", kruskal_edges.iter().fold(0., |acc, edge| acc + edge.weight));

        println!("Prim:");
        prim_edges.iter().for_each(|edge| {
            println!("{:?}", edge);
        });
        println!("{}", prim_edges.iter().fold(0., |acc, edge| acc + edge.weight));
    }


    #[test]
    fn broadcast_test() {
        let graph = random_full(50);
        let new_graph = prim(&graph);
        let edges = new_graph.edges().collect::<Vec<_>>();
        edges.iter().for_each(|edge| {
            println!("{:?}", edge);
        });

        let (stat, broadcast) = broadcast_path(&new_graph, 0);
        println!("Stat: {:?}", stat);
        broadcast.iter().enumerate().for_each(|(i, edge)| {
            println!("{}: {:?}", i, edge);
        });
    }
}


