use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use lehmer::core::{Crawford, FastU32, NaiveU32};
use lehmer::mc_pi::{estimate_pi_n, estimate_pi_n_simd};

fn criterion_benchmark(c: &mut Criterion) {
    let param = 0;
    let mut group = c.benchmark_group("monte_carlo");

    let mut n = 2;

    group.bench_function(BenchmarkId::new("Crawford", param), |b| {
        b.iter(|| estimate_pi_n::<Crawford>(n));
    });
    group.bench_function(BenchmarkId::new("NaiveU32", param), |b| {
        b.iter(|| estimate_pi_n::<NaiveU32>(n));
    });
    group.bench_function(BenchmarkId::new("FastU32", param), |b| {
        b.iter(|| estimate_pi_n::<FastU32>(n));
    });

    group.bench_function(BenchmarkId::new("Crawford simd", param), |b| {
        b.iter(|| estimate_pi_n_simd::<Crawford>(n));
    });
    group.bench_function(BenchmarkId::new("NaiveU32 simd", param), |b| {
        b.iter(|| estimate_pi_n_simd::<NaiveU32>(n));
    });
    group.bench_function(BenchmarkId::new("FastU32 simd", param), |b| {
        b.iter(|| estimate_pi_n_simd::<FastU32>(n));
    });

    n = 16;
    group.bench_function(BenchmarkId::new("Crawford 16", param), |b| {
        b.iter(|| estimate_pi_n::<Crawford>(n));
    });
    group.bench_function(BenchmarkId::new("NaiveU32 15", param), |b| {
        b.iter(|| estimate_pi_n::<NaiveU32>(n));
    });
    group.bench_function(BenchmarkId::new("FastU32 16", param), |b| {
        b.iter(|| estimate_pi_n::<FastU32>(n));
    });

    group.bench_function(BenchmarkId::new("Crawford simd 16", param), |b| {
        b.iter(|| estimate_pi_n_simd::<Crawford>(n));
    });
    group.bench_function(BenchmarkId::new("NaiveU32 simd 16", param), |b| {
        b.iter(|| estimate_pi_n_simd::<NaiveU32>(n));
    });
    group.bench_function(BenchmarkId::new("FastU32 simd 16", param), |b| {
        b.iter(|| estimate_pi_n_simd::<FastU32>(n));
    });
    group.finish();
}

criterion_group!(monte_carlo, criterion_benchmark);
criterion_main!(monte_carlo);
