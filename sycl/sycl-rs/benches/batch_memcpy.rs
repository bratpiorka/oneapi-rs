use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("test", |b| b.iter(|| 2 + 3));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
