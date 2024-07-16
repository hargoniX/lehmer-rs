use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use lehmer::core::{Crawford, FastU32, NaiveU32};
use lehmer::monte_carlo_pi::{estimate_pi_n, estimate_pi_n_simd};

fn criterion_benchmark(c: &mut Criterion) {
    let param = 0;
    let mut group = c.benchmark_group("monte_carlo");

    let mut n = 2;
    let seed = 1;

    group.bench_function(BenchmarkId::new("Crawford", param), |b| {
        b.iter(|| estimate_pi_n::<Crawford>(n, seed));
    });
    group.bench_function(BenchmarkId::new("NaiveU32", param), |b| {
        b.iter(|| estimate_pi_n::<NaiveU32>(n, seed));
    });
    group.bench_function(BenchmarkId::new("FastU32", param), |b| {
        b.iter(|| estimate_pi_n::<FastU32>(n, seed));
    });

    group.bench_function(BenchmarkId::new("Crawford simd", param), |b| {
        b.iter(|| estimate_pi_n_simd::<Crawford, 2>(seed));
    });
    group.bench_function(BenchmarkId::new("NaiveU32 simd", param), |b| {
        b.iter(|| estimate_pi_n_simd::<NaiveU32, 2>(seed));
    });
    group.bench_function(BenchmarkId::new("FastU32 simd", param), |b| {
        b.iter(|| estimate_pi_n_simd::<FastU32, 2>(seed));
    });

    n = 8;
    group.bench_function(BenchmarkId::new("Crawford 8", param), |b| {
        b.iter(|| estimate_pi_n::<Crawford>(n, seed));
    });
    group.bench_function(BenchmarkId::new("NaiveU32 8", param), |b| {
        b.iter(|| estimate_pi_n::<NaiveU32>(n, seed));
    });
    group.bench_function(BenchmarkId::new("FastU32 8", param), |b| {
        b.iter(|| estimate_pi_n::<FastU32>(n, seed));
    });

    group.bench_function(BenchmarkId::new("Crawford simd 8", param), |b| {
        b.iter(|| estimate_pi_n_simd::<Crawford, 8>(seed));
    });
    group.bench_function(BenchmarkId::new("NaiveU32 simd 8", param), |b| {
        b.iter(|| estimate_pi_n_simd::<NaiveU32, 8>(seed));
    });
    group.bench_function(BenchmarkId::new("FastU32 simd 8", param), |b| {
        b.iter(|| estimate_pi_n_simd::<FastU32, 8>(seed));
    });
    group.finish();
}

criterion_group!(monte_carlo, criterion_benchmark);
criterion_main!(monte_carlo);
