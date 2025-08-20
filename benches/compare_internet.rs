use bmssp::{ShortestPath, Graph, Edge};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;

// ---------- simple reference Dijkstra ----------
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

// ---------- helpers: parse edge lists (two ints per line) ----------
fn load_as_edgelist(path: &str, undirected: bool) -> (Graph, DjGraph) {
    // First pass: collect edges, figure out max node id
    let f = File::open(path).expect("open dataset");
    let rdr = BufReader::new(f);

    let mut edges: Vec<(usize, usize)> = Vec::new();
    let mut max_id = 0usize;

    for line in rdr.lines() {
        let line = line.expect("read line");
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') { continue; }
        // allow any whitespace separator (tabs or spaces)
        let mut it = t.split_whitespace();
        let u: usize = match it.next().and_then(|x| x.parse().ok()) { Some(v) => v, None => continue };
        let v: usize = match it.next().and_then(|x| x.parse().ok()) { Some(v) => v, None => continue };
        edges.push((u, v));
        max_id = max_id.max(u).max(v);
    }

    let n = max_id + 1;
    let mut bm_adj: Vec<Vec<Edge>> = vec![Vec::new(); n];
    let mut dj_adj: DjGraph = vec![Vec::new(); n];

    for (u, v) in edges {
        // weight 1.0 for all Internet topology edges
        bm_adj[u].push(Edge::new(v, 1.0));
        dj_adj[u].push((v, 1.0));
        if undirected {
            bm_adj[v].push(Edge::new(u, 1.0));
            dj_adj[v].push((u, 1.0));
        }
    }

    (bm_adj.into(), dj_adj)
}

// ---------- optional synthetic generator (kept small) ----------
fn gen_graph(n: usize, m: usize, seed: u64) -> (Graph, DjGraph) {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut bm: Vec<Vec<Edge>> = vec![Vec::new(); n];
    let mut dj: DjGraph = vec![Vec::new(); n];

    for _ in 0..m {
        let u = rng.random_range(0..n);
        let mut v = rng.random_range(0..n);
        if v == u { v = (v + 1) % n; }
        let w: f32 = 1.0; // keep unit weights
        bm[u].push(Edge::new(v, w));
        dj[u].push((v, w as f64));
    }
    (bm.into(), dj)
}

// ---------- the bench ----------
pub fn compare_internet(c: &mut Criterion) {
    let mut group = c.benchmark_group("InternetTopologies");
    // Heavy graphs: keep sample size modest
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(10));

    // (A) A tiny sanity check on synthetic
    for (n, m) in [(5_000, 20_000)] {
        let label = format!("synthetic_{}v_{}e", n, m);
        let (bm_graph, dj_graph) = gen_graph(n, m, 42);
        group.bench_function(BenchmarkId::new("BMSSP", &label), |b| {
            b.iter(|| {
                let mut sp = ShortestPath::new(bm_graph.clone());
                black_box(sp.get(0usize))
            });
        });
        group.bench_function(BenchmarkId::new("Dijkstra", &label), |b| {
            b.iter(|| black_box(dijkstra(&dj_graph, 0usize)));
        });
    }

    // (B) Real Internet topologies (undirected logical edges, unit weights)
    let datasets = [
    ("AS-733 (20000102)", "data/as-733.txt", true),  // undirected
    ("AS-skitter",        "data/as-skitter.txt", false), // directed
];

   for (name, path, undirected) in datasets {
    let (bm_graph, dj_graph) = load_as_edgelist(path, undirected);

        group.bench_function(BenchmarkId::new("BMSSP", name), |b| {
            b.iter(|| {
                let mut sp = ShortestPath::new(bm_graph.clone());
                black_box(sp.get(0usize))
            });
        });

        group.bench_function(BenchmarkId::new("Dijkstra", name), |b| {
            b.iter(|| black_box(dijkstra(&dj_graph, 0usize)));
        });
    }

    group.finish();
}

criterion_group!(benches, compare_internet);
criterion_main!(benches);
