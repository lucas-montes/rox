use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::tree_walk::parse::benches,
    benchmarks::tree_walk::scan::benches,
    benchmarks::tree_walk::interpreter::benches,
}
