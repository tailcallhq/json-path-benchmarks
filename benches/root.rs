use criterion::{criterion_group, criterion_main, Criterion};

mod get_path;

fn all_benchmarks(c: &mut Criterion) {
    get_path::criterion_benchmark(c);
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = all_benchmarks
}
criterion_main!(benches);
