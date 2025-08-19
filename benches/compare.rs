// benches/compare.rs
use bmssp::{ShortestPath, Graph, Edge, Length};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use rand::{rngs::StdRng, SeedableRng, Rng};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Duration;

// adjacency for Dijkstra: (to, weight) with the SAME Length type as the crate (f32)
type DjGraph = Vec<Vec<(usize, Length)>>;

// ----- Dijkstra using Length (f32) -----
#[derive(Copy, Clone, PartialEq)]
struct State { cost: Length, node: usize }
impl Eq for State {}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> { Some(self.cmp(o)) }
}

fn dijkstra(adj: &DjGraph, s: usize) -> Vec<Length> {
    let n = adj.len();
    let mut dist = vec![Length::INFINITY; n];
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

// ----- graph generator used by both algos -----
fn gen_graph(n: usize, m: usize, seed: u64) -> (Graph, DjGraph) {
    let mut rng = StdRng::seed_from_u64(seed);
    // Graph is a wrapper around Vec<Vec<Edge>> -> convert with .into()
    let mut bm: Graph = vec![Vec::<Edge>::new(); n].into();
    let mut dj: DjGraph = vec![Vec::new(); n];

    for _ in 0..m {
        let u = rng.random_range(0..n);
        let mut v = rng.random_range(0..n);
        if v == u { v = (v + 1) % n; }
        let w: Length = rng.random_range(1.0..10.0); // Length == f32

        bm[u].push(Edge::new(v, w));
        dj[u].push((v, w));
    }
    (bm, dj)
}

pub fn compare(c: &mut Criterion) {
    // keep this name stable; your speedup script looks for these labels
    let mut group = c.benchmark_group("BMSSP_vs_Dijkstra");
    group.sample_size(40);
    group.measurement_time(Duration::from_secs(8));

    // (vertices, edges) — add larger cases as you like
    let inputs = [
        (50, 200),
        (100, 400),
        (200, 800),
        (400, 1_600),
        (1_000, 5_000),
        (5_000, 20_000),
        (10_000, 50_000),
    ];

    for (n, m) in inputs {
        let (bm_graph, dj_graph) = gen_graph(n, m, 42);
        let label = format!("{}v_{}e", n, m);

        group.bench_function(BenchmarkId::new("BMSSP", &label), |b| {
            let g = bm_graph.clone();
            b.iter(|| {
                let mut sp = ShortestPath::new(g.clone());
                black_box(sp.get(0usize))
            });
        });

        group.bench_function(BenchmarkId::new("Dijkstra", &label), |b| {
            let g = dj_graph.clone();
            b.iter(|| {
                black_box(dijkstra(&g, 0usize))
            });
        });
    }

    group.finish();
}

criterion_group!(benches, compare);
criterion_main!(benches);
