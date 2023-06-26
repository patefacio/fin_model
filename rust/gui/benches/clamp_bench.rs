use criterion::{criterion_group, criterion_main, Criterion};
use fin_model_gui::utils::clamp::clamp_one_pass::clamp as clamp_one_pass;
use fin_model_gui::utils::year_clamp::YearClampStrings;
use plus_modeled::core::YearRange;

pub fn clamp_sm_benchmark(c: &mut Criterion) {
    use fin_model_gui::utils::clamp::clamp_sm::clamp;
    let min: u32 = 1900;
    let max: u32 = 2300;

    c.bench_function("ClampStateMachine on (1900, 2300)", |b| {
        b.iter(|| {
            let _ = clamp("1", min, max);
            let _ = clamp("19", min, max);
            let _ = clamp("198", min, max);
            let _ = clamp("1984", min, max);
            let _ = clamp("2", min, max);
            let _ = clamp("23", min, max);
            let _ = clamp("230", min, max);
            let _ = clamp("2300", min, max);

            let _ = clamp("24", min, max);
            let _ = clamp("23001", min, max);
            let _ = clamp("240000", min, max);
            let _ = clamp("230010000", min, max);
        })
    });
}

pub fn clamp_year_range_strings_benchmark(c: &mut Criterion) {
    let year_clamp = YearClampStrings::new(YearRange {
        start: 1900,
        end: 2300,
    });

    c.bench_function("ClampYearRangeStrings on (1900, 2300)", |b| {
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

pub fn clamp_one_pass_benchmark(c: &mut Criterion) {
    let min = 1980;
    let max = 2200;

    c.bench_function("ClampOnePass on (1900, 2300)", |b| {
        b.iter(|| {
            let _ = clamp_one_pass("1", min, max);
            let _ = clamp_one_pass("19", min, max);
            let _ = clamp_one_pass("198", min, max);
            let _ = clamp_one_pass("1984", min, max);
            let _ = clamp_one_pass("2", min, max);
            let _ = clamp_one_pass("23", min, max);
            let _ = clamp_one_pass("230", min, max);
            let _ = clamp_one_pass("2300", min, max);

            let _ = clamp_one_pass("24", min, max);
            let _ = clamp_one_pass("23001", min, max);
            let _ = clamp_one_pass("240000", min, max);
            let _ = clamp_one_pass("230010000", min, max);
        })
    });
}

pub fn clamp_parse_input_benchmark(c: &mut Criterion) {
    use fin_model_gui::utils::clamp::clamp_parse_input::clamp;

    let min = 1980;
    let max = 2200;

    c.bench_function("ClampParseInput on (1900, 2300)", |b| {
        b.iter(|| {
            let _ = clamp("1", max, min);
            let _ = clamp("19", max, min);
            let _ = clamp("198", max, min);
            let _ = clamp("1984", max, min);
            let _ = clamp("2", max, min);
            let _ = clamp("23", max, min);
            let _ = clamp("230", max, min);
            let _ = clamp("2300", max, min);

            let _ = clamp("24", max, min);
            let _ = clamp("23001", max, min);
            let _ = clamp("240000", max, min);
            let _ = clamp("230010000", max, min);
        })
    });
}

criterion_group!(
    benches,
    clamp_sm_benchmark,
    clamp_one_pass_benchmark,
    clamp_parse_input_benchmark,
    clamp_year_range_strings_benchmark,
);

criterion_main!(benches);
