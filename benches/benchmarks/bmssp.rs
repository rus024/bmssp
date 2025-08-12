use crate::benchmarks::{config, helpers};
use bmssp::ShortestPath;

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

fn bench_random_graph(c: &mut Criterion) {
    let mut group = c.benchmark_group("Random_Graph");
    config::set_default_benchmark_configs(&mut group);

    let sizes = [
        (50, 150),   // Small
        (100, 400),  // Medium
        (200, 800),  // Large
        (400, 1600), // Extra Large
    ];

    for (vertices, edges) in sizes {
        let graph = helpers::generate_random_graph(vertices, edges, 100.0, 42);

        group.bench_with_input(
            BenchmarkId::new("random", format!("{}v_{}e", vertices, edges)),
            &graph,
            |b, graph| {
                b.iter(|| {
                    let mut sp = ShortestPath::new(graph.clone());
                    black_box(sp.get(0))
                });
            },
        );
    }

    group.finish();
}

fn bench_connected_graph(c: &mut Criterion) {
    let mut group = c.benchmark_group("Connected_Graph");
    config::set_default_benchmark_configs(&mut group);

    let sizes = [(50, 200), (100, 500), (200, 1000), (400, 2000)];

    for (vertices, edges) in sizes {
        let graph = helpers::generate_connected_graph(vertices, edges, 100.0, 42);

        group.bench_with_input(
            BenchmarkId::new("connected", format!("{}v_{}e", vertices, edges)),
            &graph,
            |b, graph| {
                b.iter(|| {
                    let mut sp = ShortestPath::new(graph.clone());
                    black_box(sp.get(0))
                });
            },
        );
    }

    group.finish();
}

fn bench_sparse_graph(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sparse_Graph");
    config::set_default_benchmark_configs(&mut group);

    let test_cases = [
        (100, 0.01), // 1% density
        (100, 0.05), // 5% density
        (200, 0.01),
        (200, 0.05),
    ];

    for (vertices, density) in test_cases {
        let graph = helpers::generate_sparse_graph(vertices, density, 100.0, 42);
        let edges = (vertices * (vertices - 1)) as f32 * density;

        group.bench_with_input(
            BenchmarkId::new(
                "sparse",
                format!("{}v_d{}_{}e", vertices, density, edges as usize),
            ),
            &graph,
            |b, graph| {
                b.iter(|| {
                    let mut sp = ShortestPath::new(graph.clone());
                    black_box(sp.get(0))
                });
            },
        );
    }

    group.finish();
}

fn bench_dense_graph(c: &mut Criterion) {
    let mut group = c.benchmark_group("Dense_Graph");
    config::set_default_benchmark_configs(&mut group);

    let sizes = [
        25, // Small dense graph
        40, // Medium dense graph
        60, // Large dense graph (careful with size due to O(n²) edges)
    ];

    for vertices in sizes {
        let graph = helpers::generate_dense_graph(vertices, 100.0, 42);
        let edges = vertices * (vertices - 1); // Complete graph minus self-loops

        group.bench_with_input(
            BenchmarkId::new("dense", format!("{}v_{}e", vertices, edges)),
            &graph,
            |b, graph| {
                b.iter(|| {
                    let mut sp = ShortestPath::new(graph.clone());
                    black_box(sp.get(0))
                });
            },
        );
    }

    group.finish();
}

fn bench(c: &mut Criterion) {
    bench_random_graph(c);
    bench_connected_graph(c);
    bench_sparse_graph(c);
    bench_dense_graph(c);
}

#[cfg(not(target_os = "windows"))]
criterion_group! {
    name = benches;
    config = config::get_default_profiling_configs();
    targets = bench
}
#[cfg(target_os = "windows")]
criterion_group!(benches, bench);

criterion_main!(benches);
