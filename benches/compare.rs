// benches/compare.rs

use bmssp::{ShortestPath, Graph, Edge};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use rand::rngs::{StdRng, SeedableRng};
use rand::Rng;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Duration;

// Dijkstra adjacency list uses f32 weights
type DjGraph = Vec<Vec<(usize, f32)>>;

// ------ Minimal Dijkstra (f32) ------
#[derive(Copy, Clone, PartialEq)]
struct State {
    cost: f32,
    node: usize,
}
impl Eq for State {}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(adj: &DjGraph, s: usize) -> Vec<f32> {
    let n = adj.len();
    let mut dist = vec![f32::INFINITY; n];
    dist[s] = 0.0;
    let mut pq = BinaryHeap::new();
    pq.push(State { cost: 0.0, node: s });

    while let Some(State { cost, node }) = pq.pop() {
        if cost > dist[node] { continue; }
        for &(v, w) in &adj[node] {
            if dist[v] > cost + w {
                dist[v] = cost + w;
                pq.push(State { cost: dist[v], node: v });
            }
        }
    }
    dist
}

// ----- graph generator used by both algos -----
fn gen_graph(n: usize, m: usize, seed: u64) -> (Graph, DjGraph) {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut bm: Graph = vec![Vec::<Edge>::new(); n].into();
    let mut dj: DjGraph = vec![Vec::new(); n];

    for _ in 0..m {
        let u = rng.gen_range(0..n);
        let mut v = rng.gen_range(0..n);
        if v == u { v = (v + 1) % n; }
        let w: f32 = rng.gen_range(1.0..10.0);
        bm[u].push(Edge::new(v, w));
        dj[u].push((v, w));
    }
    (bm, dj)
}

pub fn compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("BMSSP_vs_Dijkstra");
    group.sample_size(40);
    group.measurement_time(Duration::from_secs(8));

    let inputs = [
        (50, 200),
        (100, 400),
        (200, 800),
    ];

    for &(n, m) in &inputs {
        group.bench_with_input(BenchmarkId::new("BMSSP", n), &n, |b, &_n| {
            b.iter(|| {
                let (g, _) = gen_graph(n, m, 123);
                g.shortest_path(black_box(0));
            })
        });

        group.bench_with_input(BenchmarkId::new("Dijkstra", n), &n, |b, &_n| {
            b.iter(|| {
                let (_, g) = gen_graph(n, m, 123);
                dijkstra(&g, black_box(0));
            })
        });
    }

    group.finish();
}

criterion_group!(benches, compare);
criterion_main!(benches);
