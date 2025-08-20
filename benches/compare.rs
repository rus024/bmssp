use bmssp::{Graph, Edge, ShortestPath};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use rand::{rngs::StdRng, SeedableRng};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Duration;

// ---------------- Dijkstra (reference) ----------------
type DjGraph = Vec<Vec<(usize, f64)>>;

#[derive(Copy, Clone, PartialEq)]
struct State { cost: f64, node: usize }
impl Eq for State {}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // min-heap via reverse compare
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

// --------------- Load SNAP edge list ------------------
fn load_snap_roadnet<P: AsRef<Path>>(path: P) -> (Graph, DjGraph, usize, usize) {
    let f = File::open(&path).unwrap_or_else(|e| {
        panic!(
            "Failed to open {:?}: {e}\n\
             Put the dataset at data/roadNet-PA.txt (from https://snap.stanford.edu/data/roadNet-PA.html)",
            path.as_ref()
        )
    });
    let rdr = BufReader::new(f);

    // First pass: collect edges and find max node id
    let mut edges: Vec<(usize, usize)> = Vec::new();
    let mut max_id: usize = 0;
    for line in rdr.lines() {
        let line = line.unwrap();
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') { continue; }
        let mut it = t.split_whitespace();
        let u: usize = it.next().unwrap().parse().unwrap();
        let v: usize = it.next().unwrap().parse().unwrap();
        max_id = max_id.max(u).max(v);
        edges.push((u, v));
    }
    let n = max_id + 1;
    let m = edges.len();

    // Build both graph representations (directed, unweighted -> weight 1)
    let mut bm_adj: Vec<Vec<Edge>> = vec![Vec::new(); n];
    let mut dj_adj: DjGraph = vec![Vec::new(); n];
    for (u, v) in edges.into_iter() {
        bm_adj[u].push(Edge::new(v, 1.0f32));
        dj_adj[u].push((v, 1.0f64));
    }

    (bm_adj.into(), dj_adj, n, m)
}

// ------------------ Criterion bench -------------------
pub fn compare_real(c: &mut Criterion) {
    // Path relative to repo root
    let path = std::env::var("ROADNET_PATH").unwrap_or_else(|_| "data/roadNet-PA.txt".to_string());
    let (bm_graph, dj_graph, n, m) = load_snap_roadnet(&path);

    let mut group = c.benchmark_group("BMSSP_vs_Dijkstra_REAL(roadNet-PA)");
    // Keep it quick; you can bump if you want tighter stats
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(5));

    // Choose a source node. You can change to something else or randomize.
    let source: usize = 0.min(n - 1);

    // BM-SSP
    group.bench_function(BenchmarkId::new("BMSSP", format!("{n}v_{m}e_src{source}")), |b| {
        b.iter(|| {
            let mut sp = ShortestPath::new(bm_graph.clone());
            black_box(sp.get(source))
        });
    });

    // Dijkstra
    group.bench_function(BenchmarkId::new("Dijkstra", format!("{n}v_{m}e_src{source}")), |b| {
        b.iter(|| {
            black_box(dijkstra(&dj_graph, source))
        });
    });

    group.finish();
}

criterion_group!(benches, compare_real);
criterion_main!(benches);
