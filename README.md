# 🚀 BMSSP: Breaking the Sorting Barrier for Shortest Paths

⚡ **A fast Rust implementation of BMSSP** — the first algorithm to outperform Dijkstra's $O(m + n \log n)$ on large-scale graphs.  
📜 Based on the 2024 breakthrough paper: [_"Breaking the Sorting Barrier for Directed Single-Source Shortest Paths"_](https://arxiv.org/pdf/2504.17033)  
🛠️ Author: [RuslanC](https://github.com/rus024)

---

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

---

## 📦 How to Use

```bash
cargo add bmssp
rust
Copy
Edit
use bmssp::{ShortestPath, Edge};

let mut graph = vec![vec![]; 4];
graph[0].push(Edge::new(1, 1.0));
graph[0].push(Edge::new(2, 4.0));
// ...

let mut sp = ShortestPath::new(graph);
let dist = sp.get(0);
println!("To node 3: {}", dist[3]);
🧪 Run Benchmarks
bash
Copy
Edit
cargo bench --bench compare_internet -- --sample-size 40
📌 Why This Matters
✅ First algorithm to break sorting barrier
✅ Recursively decomposes SSSP problem
✅ Outperforms Dijkstra on massive datasets
✅ Written in clean, modern Rust

yaml
Copy
Edit
