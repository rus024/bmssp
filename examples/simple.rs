use bmssp::{Edge, ShortestPath};

fn main() {
    let mut graph = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    graph[0].push(Edge::new(1, 1.0));
    graph[0].push(Edge::new(2, 4.0));
    graph[1].push(Edge::new(2, 2.0));
    graph[1].push(Edge::new(3, 5.0));
    graph[2].push(Edge::new(3, 1.0));

    let mut sp = ShortestPath::new(graph);
    let distances = sp.get(0);
    println!("Shortest distances from node 0: {:?}", distances);
}
