BMSSP – Breaking the Sorting Barrier for Single-Source Shortest Paths

Rust implementation of the BMSSP algorithm from the paper “Breaking the Sorting Barrier for Directed Single-Source Shortest Paths” by Duan, Mao, Shu, and Yin.

This is the first algorithm to beat the long-standing “sorting barrier” that has limited performance since Dijkstra’s algorithm.

✨ Why it matters

Time Complexity: O(m log^(2/3) n)

Sorting Barrier Broken: Faster than the classic O(m + n log n)

Big Deal: First real progress on shortest paths in decades

⚙️ How it works (in simple words)

Find pivots – choose key vertices to split the graph

Base case – solve small pieces with a modified Dijkstra

Recursive BMSSP – combine results, keeping searches bounded

Instead of one giant priority queue (like Dijkstra), BMSSP runs several smaller frontiers in parallel.
That’s how it gets better scaling on huge graphs.

🔧 Install & Run

Add to your Cargo.toml

cargo add bmssp

Build from source:

git clone https://github.com/rus024/bmssp
cd bmssp
cargo build --release


Try the simple example:

cargo run --example simple

Run the tests:

cargo test

Run the benchmarks:

cargo bench --bench bench_bmssp

🚀 Benchmark Results (my runs)

Synthetic graphs (randomly generated)

50 nodes / 200 edges → Dijkstra faster (1.07 µs vs 20.3 µs)

1000 nodes / 5000 edges → still Dijkstra (27.6 µs vs 452 µs)

Real dataset – Pennsylvania road network (1.09M nodes, 1.5M edges)

BMSSP → 65.1 ms

Dijkstra → 74.5 ms

✅ BMSSP wins on real-world scale with ~15% speedup.
❌ Dijkstra dominates on small toy graphs.

📌 Takeaway

Small graphs: use Dijkstra – simple, efficient

Massive networks: BMSSP pulls ahead and shows its strength

Status: this repo is a working Rust implementation + benchmarks to prove it

🛠 Contributing

Ideas to improve:

Replace simplified heap with the optimized version from the paper

Reduce memory overhead

Add more real-world datasets

Explore parallelism

🗂 Repo Structure

models.rs → graph & edge data types

heaps.rs → priority queue variants

shortest_path.rs → main BMSSP algorithm

🔍 Quick Recap

This repo:

Implements BMSSP in Rust

Benchmarks it vs. Dijkstra

Shows real dataset results (roadNet-PA)

Makes it easy for others to re-run and compare
