//! Simple benchmarks to assess the speed of coefficient calculation.

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use opltypes::*;

use std::hint::black_box;

pub fn coefficient_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("coefficients");

    // Constants used in the benchmarks below, to keep them all the same.
    const BODYWEIGHT_MEN: WeightKg = WeightKg::from_i32(93);
    const TOTAL_MEN: WeightKg = WeightKg::from_i32(93);

    const BODYWEIGHT_WOMEN: WeightKg = WeightKg::from_i32(63);
    const TOTAL_WOMEN: WeightKg = WeightKg::from_i32(475);

    const AGE: Age = Age::Exact(55);

    // Each test benchmarks the calculation for a single entry.
    group.throughput(Throughput::Elements(1));

    // Benchmarks AH.
    group.bench_function("ah", |b| {
        b.iter(|| {
            coefficients::ah(
                black_box(Sex::M),
                black_box(BODYWEIGHT_MEN),
                black_box(TOTAL_MEN),
            );
        });
    });

    // Benchmarks Dots.
    group.bench_function("dots", |b| {
        b.iter(|| {
            coefficients::dots(
                black_box(Sex::M),
                black_box(BODYWEIGHT_MEN),
                black_box(TOTAL_MEN),
            );
        });
    });

    // Benchmarks Glossbrenner for men and women, since they have different implementations.
    group.bench_function("glossbrenner men", |b| {
        b.iter(|| {
            coefficients::glossbrenner(
                black_box(Sex::M),
                black_box(BODYWEIGHT_MEN),
                black_box(TOTAL_MEN),
            );
        });
    });
    group.bench_function("glossbrenner women", |b| {
        b.iter(|| {
            coefficients::glossbrenner(
                black_box(Sex::F),
                black_box(BODYWEIGHT_WOMEN),
                black_box(TOTAL_WOMEN),
            );
        });
    });

    // Benchmarks IPF Goodlift Points.
    group.bench_function("goodlift", |b| {
        b.iter(|| {
            coefficients::goodlift(
                black_box(Sex::M),
                black_box(Equipment::Raw),
                black_box(Event::sbd()),
                black_box(BODYWEIGHT_MEN),
                black_box(TOTAL_MEN),
            );
        });
    });

    // Benchmarks Hoffman.
    group.bench_function("hoffman", |b| {
        b.iter(|| {
            coefficients::hoffman(black_box(BODYWEIGHT_MEN), black_box(TOTAL_MEN));
        });
    });

    // Benchmarks IPF Points.
    group.bench_function("ipf", |b| {
        b.iter(|| {
            coefficients::ipf(
                black_box(Sex::M),
                black_box(Equipment::Raw),
                black_box(Event::sbd()),
                black_box(BODYWEIGHT_MEN),
                black_box(TOTAL_MEN),
            );
        });
    });

    // Benchmarks McCulloch.
    group.bench_function("mcculloch", |b| {
        b.iter(|| {
            coefficients::mcculloch(
                black_box(Sex::M),
                black_box(BODYWEIGHT_MEN),
                black_box(TOTAL_MEN),
                black_box(AGE),
            );
        });
    });

    // Benchmarks NASA Points.
    group.bench_function("nasa", |b| {
        b.iter(|| {
            coefficients::nasa(black_box(BODYWEIGHT_MEN), black_box(TOTAL_MEN));
        });
    });

    // Benchmarks Reshel.
    group.bench_function("reshel", |b| {
        b.iter(|| {
            coefficients::reshel(
                black_box(Sex::M),
                black_box(BODYWEIGHT_MEN),
                black_box(TOTAL_MEN),
            );
        });
    });

    // Benchmarks Schwartz/Malone for men and women, since they have different implementations.
    group.bench_function("schwartzmalone men", |b| {
        b.iter(|| {
            coefficients::schwartzmalone(
                black_box(Sex::M),
                black_box(BODYWEIGHT_MEN),
                black_box(TOTAL_MEN),
            );
        });
    });
    group.bench_function("schwartzmalone women", |b| {
        b.iter(|| {
            coefficients::schwartzmalone(
                black_box(Sex::F),
                black_box(BODYWEIGHT_WOMEN),
                black_box(TOTAL_WOMEN),
            );
        });
    });

    // Benchmarks Wilks.
    group.bench_function("wilks", |b| {
        b.iter(|| {
            coefficients::wilks(
                black_box(Sex::M),
                black_box(BODYWEIGHT_MEN),
                black_box(TOTAL_MEN),
            );
        });
    });

    // Benchmarks Wilks2020.
    group.bench_function("wilks2020", |b| {
        b.iter(|| {
            coefficients::wilks2020(
                black_box(Sex::M),
                black_box(BODYWEIGHT_MEN),
                black_box(TOTAL_MEN),
            );
        });
    });

    group.finish();
}

criterion_group!(benches, coefficient_benchmarks);
criterion_main!(benches);
