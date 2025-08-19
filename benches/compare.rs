// benches/compare.rs

use bmssp::{ShortestPath, Graph, Edge};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use rand::{rngs::StdRng, SeedableRng, Rng};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Duration;

// Dijkstra adjacency uses f32 so it matches Edge::new(_, f32)
type DjGraph = Vec<Vec<(usize, f32)>>;

// ---- Minimal Dijkstra (f32) ----
#[derive(Copy, Clone, PartialEq)]
struct State { cost: f32, node: usize }
impl Eq for State {}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for State { fn partial_cmp(&self, o: &Self) -> Option<Ordering> { Some(self.cmp(o)) } }

fn dijkstra(adj: &DjGraph, s: usize) -> Vec<f32> {
    let n = adj.len();
    let mut dist = vec![f32::INFINITY; n];
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

// ---- Build the same random graph in two shapes ----
fn gen_graph(n: usize, m: usize, seed: u64) -> (Graph, DjGraph) {
    let mut rng = StdRng::seed_from_u64(seed);

    // Build as Vec<Vec<Edge>> first (so we can mutate), then convert to Graph
    let mut bm_adj: Vec<Vec<Edge>> = vec![Vec::new(); n];
    let mut dj: DjGraph = vec![Vec::new(); n];

    for _ in 0..m {
        // use random_range to avoid deprecation warnings
        let u = rng.random_range(0..n);
        let mut v = rng.random_range(0..n);
        if v == u { v = (v + 1) % n; }
        let w: f32 = rng.random_range(1.0..10.0);

        bm_adj[u].push(Edge::new(v, w));
        dj[u].push((v, w));
    }

    let bm: Graph = bm_adj.into(); // convert once at the end
    (bm, dj)
}

pub fn compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("BMSSP_vs_Dijkstra");
    group.sample_size(40);
    group.measurement_time(Duration::from_secs(8));

    // (vertices, edges) — expand if you want bigger cases
    let inputs = [
        (50, 200),
        (100, 400),
        (200, 800),
    ];

    for &(n, m) in &inputs {
        let label = format!("{}v_{}e", n, m);
        let (bm_graph, dj_graph) = gen_graph(n, m, 123);

        // BMSSP (your crate)
        group.bench_function(BenchmarkId::new("BMSSP", &label), |b| {
            b.iter(|| {
                // if ShortestPath consumes the graph, clone; adjust if your API differs
                let mut sp = ShortestPath::new(bm_graph.clone());
                black_box(sp.get(0usize))
            });
        });

        // Dijkstra (baseline)
        group.bench_function(BenchmarkId::new("Dijkstra", &label), |b| {
            b.iter(|| {
                black_box(dijkstra(&dj_graph, 0usize))
            });
        });
    }

    group.finish();
}

criterion_group!(benches, compare);
criterion_main!(benches);
