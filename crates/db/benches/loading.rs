//! Benchmarks loading the database from data files.

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use opldb::OplDb;

use std::path::Path;

const LIFTERS_CSV: &str = "../../build/lifters.csv";
const MEETS_CSV: &str = "../../build/meets.csv";
const ENTRIES_CSV: &str = "../../build/entries.csv";

/// Counts the number of lines in a file, for establishing throughput.
fn count_lines_in(path: &str) -> usize {
    use std::io::BufRead;
    let file = std::fs::File::open(path).expect("a readable file");
    let reader = std::io::BufReader::new(file);
    reader.lines().count()
}

pub fn loading_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("loading");
    let num_entries = count_lines_in(ENTRIES_CSV);

    // Loading CSV files takes forever, so collect the minimum number of samples.
    // Otherwise, the benchmark takes something like 20 minutes to run.
    group.sample_size(10);

    group.throughput(Throughput::Elements(num_entries as u64));
    group.bench_function("csv", |b| {
        b.iter(|| {
            OplDb::from_csv(
                Path::new(LIFTERS_CSV),
                Path::new(MEETS_CSV),
                Path::new(ENTRIES_CSV),
            )
            .unwrap();
        });
    });
}

criterion_group!(benches, loading_benchmarks);
criterion_main!(benches);
