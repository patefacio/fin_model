use criterion::{criterion_group, criterion_main, Criterion};
use fin_model_gui::utils::live_parsed_date::{clean_date, LiveParsedDate};
use std::hint::black_box;

pub fn clean_date_benchmark(c: &mut Criterion) {
    let dates = [
        "1",
        "1/",
        "12/",
        "12/1",
        "12/12",
        "12/12/1",
        "12/12/19",
        "12/12/199",
        "12/12/1999",
    ]
    .iter()
    .map(|d| d.to_string())
    .collect::<Vec<_>>();

    c.bench_function("Cleaning dates", |b| {
        b.iter(|| {
            for d in dates.iter() {
                let _ = clean_date(d.clone());
            }
        })
    });
}

pub fn scrub_date_benchmark(c: &mut Criterion) {
    let dates = [
        "1",
        "1/",
        "12/",
        "12/1",
        "12/12",
        "12/12/1",
        "12/12/19",
        "12/12/199",
        "12/12/1999",
    ]
    .iter()
    .map(|d| d.to_string())
    .collect::<Vec<_>>();

    let mut live_parsed_date = LiveParsedDate::new(None);

    c.bench_function("Cleaning dates", |b| {
        b.iter(|| {
            for d in dates.iter() {
                // following clone not required but there to make more fair comparison
                let (new_value, new_date, position) =
                    black_box(live_parsed_date.parse_date(&d.clone(), 0));
                tracing::debug!("{new_value:?}, {position} {new_date:?}");
            }
        })
    });
}

criterion_group!(benches, clean_date_benchmark, scrub_date_benchmark,);

criterion_main!(benches);
