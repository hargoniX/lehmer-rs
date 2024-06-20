use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use lehmer::core::{Crawford, FastU32, NaiveU32};
use lehmer::monte::estimate_pi;

const SEED: u64 = 333;

fn criterion_benchmark(c: &mut Criterion) {
    let param = 0;
    let mut group = c.benchmark_group("monte_carlo");

    group.bench_function(BenchmarkId::new("Crawford", param), |b| {
        b.iter(|| estimate_pi::<Crawford>());
    });
    group.bench_function(BenchmarkId::new("NaiveU32", param), |b| {
        b.iter(|| estimate_pi::<NaiveU32>());
    });
    group.bench_function(BenchmarkId::new("FastU32", param), |b| {
        b.iter(|| estimate_pi::<FastU32>());
    });
    group.finish();
}

criterion_group!(monte_carlo, criterion_benchmark);
criterion_main!(monte_carlo);
