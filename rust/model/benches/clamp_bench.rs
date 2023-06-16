use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fin_model::clamp_ext::clamp;

pub fn bench_clamp_bool(c: &mut Criterion) {
    let start = 1900;
    let end = 2300;
    c.bench_function("clamp_bool(1984, 1900, 2300)", |b| {
        b.iter(|| {
            clamp("1984", start, end);

            clamp("1", start, end);
            clamp("19", start, end);
            clamp("198", start, end);
            clamp("2", start, end);
            clamp("23", start, end);
            clamp("230", start, end);
            clamp("2300", start, end);

            clamp("24", start, end);
            clamp("23001", start, end);
            //clamp("240000", start, end);
            //clamp("230010000", start, end);
        })
    });
}
/*
pub fn bench_clamp_string(c: &mut Criterion) {
    use fin_model::clamp_ext::clamp;
    let start = 1900;
    let end = 2300;
    c.bench_function("clamp_string(1984, 1900, 2300)", |b| {
        b.iter(|| {
            clamp("1984", start, end);

            clamp("1", start, end);
            clamp("19", start, end);
            clamp("198", start, end);
            clamp("2", start, end);
            clamp("23", start, end);
            clamp("230", start, end);
            clamp("2300", start, end);

            clamp("24", start, end);
            clamp("23001", start, end);
        })
    });
}
*/

criterion_group!(benches, bench_clamp_bool);
criterion_main!(benches);
