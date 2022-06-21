use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sub(1-1)", |b| b.iter(|| two::sub(1,1)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);