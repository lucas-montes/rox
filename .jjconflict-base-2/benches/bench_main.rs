use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::parse::benches,
    benchmarks::scan::benches,
    benchmarks::interpreter::benches,
}
