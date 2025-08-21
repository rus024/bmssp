BMSSP - Breaking the Sorting Barrier for Single-Source Shortest Paths
A Rust implementation of the groundbreaking BMSSP (Bounded Multi-Source Shortest Path) algorithm from the paper "Breaking the Sorting Barrier for Directed Single-Source Shortest Paths" by R. Duan, J. Mao, X. Mao, X. Shu, and L. Yin.

Overview
This library implements the first algorithm to achieve a sub-quadratic time complexity for the single-source shortest path problem on directed graphs, breaking the long-standing "sorting barrier" that has limited performance since Dijkstra's algorithm.

Key Achievements
Time Complexity: 
O
(
m
log
2
/
3
⁡
n
)
Breaking the Sorting Barrier: First algorithm to achieve better than 
O
(
m
+
n
log
⁡
n
)
 complexity
Theoretical Breakthrough: Represents a major advancement in algorithmic graph theory after decades of research
Algorithm Description
The BMSSP algorithm uses a recursive divide-and-conquer approach with three main components:

Find Pivots (Algorithm 1): Identifies key vertices to partition the problem space
Base Case (Algorithm 2): Handles small subproblems using a modified Dijkstra approach
BMSSP (Algorithm 3): The main recursive algorithm that coordinates the solution
Core Innovation
Instead of maintaining a single global priority queue like Dijkstra's algorithm, BMSSP:

Recursively partitions the problem into bounded subproblems
Uses specialized data structures to efficiently manage multiple search frontiers
Achieves better complexity through careful control of recursion depth and problem size
Installation
Add this to your Cargo.toml:

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

// Initialize the shortest path solver
let mut sp = ShortestPath::new(graph);

// Compute shortest paths from vertex 0
let distances = sp.get(0);

// distances[i] now contains the shortest distance from vertex 0 to vertex i
println!("Distance to vertex 1: {}", distances[1]); // Output: 1.0
println!("Distance to vertex 2: {}", distances[2]); // Output: 3.0
println!("Distance to vertex 3: {}", distances[3]); // Output: 4.0
Testing
You can check the example:

cargo run --example simple
Run the test suite:

cargo test
The implementation is verified against the AOJ GRL_1_A test cases to ensure correctness.

Benchmarking
Run benchmarks with different type of graphs

cargo bench --bench bench_bmssp
If you want to profile the functions you can use

cargo bench --bench bench_bmssp -- --profile-time=30
Development
Building from Source
git clone https://github.com/lucas-montes/bmssp
cd bmssp
cargo build --release
With Nix
This project includes a Nix flake for reproducible builds:

nix develop
Limitations and Future Work
Currently implements the basic algorithm without advanced optimizations
The BlockHeap uses a simplified implementation (BTreeSet) rather than the specialized data structure from Lemma 3.3
Performance gains should be most noticeable on very large graphs
Make it more idiomatic
Add more tests
Add comparaisons
Contributing
Contributions are welcome! Areas for improvement:

Implement the specialized data structure from Lemma 3.3
Optimize memory usage and constant factors
Add parallel processing capabilities
Architecture
The library is organized into several key modules:

models.rs: Core data types (Vertex, Length, Edge, Graph)
heaps.rs: Priority queue implementations (Heap, BlockHeap)
shortest_path.rs: Main BMSSP algorithm implementation (ShortestPath)
Core Algorithm Flow
Initialization: The ShortestPath::get method sets up parameters and calls the main algorithm
Recursive Decomposition: ShortestPath::bmssp recursively breaks down the problem
Pivot Selection: ShortestPath::find_pivots identifies key vertices for partitioning
Base Case: ShortestPath::base_case handles small subproblems with a modified Dijkstra approach

## 📊 Benchmark Results (Real + Synthetic)

| Dataset                 | 🔵 BMSSP Time | 🟢 Dijkstra Time | 🏆 Winner    |
|--------------------------|---------------|------------------|--------------|
| `roadNet-PA`            | 72.9 ms       | 80.9 ms          | ✅ **BMSSP** |
| `roadNet-CA`            | 134.9 ms      | 162.7 ms         | ✅ **BMSSP** |
| `roadNet-TX`            | 83.8 ms       | 98.9 ms          | ✅ **BMSSP** |
| `as-caida20071105`      | 1.50 ms       | 15.8 µs          | 🟢 Dijkstra  |
| `USA-road-d.NY`         | 124.5 ms      | 20.1 ms          | 🟢 Dijkstra  |
| `USA-road-d.BAY`        | 145.8 ms      | 24.9 ms          | 🟢 Dijkstra  |

> 🧠 **Takeaway:** BMSSP wins on real-world **large-scale** graphs. Dijkstra still rules on **small-to-medium** graphs.
