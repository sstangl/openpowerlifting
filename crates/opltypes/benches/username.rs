//! Simple benchmarks to assess the speed of username generation.

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use opltypes::Username;

use std::hint::black_box;

pub fn username_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("username");

    // Tests the all-ASCII fast path (50 ASCII characters).
    let ascii_name = "1234567890".repeat(5);
    group.throughput(Throughput::Elements(ascii_name.len() as u64));
    group.bench_function("ascii", |b| {
        b.iter(|| {
            Username::from_name(black_box(&ascii_name)).unwrap();
        });
    });

    // Tests the UTF-8 path (50 UTF-8 characters).
    //
    // The string that's chosen is one where a single UTF-8 character
    // expands into two ASCII characters, doubling the resultant string size.
    let utf8_name = "þ".repeat(50);
    group.throughput(Throughput::Elements(utf8_name.len() as u64));
    group.bench_function("utf8", |b| {
        b.iter(|| {
            Username::from_name(black_box(&utf8_name)).unwrap();
        });
    });

    // Tests Japanese names written in Hiragana, which get normalized into Katakana.
    let hiragana_name = "なべやかん".repeat(10);
    group.throughput(Throughput::Elements(hiragana_name.len() as u64));
    group.bench_function("hiragana", |b| {
        b.iter(|| {
            Username::from_name(black_box(&hiragana_name)).unwrap();
        });
    });

    group.finish();
}

criterion_group!(benches, username_benchmarks);
criterion_main!(benches);
