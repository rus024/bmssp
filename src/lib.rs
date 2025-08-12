mod heaps;
mod models;
mod shortest_path;

pub use models::{Edge, Graph};
pub use shortest_path::ShortestPath;

#[cfg(test)]
mod tests {
    use super::*;

    // AOJ GRL_1_A (https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_A)
    #[test]
    fn test_bmssp_sample_1() {
        // Graph: 0->1(1), 0->2(4), 1->2(2), 2->3(1), 1->3(5)
        let mut graph = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        graph[0].push(Edge::new(1, 1.0));
        graph[0].push(Edge::new(2, 4.0));
        graph[1].push(Edge::new(2, 2.0));
        graph[1].push(Edge::new(3, 5.0));
        graph[2].push(Edge::new(3, 1.0));

        let mut sp = ShortestPath::new(graph);
        let distances = sp.get(0);

        assert_eq!(distances[0], 0.0);
        assert_eq!(distances[1], 1.0);
        assert_eq!(distances[2], 3.0);
        assert_eq!(distances[3], 4.0);
    }

    #[test]
    fn test_bmssp_sample_2() {
        // Graph from sample 2 with source vertex 1
        let mut graph = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        graph[0].push(Edge::new(1, 1.0));
        graph[0].push(Edge::new(2, 4.0));
        graph[2].push(Edge::new(0, 1.0));
        graph[1].push(Edge::new(2, 2.0));
        graph[3].push(Edge::new(1, 1.0));
        graph[3].push(Edge::new(2, 5.0));

        let mut sp = ShortestPath::new(graph);
        let distances = sp.get(1);

        assert_eq!(distances[0], 3.0);
        assert_eq!(distances[1], 0.0);
        assert_eq!(distances[2], 2.0);
        assert_eq!(distances[3], f32::INFINITY);
    }
}
