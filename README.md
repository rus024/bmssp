BMSSP - Breaking the Sorting Barrier for Single-Source Shortest Paths

A Rust implementation of the groundbreaking BMSSP (Bounded Multi-Source Shortest Path) algorithm from the paper "Breaking the Sorting Barrier for Directed Single-Source Shortest Paths"
 by R. Duan, J. Mao, X. Mao, X. Shu, and L. Yin.

Overview

This library implements the first algorithm to achieve a sub-quadratic time complexity for the single-source shortest path (SSSP) problem on directed graphs, breaking the long-standing sorting barrier that has limited performance since Dijkstra’s algorithm.

Key Achievements

Time Complexity: $O(m \log^{2/3} n)$

Sorting Barrier: First algorithm to beat $O(m + n \log n)$ complexity

Theoretical Breakthrough: Major advance in algorithmic graph theory after decades of research

Algorithm Description

BMSSP uses a recursive divide-and-conquer approach with three components:

Find Pivots – partition the problem

Base Case – handle small subproblems via modified Dijkstra

BMSSP Main – recursive coordination of subproblems

Innovation vs. Dijkstra

No single global priority queue

Multiple bounded subproblems

Specialized data structures manage frontiers efficiently

Installation
cargo add bmssp

Usage
use bmssp::{ShortestPath, Graph, Edge};

// Create a graph
let mut graph = vec![Vec::new(); 4];
graph[0].push(Edge::new(1, 1.0));
graph[0].push(Edge::new(2, 4.0));
graph[1].push(Edge::new(2, 2.0));
graph[1].push(Edge::new(3, 5.0));
graph[2].push(Edge::new(3, 1.0));

let mut sp = ShortestPath::new(graph);
let distances = sp.get(0);

println!("Distance to vertex 1: {}", distances[1]);
println!("Distance to vertex 2: {}", distances[2]);
println!("Distance to vertex 3: {}", distances[3]);

Testing
cargo run --example simple
cargo test

Benchmarking

Run with synthetic or real graphs:

cargo bench --bench bench_bmssp


Profile mode:

cargo bench --bench bench_bmssp -- --profile-time=30

Real-World Benchmarks 🚀 (by RuslanC)

We benchmarked BMSSP vs. Dijkstra on both synthetic and real datasets.

Synthetic Graphs (random)
Graph Size (V/E)	BMSSP (µs)	Dijkstra (µs)	Faster
50 / 200	20.3	1.07	Dijkstra
100 / 400	39.2	2.15	Dijkstra
200 / 800	84.1	4.18	Dijkstra
400 / 1600	166.6	8.64	Dijkstra
1000 / 5000	452.5	27.6	Dijkstra

👉 On small graphs, Dijkstra dominates due to lower constants.

Real Dataset: roadNet-PA
Algorithm	Time (ms)
BMSSP	65.1
Dijkstra	74.5

👉 On 1.09M vertices, 1.5M edges, BMSSP is ~15% faster.

Extended Benchmarks (RuslanC, Aug 2025)

Command used:

cargo bench --bench compare_internet -- --sample-size 40

Dataset	BMSSP	Dijkstra	Faster
roadNet-PA	72.9 ms	80.9 ms	BMSSP (~10%)
roadNet-CA	134.9 ms	162.7 ms	BMSSP (~17%)
roadNet-TX	83.8 ms	98.9 ms	BMSSP (~15%)
as-caida20071105	1.50 ms	15.8 µs	Dijkstra
USA-road-d.NY	124.5 ms	20.1 ms	Dijkstra
USA-road-d.BAY	145.8 ms	24.9 ms	Dijkstra

👉 Summary

BMSSP pulls ahead on massive graphs (state-wide road networks).

Dijkstra still wins on small or medium datasets.

Confirms theory: BMSSP = big-graph algorithm.

Development
git clone https://github.com/rus024/bmssp
cd bmssp
cargo build --release


Optional (Nix):

nix develop

Limitations / Future Work

BlockHeap simplified (BTreeSet, not Lemma 3.3 structure)

Optimizations + parallelization TBD

Performance boost only visible at large scales

Contributing

Contributions welcome:

Specialized heap implementation

Memory optimization

Parallel/async version

📌 Takeaway:
BMSSP = theory-driven breakthrough.
Dijkstra = small-scale king.
Together, they set the baseline for shortest-path algorithms in practice.
