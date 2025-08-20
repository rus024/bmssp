use bmssp::{ShortestPath, Graph, Edge};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Duration;

mod datasets;
use datasets::{ensure_gz_decompressed, ensure_snap_txt};

type DjGraph = Vec<Vec<(usize, f64)>>;

#[derive(Copy, Clone, PartialEq)]
struct State {
    cost: f64,
    node: usize,
}
impl Eq for State {}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

fn dijkstra(adj: &DjGraph, s: usize) -> Vec<f64> {
    let n = adj.len();
    let mut dist = vec![f64::INFINITY; n];
    dist[s] = 0.0;
    let mut pq = BinaryHeap::new();
    pq.push(State { cost: 0.0, node: s });
    while let Some(State { cost, node }) = pq.pop() {
        if cost > dist[node] {
            continue;
        }
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

// Parse DIMACS shortest path .gr format: lines like
//   c ... (comments)
//   p sp <n> <m>
//   a <u> <v> <w>
// Nodes are 1-indexed in DIMACS files
fn load_dimacs_gr(path: &str, undirected: bool) -> (Graph, DjGraph) {
    let f = File::open(path).expect("open .gr file");
    let rdr = BufReader::new(f);

    let mut n: usize = 0;
    let mut edges: Vec<(usize, usize, f64)> = Vec::new();

    for line in rdr.lines() {
        let s = line.expect("read line");
        let t = s.trim();
        if t.is_empty() || t.starts_with('c') { continue; }
        if let Some(rest) = t.strip_prefix('p') {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            // expect: ["sp", n, m]
            if parts.len() >= 3 {
                n = parts[1].parse::<usize>().expect("n");
            }
        } else if let Some(rest) = t.strip_prefix('a') {
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() >= 3 {
                let u: usize = parts[0].parse().expect("u");
                let v: usize = parts[1].parse().expect("v");
                let w: f64 = parts[2].parse::<i64>().expect("w") as f64;
                edges.push((u, v, w));
            }
        }
    }

    // allocate n+1 to use 1-based indexing directly
    let mut bm_adj: Vec<Vec<Edge>> = vec![Vec::new(); n + 1];
    let mut dj_adj: DjGraph = vec![Vec::new(); n + 1];

    for (u, v, w) in edges {
        let w_f32 = w as f32;
        bm_adj[u].push(Edge::new(v, w_f32));
        dj_adj[u].push((v, w));
        if undirected {
            bm_adj[v].push(Edge::new(u, w_f32));
            dj_adj[v].push((u, w));
        }
    }

    (bm_adj.into(), dj_adj)
}

fn load_as_edgelist(path: &str, undirected: bool) -> (Graph, DjGraph) {
    let f = File::open(path).expect("open dataset");
    let rdr = BufReader::new(f);
    let mut edges: Vec<(usize, usize)> = Vec::new();
    let mut max_id = 0usize;

    for line in rdr.lines() {
        let line = line.expect("read line");
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let mut it = t.split_whitespace();
        let u: usize = match it.next().and_then(|x| x.parse().ok()) {
            Some(v) => v,
            None => continue,
        };
        let v: usize = match it.next().and_then(|x| x.parse().ok()) {
            Some(v) => v,
            None => continue,
        };
        edges.push((u, v));
        max_id = max_id.max(u).max(v);
    }

    let n = max_id + 1;
    let mut bm_adj: Vec<Vec<Edge>> = vec![Vec::new(); n];
    let mut dj_adj: DjGraph = vec![Vec::new(); n];

    for (u, v) in edges {
        bm_adj[u].push(Edge::new(v, 1.0));
        dj_adj[u].push((v, 1.0));
        if undirected {
            bm_adj[v].push(Edge::new(u, 1.0));
            dj_adj[v].push((u, 1.0));
        }
    }

    (bm_adj.into(), dj_adj)
}

fn gen_graph(n: usize, m: usize, seed: u64) -> (Graph, DjGraph) {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut bm: Vec<Vec<Edge>> = vec![Vec::new(); n];
    let mut dj: DjGraph = vec![Vec::new(); n];

    for _ in 0..m {
        let u = rng.gen_range(0..n);
        let mut v = rng.gen_range(0..n);
        if v == u {
            v = (v + 1) % n;
        }
        let w: f32 = 1.0;
        bm[u].push(Edge::new(v, w));
        dj[u].push((v, w as f64));
    }
    (bm.into(), dj)
}

fn compare_internet(c: &mut Criterion) {
    let mut group = c.benchmark_group("InternetTopologies");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(10));

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

    let datasets = [
        (
            "roadNet-PA",
            "https://snap.stanford.edu/data/roadNet-PA.txt.gz",
            true,
        ),
        (
            "roadNet-CA",
            "https://snap.stanford.edu/data/roadNet-CA.txt.gz",
            true,
        ),
        (
            "roadNet-TX",
            "https://snap.stanford.edu/data/roadNet-TX.txt.gz",
            true,
        ),
        (
            "as-733",
            "https://snap.stanford.edu/data/as-733.txt.gz",
            false,
        ),
    ];

    for (name, url_gz, undirected) in datasets {
        let path_buf = ensure_snap_txt(name, url_gz);
        let path_str = path_buf.to_string_lossy();
        let (bm_graph, dj_graph) = load_as_edgelist(&path_str, undirected);

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

    // DIMACS shortest path datasets (9th DIMACS) — directed, positive weights
    let dimacs = [
        (
            "USA-road-d.NY",
            "https://users.diag.uniroma1.it/challenge9/data/USA-road-d/USA-road-d.NY.gr.gz",
            false,
        ),
        (
            "USA-road-d.BAY",
            "https://users.diag.uniroma1.it/challenge9/data/USA-road-d/USA-road-d.BAY.gr.gz",
            false,
        ),
    ];

    for (name, url_gz, undirected) in dimacs {
        let gr_path = ensure_gz_decompressed(name, url_gz, "gr");
        let (bm_graph, dj_graph) = load_dimacs_gr(&gr_path.to_string_lossy(), undirected);

        group.bench_function(BenchmarkId::new("BMSSP", name), |b| {
            b.iter(|| {
                let mut sp = ShortestPath::new(bm_graph.clone());
                // use source 1 due to 1-based indexing
                black_box(sp.get(1usize))
            });
        });
        group.bench_function(BenchmarkId::new("Dijkstra", name), |b| {
            b.iter(|| black_box(dijkstra(&dj_graph, 1usize)));
        });
    }

    group.finish();
}

criterion_group!(benches, compare_internet);
criterion_main!(benches);
