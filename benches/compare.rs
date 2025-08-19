cd ~/bmssp

# 1) Overwrite benches/compare.rs with a known-good file
cat > benches/compare.rs <<'RS'
// benches/compare.rs
use bmssp::{ShortestPath, Graph, Edge};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// Build the same random graph in two shapes:
/// - BMSSP: Graph (your crate's Graph)
/// - Dijkstra: Vec<Vec<(to, weight)>> with f64 weights
fn make_graph_pair(n: usize, m: usize, seed: u64) -> (Graph, Vec<Vec<(usize, f64)>>) {
    // Graph is a wrapper; convert Vec<Vec<Edge>> -> Graph via .into()
    let mut bm: Graph = vec![Vec::<Edge>::new(); n].into();
    let mut dj: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n];

    let mut rng = StdRng::seed_from_u64(seed);
    for _ in 0..m {
        let u = rng.gen_range(0..n);
        let v = rng.gen_range(0..n);
        if u == v { continue; }
        let w64: f64 = rng.gen_range(1.0f64..10.0f64); // force f64

        bm[u].push(Edge::new(v, w64)); // BMSSP
        dj[u].push((v, w64));          // Dijkstra
    }
    (bm, dj)
}

#[derive(Copy, Clone, PartialEq)]
struct State { cost: f64, node: usize }
impl Eq for State {}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for State { fn partial_cmp(&self, o: &Self) -> Option<Ordering> { Some(self.cmp(o)) } }

fn dijkstra(adj: &Vec<Vec<(usize, f64)>>, s: usize) -> Vec<f64> {
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

pub fn compare(c: &mut Criterion) {
    let sizes = [
        (100, 400),
        (200, 800),
        (400, 1600),
        (1_000, 5_000),
        (5_000, 20_000),
        (10_000, 50_000),
    ];

    let mut group = c.benchmark_group("Compare_BMSSP_vs_Dijkstra");
    group.sample_size(40);

    for (n, m) in sizes {
        let label = format!("{}v_{}e", n, m);
        let (bm_graph, dj_graph) = make_graph_pair(n, m, 42);

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

    group.finish();
}

criterion_group!(benches, compare);
criterion_main!(benches);
RS

# 2) Confirm those two critical fixes are present
rg -n 'Vec::<Edge>|w64|1\.0f64\.\.10\.0f64' benches/compare.rs

# 3) Clean cache so Cargo can’t reuse stale bench code
cargo clean

# 4) Build & run the bench
cargo bench --bench compare -- --sample-size 40
