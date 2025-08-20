use bmssp::{ShortestPath, Graph, Edge};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, black_box};
use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path};
use std::time::Duration;

type DjGraph = Vec<Vec<(usize, f64)>>;

/// Build BMSSP Graph + a simple adjacency list for our Dijkstra from an edge list file.
/// File format: whitespace-separated pairs "u v" per line. Lines starting with '#' ignored.
/// We remap arbitrary node ids to 0..n-1. If `undirected` is true, we add both directions.
fn load_edge_list<P: AsRef<Path>>(path: P, undirected: bool) -> (Graph, DjGraph) {
    let f = File::open(path.as_ref())
        .unwrap_or_else(|e| panic!("Cannot open {:?}: {e}", path.as_ref()));
    let br = BufReader::new(f);

    // Map external IDs -> compact 0..n-1
    let mut idmap: HashMap<u64, usize> = HashMap::new();
    let mut edges_raw: Vec<(usize, usize)> = Vec::new();
    let mut next = 0usize;

    for line in br.lines() {
        let line = line.unwrap();
        let s = line.trim();
        if s.is_empty() || s.starts_with('#') { continue; }
        // tolerate multi-space or tab
        let mut it = s.split_whitespace();
        let a = it.next().expect("bad edge line");
        let b = it.next().expect("bad edge line");
        let u_ext: u64 = a.parse().expect("non-integer node id");
        let v_ext: u64 = b.parse().expect("non-integer node id");

        let u = *idmap.entry(u_ext).or_insert_with(|| { let t = next; next += 1; t });
        let v = *idmap.entry(v_ext).or_insert_with(|| { let t = next; next += 1; t });
        edges_raw.push((u, v));
        if undirected {
            edges_raw.push((v, u));
        }
    }

    let n = next;
    let mut bm_adj: Vec<Vec<Edge>> = vec![Vec::new(); n];
    let mut dj_adj: DjGraph = vec![Vec::new(); n];
    for (u, v) in edges_raw {
        // unweighted → weight = 1.0
        bm_adj[u].push(Edge::new(v, 1.0));
        dj_adj[u].push((v, 1.0));
    }
    (bm_adj.into(), dj_adj)
}

/// A tiny in-file Dijkstra (unchanged from your earlier bench).
use std::cmp::Ordering;
use std::collections::BinaryHeap;
#[derive(Copy, Clone, PartialEq)]
struct State { cost: f64, node: usize }
impl Eq for State {}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
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

pub fn compare_internet(c: &mut Criterion) {
    let mut group = c.benchmark_group("InternetTopologies");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(8));

    // Put your files in ./data/ as edge lists (two ints per line).
    // We’ll treat SNAP AS graphs as UNDIRECTED (they’re effectively undirected at AS level).
    // If you know a file is directed, set undirected = false.
    let datasets = [
        // 1) CAIDA/SNAP (AS-level)
        ("as-733",          "data/as-733.txt",          true),
        ("as-skitter",      "data/as-skitter.txt",      true),

        // 2) Rocketfuel (router/ISP level) – you’ll convert to edge list below
        ("rocketfuel-3257", "data/rocketfuel-3257.edgelist", true),

        // 3) Internet Topology Zoo (GraphML→edge list) – see conversion below
        ("topozoo-Agis",    "data/topozoo-Agis.edgelist",    true),
    ];

    for (name, path, undirected) in datasets {
        if !Path::new(path).exists() {
            eprintln!("[skip] {name}: {path} not found");
            continue;
        }

        let (bm_graph, dj_graph) = load_edge_list(path, undirected);
        let label = format!("{} (n={}, m={})", name, bm_graph.len(),
            bm_graph.iter().map(|v| v.len()).sum::<usize>());

        group.bench_function(BenchmarkId::new("BMSSP", &label), |b| {
            b.iter(|| {
                let mut sp = ShortestPath::new(bm_graph.clone());
                black_box(sp.get(0usize))
            });
        });
        group.bench_function(BenchmarkId::new("Dijkstra", &label), |b| {
            b.iter(|| { black_box(dijkstra(&dj_graph, 0usize)) });
        });
    }

    group.finish();
}

criterion_group!(benches, compare_internet);
criterion_main!(benches);
