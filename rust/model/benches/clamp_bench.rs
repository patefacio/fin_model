use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fin_model::clamp::clamp;

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
        })
    });
}

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

pub fn bench_clamp_sm(c: &mut Criterion) {
    use fin_model::clamp_sm::clamp;
    let start = 1900;
    let end = 2300;
    c.bench_function("clamp_sm(1984, 1900, 2300)", |b| {
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

pub fn bench_clamp_parse(c: &mut Criterion) {
    use fin_model::clamp_sm::clamp_parse as clamp;
    let start = 1900;
    let end = 2300;
    c.bench_function("clamp_sm(1984, 1900, 2300)", |b| {
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

criterion_group!(
    benches,
    bench_clamp_bool,
    bench_clamp_string,
    bench_clamp_sm,
    bench_clamp_parse
);
criterion_main!(benches);
