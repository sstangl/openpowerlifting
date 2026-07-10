//! Simple benchmarks to assess the speed of weightkg functions.

use criterion::{Criterion, criterion_group, criterion_main};
use opltypes::WeightKg;

use std::hint::black_box;
use std::str::FromStr;

const CASES: [&str; 6] = ["", "123", "123.5", "-123.5", "123.45", "123.45678"];

pub fn weightkg_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("weightkg");

    for case in CASES {
        let test_name = format!("from_str(\"{case}\")");
        group.bench_function(test_name, |b| {
            b.iter(|| {
                WeightKg::from_str(black_box(case)).unwrap();
            })
        });
    }
    group.finish();
}

criterion_group!(benches, weightkg_benchmarks);
criterion_main!(benches);
