//! Benchmarks loading the database from data files.

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use opldb::{Entry, Meet, OplDb, import_meets_csv};

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

pub fn metafed_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("metafederation");

    // Meet data is needed for processing the entries data.
    let meets: Vec<Meet> = import_meets_csv(Path::new(MEETS_CSV)).unwrap();

    // Open-code about half of import_entries_csv, prior to making the cache.
    let entries: Vec<Entry> = {
        let mut vec = Vec::with_capacity(5_000_000);

        let mut rdr = csv::ReaderBuilder::new()
            .quoting(false)
            .from_path(Path::new(ENTRIES_CSV))
            .unwrap();
        for entry in rdr.deserialize() {
            let entry: Entry = entry.expect("failed to deserialize Entry");
            vec.push(entry);
        }
        vec
    };

    // Making the MetaFederation cache takes about 5s (out of ~21s).
    group.sample_size(10);

    group.throughput(Throughput::Elements(entries.len() as u64));
    group.bench_function("make_cache", |b| {
        b.iter(|| opldb::MetaFederationCache::make(&meets, &entries))
    });
}

criterion_group!(benches, loading_benchmarks, metafed_benchmarks);
criterion_main!(benches);
