use bmssp::{Edge, Graph};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn generate_random_graph(vertices: usize, edges: usize, max_weight: f32, seed: u64) -> Graph {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut graph = vec![Vec::new(); vertices];

    for _ in 0..edges {
        let from = rng.random_range(0..vertices);
        let to = rng.random_range(0..vertices);
        let weight = rng.random::<f32>() * max_weight;

        // Avoid self-loops
        if from != to {
            graph[from].push(Edge::new(to, weight));
        }
    }

    graph.into()
}

pub fn generate_connected_graph(
    vertices: usize,
    edges: usize,
    max_weight: f32,
    seed: u64,
) -> Graph {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut graph = vec![Vec::new(); vertices];

    // First, create a spanning tree to ensure connectivity
    for i in 1..vertices {
        let parent = rng.random_range(0..i);
        let weight = rng.random::<f32>() * max_weight;
        graph[parent].push(Edge::new(i, weight));
        // Add reverse edge for undirected graph
        graph[i].push(Edge::new(parent, weight));
    }

    // Add remaining random edges
    let remaining_edges = edges.saturating_sub(vertices - 1);
    for _ in 0..remaining_edges {
        let from = rng.random_range(0..vertices);
        let to = rng.random_range(0..vertices);
        let weight = rng.random::<f32>() * max_weight;

        if from != to {
            graph[from].push(Edge::new(to, weight));
        }
    }

    graph.into()
}

pub fn generate_sparse_graph(vertices: usize, density: f32, max_weight: f32, seed: u64) -> Graph {
    let max_edges = vertices * (vertices - 1);
    let edges = ((max_edges as f32) * density) as usize;
    generate_random_graph(vertices, edges, max_weight, seed)
}

pub fn generate_dense_graph(vertices: usize, max_weight: f32, seed: u64) -> Graph {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut graph = vec![Vec::new(); vertices];

    // Create edges between all pairs of vertices
    for from in 0..vertices {
        for to in 0..vertices {
            if from != to {
                let weight = rng.random::<f32>() * max_weight;
                graph[from].push(Edge::new(to, weight));
            }
        }
    }

    graph.into()
}
