use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sub(10-5)", |b| b.iter(|| two::sub(10,5)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);