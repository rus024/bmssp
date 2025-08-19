// benches/compare.rs
use bmssp::{ShortestPath, Graph, Edge};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// --- If your Edge uses different field names (e.g. `len`), change these two helpers only.
#[inline]
fn edge_to(e: &Edge) -> usize { e.to }
#[inline]
fn edge_weight(e: &Edge) -> f64 { e.weight }

/// Random directed graph
fn make_graph(n: usize, m: usize, seed: u64) -> Graph {
    // Your Graph implements From<Vec<Vec<Edge>>>, so use .into()
    let mut g: Graph = vec![Vec::new(); n].into();
    let mut rng = StdRng::seed_from_u64(seed);
    for _ in 0..m {
        let u = rng.gen_range(0..n);
        let v = rng.gen_range(0..n);
        if u == v { continue; }
        let w: f64 = rng.gen_range(1.0f64..10.0f64); // <- f64, not f32
        g[u].push(Edge::new(v, w));
    }
    g
}

/// Plain Dijkstra over your Graph/Edge types
#[derive(Copy, Clone, PartialEq)]
struct State { cost: f64, node: usize }
impl Eq for State {}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for State { fn partial_cmp(&self, o:&Self)->Option<Ordering>{ Some(self.cmp(o)) } }

fn dijkstra(g: &Graph, s: usize) -> Vec<f64> {
    let n = g.len();
    let mut dist = vec![f64::INFINITY; n];
    dist[s] = 0.0;
    let mut pq = BinaryHeap::new();
    pq.push(State{ cost: 0.0, node: s });

    while let Some(State{ cost, node }) = pq.pop() {
        if cost > dist[node] { continue; }
        for e in &g[node] {
            let v = edge_to(e);
            let nd = cost + edge_weight(e);
            if nd < dist[v] {
                dist[v] = nd;
                pq.push(State{ cost: nd, node: v });
            }
        }
    }
    dist
}

/// Benchmark both on the same graphs
pub fn compare(c: &mut Criterion) {
    let sizes = [
        (100, 400),
        (200, 800),
        (400, 1_600),
        (1_000, 5_000),
        (5_000, 20_000),
        (10_000, 50_000),
    ];

    let mut group = c.benchmark_group("Compare_BMSSP_vs_Dijkstra");
    group.sample_size(40); // tune as needed

    for (n, m) in sizes {
        let label = format!("{}v_{}e", n, m);
        let g = make_graph(n, m, 42);

        group.bench_function(BenchmarkId::new("BMSSP", &label), |b| {
            let g_ref = &g;
            b.iter(|| {
                let mut sp = ShortestPath::new(g_ref.clone());
                black_box(sp.get(0usize))
            });
        });

        group.bench_function(BenchmarkId::new("Dijkstra", &label), |b| {
            let g_ref = &g;
            b.iter(|| {
                black_box(dijkstra(g_ref, 0usize))
            });
        });
    }

    group.finish();
}

criterion_group!(benches, compare);
criterion_main!(benches);
