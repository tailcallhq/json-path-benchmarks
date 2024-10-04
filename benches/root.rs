use criterion::{criterion_group, criterion_main, Criterion};

mod get_path;

fn all_benchmarks(c: &mut Criterion) {
    // bench_jsonpath::bench(c);
    // bench_jsonpath_rust::bench(c);
    // bench_jsonpath_rs::bench(c);
    // bench_custom::bench(c);
    get_path::criterion_benchmark(c);
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = all_benchmarks
}
criterion_main!(benches);
