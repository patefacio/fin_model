use criterion::{criterion_group, criterion_main, Criterion};
use fin_model_gui::utils::integer_clamp::{IntegerClamp, IntegerClampStrings};


pub fn clamp_integer_range_benchmark(c: &mut Criterion) {
    let year_clamp = IntegerClamp::new(1900..=2300);

    c.bench_function("ClampIntegerRange on (1900, 2300)", |b| {
        b.iter(|| {
            let _ = year_clamp.clamp("1");
            let _ = year_clamp.clamp("19");
            let _ = year_clamp.clamp("198");
            let _ = year_clamp.clamp("1984");
            let _ = year_clamp.clamp("2");
            let _ = year_clamp.clamp("23");
            let _ = year_clamp.clamp("230");
            let _ = year_clamp.clamp("2300");

            let _ = year_clamp.clamp("24");
            let _ = year_clamp.clamp("23001");
            let _ = year_clamp.clamp("240000");
            let _ = year_clamp.clamp("230010000");
        })
    });
}

pub fn clamp_integer_range_strings_benchmark(c: &mut Criterion) {
    let year_clamp = IntegerClampStrings::new(1900..=2300);

    c.bench_function("ClampIntegerRangeStrings on (1900, 2300)", |b| {
        b.iter(|| {
            let _ = year_clamp.clamp("1");
            let _ = year_clamp.clamp("19");
            let _ = year_clamp.clamp("198");
            let _ = year_clamp.clamp("1984");
            let _ = year_clamp.clamp("2");
            let _ = year_clamp.clamp("23");
            let _ = year_clamp.clamp("230");
            let _ = year_clamp.clamp("2300");

            let _ = year_clamp.clamp("24");
            let _ = year_clamp.clamp("23001");
            let _ = year_clamp.clamp("240000");
            let _ = year_clamp.clamp("230010000");
        })
    });
}

criterion_group!(
    benches,
    clamp_integer_range_benchmark,
    clamp_integer_range_strings_benchmark,
);

criterion_main!(benches);
