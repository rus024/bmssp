use bmssp::{ShortestPath, Graph, Edge};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;

type DjGraph = Vec<Vec<(usize, f64)>>;

#[derive(Copy, Clone, PartialEq)]
struct State { cost: f64, node: usize }
impl Eq for State {}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> { Some(self.cmp(o)) }
}

fn dijkstra(adj: &DjGraph, s: usize) -> Vec<f64> {
    let n = adj.len();
    let mut dist = vec![f64::INFINITY; n];
    dist[s] = 0.0;
    let mut pq = BinaryHeap::new();
    pq.push(State { cost: 0.0, node: s });

    while let Some(State { cost, node }) = pq.pop() {
        if cost > dist[node] { continue; }
        for &(v, w) in &adj[node] {
            let nd = cost + w;
            if nd < dist[v] {
                dist[v] = nd;
                pq.push(State { cost: nd, node: v });
            }
        }
    }
    dist
}

/// Generate random graphs for quick benchmarks
fn gen_graph(n: usize, m: usize, seed: u64) -> (Graph, DjGraph) {
    let mut rng = StdRng::seed_from_u64(seed);

    let mut adj_edges: Vec<Vec<Edge>> = vec![Vec::new(); n];
    let mut dj: DjGraph = vec![Vec::new(); n];

    for _ in 0..m {
        let u = rng.random_range(0..n);
        let mut v = rng.random_range(0..n);
        if v == u { v = (v + 1) % n; }

        let w_f32: f32 = rng.random_range(1.0f32..10.0f32);

        adj_edges[u].push(Edge::new(v, w_f32));
        dj[u].push((v, w_f32 as f64));
    }

    let bm: Graph = adj_edges.into();
    (bm, dj)
}

/// Load real road network dataset from SNAP
fn load_roadnet(path: &str) -> (Graph, DjGraph) {
    let file = File::open(path).expect("roadNet-PA.txt not found");
    let reader = BufReader::new(file);

    let mut max_node = 0usize;
    let mut edges: Vec<(usize, usize)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with('#') { continue; } // skip comments
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 { continue; }
        let u: usize = parts[0].parse().unwrap();
        let v: usize = parts[1].parse().unwrap();
        max_node = max_node.max(u).max(v);
        edges.push((u, v));
    }

    let n = max_node + 1;
    let mut adj_edges: Vec<Vec<Edge>> = vec![Vec::new(); n];
    let mut dj: DjGraph = vec![Vec::new(); n];

    for (u, v) in edges {
        // unweighted graph, so just weight=1.0
        adj_edges[u].push(Edge::new(v, 1.0));
        dj[u].push((v, 1.0));
    }

    (adj_edges.into(), dj)
}

pub fn compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("BMSSP_vs_Dijkstra");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(5));

    // ---- synthetic graphs ----
    let inputs = [
        (50, 200),
        (100, 400),
        (200, 800),
        (400, 1_600),
        (1_000, 5_000),
    ];

    for (n, m) in inputs {
        let label = format!("synthetic_{}v_{}e", n, m);
        let (bm_graph, dj_graph) = gen_graph(n, m, 42);

        group.bench_function(BenchmarkId::new("BMSSP", &label), |b| {
            b.iter(|| {
                let mut sp = ShortestPath::new(bm_graph.clone());
                black_box(sp.get(0usize))
            });
        });

        group.bench_function(BenchmarkId::new("Dijkstra", &label), |b| {
            b.iter(|| {
                black_box(dijkstra(&dj_graph, 0usize))
            });
        });
    }

    // ---- real dataset ----
    let (bm_graph, dj_graph) = load_roadnet("data/roadNet-PA.txt");
    let label = "roadNet-PA";
    group.bench_function(BenchmarkId::new("BMSSP", &label), |b| {
        b.iter(|| {
            let mut sp = ShortestPath::new(bm_graph.clone());
            black_box(sp.get(0usize))
        });
    });
    group.bench_function(BenchmarkId::new("Dijkstra", &label), |b| {
        b.iter(|| {
            black_box(dijkstra(&dj_graph, 0usize))
        });
    });

    group.finish();
}

criterion_group!(benches, compare);
criterion_main!(benches);
