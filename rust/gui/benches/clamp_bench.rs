use criterion::{black_box, criterion_group, criterion_main, Criterion};
use plus_modeled::core::YearRange;
use fin_model_gui::utils::clamp_exp::{clamp, clamp2};
use fin_model_gui::utils::year_clamp::{YearClamp, YearClampStrings};


pub fn benchmark_st_machine(c: &mut Criterion) {
    use fin_model_gui::utils::clamp_exp::clamp::clamp as clamp_sm;
    let min: u32 = 1900;
    let max: u32 = 2300;

    c.bench_function("ClampStateMachine on (1900, 2300)", |b| {
        b.iter(|| {
            let _ = clamp_sm("1", min, max);
            let _ = clamp_sm("19", min, max);
            let _ = clamp_sm("198", min, max);
            let _ = clamp_sm("1984", min, max);
            let _ = clamp_sm("2", min, max);
            let _ = clamp_sm("23", min, max);
            let _ = clamp_sm("230", min, max);
            let _ = clamp_sm("2300", min, max);

            let _ = clamp_sm("24", min, max);
            let _ = clamp_sm("23001", min, max);
            let _ = clamp_sm("240000", min, max);
            let _ = clamp_sm("230010000", min, max);
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
            let _ = year_clamp.clamp("240000");
            let _ = year_clamp.clamp("230010000");
        })
    });
}

pub fn clamp_benchmark(c: &mut Criterion) {
    
    let max = 2200;
    let min = 1980;

    c.bench_function("YearClampStrings on (1900, 2300)", |b| {
        b.iter(|| {
            let _ = clamp::clamp("1", max, min);
            let _ = clamp::clamp("19", max, min);
            let _ = clamp::clamp("198", max, min);
            let _ = clamp::clamp("1984", max, min);
            let _ = clamp::clamp("2", max, min);
            let _ = clamp::clamp("23", max, min);
            let _ = clamp::clamp("230", max, min);
            let _ = clamp::clamp("2300", max, min);

            let _ = clamp::clamp("24", max, min);
            let _ = clamp::clamp("23001", max, min);
            let _ = clamp::clamp("240000", max, min);
            let _ = clamp::clamp("230010000", max, min);
        })
    });
}

pub fn clamp2_benchmark(c: &mut Criterion) {
    
    let max = 2200;
    let min = 1980;

    c.bench_function("clamp2 on (1900, 2300)", |b| {
        b.iter(|| {
            let _ = clamp2::clamp2("1", max, min);
            let _ = clamp2::clamp2("19", max, min);
            let _ = clamp2::clamp2("198", max, min);
            let _ = clamp2::clamp2("1984", max, min);
            let _ = clamp2::clamp2("2", max, min);
            let _ = clamp2::clamp2("23", max, min);
            let _ = clamp2::clamp2("230", max, min);
            let _ = clamp2::clamp2("2300", max, min);

            let _ = clamp2::clamp2("24", max, min);
            let _ = clamp2::clamp2("23001", max, min);
            let _ = clamp2::clamp2("240000", max, min);
            let _ = clamp2::clamp2("230010000", max, min);
        })
    });
}

criterion_group!(benches, year_range_strings_benchmark, clamp_benchmark, clamp2_benchmark);
criterion_main!(benches);
