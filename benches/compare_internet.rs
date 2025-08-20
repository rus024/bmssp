use bmssp::{ShortestPath, Graph, Edge};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box, BatchSize};
use rand::{rngs::StdRng, Rng, SeedableRng}; // (not used here; kept if you add synthetic later)
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Duration;

type DjGraph = Vec<Vec<(usize, f64)>>;

#[derive(Copy, Clone, PartialEq)]
struct State { cost: f64, node: usize }
impl Eq for State {}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse (min-heap behavior)
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for State { fn partial_cmp(&self, o: &Self) -> Option<Ordering> { Some(self.cmp(o)) } }

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

/// Load a two-column edgelist (u v per line), ignoring comments/headers.
/// Weights are set to 1.0. Works for SNAP as-733 / as-skitter.
fn load_edgelist_unweighted<P: AsRef<Path>>(path: P) -> (Graph, DjGraph) {
    let f = File::open(&path).expect("cannot open edgelist");
    let r = BufReader::new(f);

    // First pass: find max node id
    let mut max_id: usize = 0;
    let mut edges: Vec<(usize, usize)> = Vec::new();

    for line in r.lines() {
        let line = match line { Ok(s) => s, Err(_) => continue };
        let s = line.trim();
        if s.is_empty() || s.starts_with('#') { continue; }
        // common SNAP headers like "FromNodeId\tToNodeId"
        if s.chars().next().map(|c| !c.is_ascii_digit()).unwrap_or(true) { continue; }

        let mut it = s.split_whitespace();
        let u: usize = match it.next().and_then(|x| x.parse().ok()) { Some(v) => v, None => continue };
        let v: usize = match it.next().and_then(|x| x.parse().ok()) { Some(v) => v, None => continue };
        edges.push((u, v));
        max_id = max_id.max(u).max(v);
    }

    let n = max_id + 1;
    let mut bm_adj: Vec<Vec<Edge>> = vec![Vec::new(); n];
    let mut dj_adj: DjGraph = vec![Vec::new(); n];

    // directed edges (SNAP AS graphs are directed)
    for (u, v) in edges {
        bm_adj[u].push(Edge::new(v, 1.0));
        dj_adj[u].push((v, 1.0));
    }

    let bm: Graph = bm_adj.into();
    (bm, dj_adj)
}

pub fn compare_internet(c: &mut Criterion) {
    let mut group = c.benchmark_group("BMSSP_vs_Dijkstra_Internet");
    // Keep runs short on huge graphs:
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(5));

    let datasets = [
        ("as-733", "data/as-733.txt"),
        ("as-skitter", "data/as-skitter.txt"),
        // If you add a Rocketfuel edge list later, put it here:
        // ("rocketfuel-3257", "data/rocketfuel-3257.edgelist"),
    ];

    for (name, path) in datasets {
        if !Path::new(path).exists() {
            eprintln!("[skip] {} not found at {}", name, path);
            continue;
        }
        eprintln!("[load] {} from {}", name, path);
        let (bm_graph, dj_graph) = load_edgelist_unweighted(path);

        // BMSSP: build ShortestPath each iter (it takes ownership of the graph)
        group.bench_function(BenchmarkId::new("BMSSP", name), |b| {
            b.iter_batched(
                || ShortestPath::new(bm_graph.clone()),
                |mut sp| { black_box(sp.get(0usize)); },
                BatchSize::SmallInput
            );
        });

        // Dijkstra: borrow the graph (no clone needed per-iter)
        group.bench_function(BenchmarkId::new("Dijkstra", name), |b| {
            b.iter(|| { black_box(dijkstra(&dj_graph, 0usize)); });
        });
    }

    group.finish();
}

criterion_group!(benches, compare_internet);
criterion_main!(benches);
