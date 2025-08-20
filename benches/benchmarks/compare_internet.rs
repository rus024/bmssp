use bmssp::{ShortestPath, Edge};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Load SNAP-style graphs (edge list format: "src dst")
fn load_snap(path: &str) -> (Vec<Vec<Edge>>, Vec<Vec<(usize, f64)>>) {
    let file = File::open(path).expect("File not found");
    let reader = BufReader::new(file);

    // Find max node id
    let mut max_node = 0;
    let mut edges = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            continue;
        }
        let u: usize = parts[0].parse().unwrap();
        let v: usize = parts[1].parse().unwrap();
        max_node = max_node.max(u.max(v));
        edges.push((u, v));
    }

    let n = max_node + 1;
    let mut bm_graph = vec![Vec::new(); n];
    let mut dj_graph = vec![Vec::new(); n];

    for (u, v) in edges {
        bm_graph[u].push(Edge::new(v, 1.0));
        dj_graph[u].push((v, 1.0));
    }

    (bm_graph, dj_graph)
}

fn compare_internet(c: &mut Criterion) {
    let mut group = c.benchmark_group("BMSSP_vs_Dijkstra_Internet");

    // AS-733
    let (bm_graph, dj_graph) = load_snap("data/as-733.txt");
    group.bench_function(BenchmarkId::new("BMSSP", "AS-733"), |b| {
        b.iter(|| {
            let mut sp = ShortestPath::new(bm_graph.clone());
            black_box(sp.get(0usize))
        });
    });
    group.bench_function(BenchmarkId::new("Dijkstra", "AS-733"), |b| {
        b.iter(|| black_box(dijkstra(&dj_graph, 0usize)));
    });

    // AS-Skitter
    let (bm_graph, dj_graph) = load_snap("data/as-skitter.txt");
    group.bench_function(BenchmarkId::new("BMSSP", "AS-Skitter"), |b| {
        b.iter(|| {
            let mut sp = ShortestPath::new(bm_graph.clone());
            black_box(sp.get(0usize))
        });
    });
    group.bench_function(BenchmarkId::new("Dijkstra", "AS-Skitter"), |b| {
        b.iter(|| black_box(dijkstra(&dj_graph, 0usize)));
    });

    group.finish();
}

criterion_group!(benches, compare_internet);
criterion_main!(benches);
