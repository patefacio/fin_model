
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fin_model::clamp::clamp;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("clamp(1984, 2300, 1900)", 
    |b| b.iter(|| clamp("1984", 2300, 1900)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
