use cdf::cdf::Cdf;
use criterion::{criterion_group, criterion_main, Criterion};
use std::path::PathBuf;

fn criterion_benchmark(c: &mut Criterion) {
    let input_file: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "examples",
        "data",
        "test_alltypes.cdf",
    ]
    .iter()
    .collect();

    c.bench_function("read_cdf_test_alltypes", |b| {
        b.iter(|| Cdf::read_cdf_file(input_file.clone()))
    });

    let input_file2: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "examples",
        "data",
        "ulysses.cdf",
    ]
    .iter()
    .collect();

    c.bench_function("read_cdf_ulysses", |b| {
        b.iter(|| Cdf::read_cdf_file(input_file2.clone()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
