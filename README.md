# BMSSP - Breaking the Sorting Barrier for Single-Source Shortest Paths

A Rust implementation of the groundbreaking BMSSP (Bounded Multi-Source Shortest Path) algorithm from the paper ["Breaking the Sorting Barrier for Directed Single-Source Shortest Paths"](https://arxiv.org/pdf/2504.17033) by R. Duan, J. Mao, X. Mao, X. Shu, and L. Yin.

## Overview

This library implements the first algorithm to achieve a sub-quadratic time complexity for the single-source shortest path problem on directed graphs, breaking the long-standing "sorting barrier" that has limited performance since Dijkstra's algorithm.

### Key Achievements

- **Time Complexity**: $O(m \log^{2/3} n)$
- **Breaking the Sorting Barrier**: First algorithm to achieve better than $O(m + n \log n)$ complexity
- **Theoretical Breakthrough**: Represents a major advancement in algorithmic graph theory after decades of research

## Algorithm Description

The BMSSP algorithm uses a recursive divide-and-conquer approach with three main components:

1. **Find Pivots** (Algorithm 1): Identifies key vertices to partition the problem space
2. **Base Case** (Algorithm 2): Handles small subproblems using a modified Dijkstra approach
3. **BMSSP** (Algorithm 3): The main recursive algorithm that coordinates the solution

### Core Innovation

Instead of maintaining a single global priority queue like Dijkstra's algorithm, BMSSP:
- Recursively partitions the problem into bounded subproblems
- Uses specialized data structures to efficiently manage multiple search frontiers
- Achieves better complexity through careful control of recursion depth and problem size

## Installation

Add this to your `Cargo.toml`:

```bash
cargo add bmssp
```

## Usage

```rust
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
```

## Testing

You can check the example:

```bash
cargo run --example simple
```

Run the test suite:

```bash
cargo test
```

The implementation is verified against the AOJ GRL_1_A test cases to ensure correctness.

## Benchmarking

Run benchmarks with different type of graphs

```bash
cargo bench --bench bench_bmssp
```

If you want to profile the functions you can use

```bash
cargo bench --bench bench_bmssp -- --profile-time=30
```

## Development

### Building from Source

```bash
git clone https://github.com/lucas-montes/bmssp
cd bmssp
cargo build --release
```

### With Nix

This project includes a Nix flake for reproducible builds:

```bash
nix develop
```

## Limitations and Future Work

- Currently implements the basic algorithm without advanced optimizations
- The [`BlockHeap`](src/heaps.rs) uses a simplified implementation (BTreeSet) rather than the specialized data structure from Lemma 3.3
- Performance gains should be most noticeable on very large graphs
- Make it more idiomatic
- Add more tests
- Add comparaisons

## Contributing

Contributions are welcome! Areas for improvement:

- Implement the specialized data structure from Lemma 3.3
- Optimize memory usage and constant factors
- Add parallel processing capabilities

## Architecture

The library is organized into several key modules:

- **[`models.rs`](src/models.rs)**: Core data types ([`Vertex`](src/models.rs), [`Length`](src/models.rs), [`Edge`](src/models.rs), [`Graph`](src/models.rs))
- **[`heaps.rs`](src/heaps.rs)**: Priority queue implementations ([`Heap`](src/heaps.rs), [`BlockHeap`](src/heaps.rs))
- **[`shortest_path.rs`](src/shortest_path.rs)**: Main BMSSP algorithm implementation ([`ShortestPath`](src/shortest_path.rs))

### Core Algorithm Flow

1. **Initialization**: The [`ShortestPath::get`](src/shortest_path.rs) method sets up parameters and calls the main algorithm
2. **Recursive Decomposition**: [`ShortestPath::bmssp`](src/shortest_path.rs) recursively breaks down the problem
3. **Pivot Selection**: [`ShortestPath::find_pivots`](src/shortest_path.rs) identifies key vertices for partitioning
4. **Base Case**: [`ShortestPath::base_case`](src/shortest_path.rs) handles small subproblems with a modified Dijkstra approach

## Real-World Benchmarks 🚀 (Made by RuslanC)

To validate the implementation, we benchmarked **BMSSP** against **Dijkstra’s algorithm** using both synthetic graphs and a real road network.

### Synthetic Graphs (randomly generated)

| Graph Size (vertices/edges) | BMSSP (µs) | Dijkstra (µs) | Faster |
|-----------------------------|------------|---------------|--------|
| 50 / 200                    | 20.3       | 1.07          | Dijkstra |
| 100 / 400                   | 39.2       | 2.15          | Dijkstra |
| 200 / 800                   | 84.1       | 4.18          | Dijkstra |
| 400 / 1600                  | 166.6      | 8.64          | Dijkstra |
| 1000 / 5000                 | 452.5      | 27.6          | Dijkstra |

👉 On **small graphs**, Dijkstra is faster due to lower constant factors.

### Real Dataset: [roadNet-PA](http://snap.stanford.edu/data/roadNet-PA.html)

| Algorithm | Time (ms) |
|-----------|-----------|
| BMSSP     | **65.1** |
| Dijkstra  | 74.5     |

👉 On **roadNet-PA (1.09M vertices, 1.5M edges)**, BMSSP outperforms Dijkstra by ~15%, demonstrating its advantage on large-scale, real-world graphs.

---

📌 **Takeaway**: BMSSP shows its strength on massive graphs where the *sorting barrier* matters, while Dijkstra dominates on smaller inputs.
