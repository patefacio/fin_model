use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fin_model::core::YearRange;
use fin_model_gui::utils::year_clamp::{YearClamp, YearClampStrings};

pub fn year_range_benchmark(c: &mut Criterion) {
    let year_clamp = YearClamp::new(YearRange {
        start: 1900,
        end: 2300,
    });

    c.bench_function("YearClamp on (1900, 2300)", |b| {
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
        })
    });
}

pub fn year_range_strings_benchmark(c: &mut Criterion) {
    let year_clamp = YearClampStrings::new(YearRange {
        start: 1900,
        end: 2300,
    });

    c.bench_function("YearClampStrings on (1900, 2300)", |b| {
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
        })
    });
}

criterion_group!(benches, year_range_benchmark, year_range_strings_benchmark);
criterion_main!(benches);
