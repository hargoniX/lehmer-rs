use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use lehmer::core::{
    BoroshNiederreiter, Crawford, CrayRanf, FastU32, Fishman18, LEcuyer, NaiveParkMiller,
    NaiveParkMillerOld, NaiveU32, ParkMillerEfficient, Randu, Waterman, INMOS,
};
use rand_core::{RngCore, SeedableRng};

fn nth_park_miller<R: RngCore + SeedableRng>(n: u64) {
    let seed = 1234;
    let mut rng = R::seed_from_u64(seed);
    for _ in 0..n {
        rng.next_u32();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let size = 8096 * 16;
    let mut group = c.benchmark_group("from_elem");
    group.throughput(Throughput::Elements(size));

    group.bench_with_input(
        BenchmarkId::new("NaiveParkMillerOld", size),
        &size,
        |b, &size| {
            b.iter(|| nth_park_miller::<NaiveParkMillerOld>(size));
        },
    );

    group.bench_with_input(
        BenchmarkId::new("NaiveParkMiller", size),
        &size,
        |b, &size| {
            b.iter(|| nth_park_miller::<NaiveParkMiller>(size));
        },
    );

    group.bench_with_input(BenchmarkId::new("Randu", size), &size, |b, &size| {
        b.iter(|| nth_park_miller::<Randu>(size));
    });

    group.bench_with_input(BenchmarkId::new("Crawford", size), &size, |b, &size| {
        b.iter(|| nth_park_miller::<Crawford>(size));
    });

    group.bench_with_input(BenchmarkId::new("CrayRanf", size), &size, |b, &size| {
        b.iter(|| nth_park_miller::<CrayRanf>(size));
    });

    group.bench_with_input(
        BenchmarkId::new("ParkMillerEfficient", size),
        &size,
        |b, &size| {
            b.iter(|| nth_park_miller::<ParkMillerEfficient>(size));
        },
    );

    group.bench_with_input(
        BenchmarkId::new("BoroshNiederreiter", size),
        &size,
        |b, &size| {
            b.iter(|| nth_park_miller::<BoroshNiederreiter>(size));
        },
    );

    group.bench_with_input(BenchmarkId::new("INMOS", size), &size, |b, &size| {
        b.iter(|| nth_park_miller::<INMOS>(size));
    });

    group.bench_with_input(BenchmarkId::new("Waterman", size), &size, |b, &size| {
        b.iter(|| nth_park_miller::<Waterman>(size));
    });

    group.bench_with_input(BenchmarkId::new("Fishman18", size), &size, |b, &size| {
        b.iter(|| nth_park_miller::<Fishman18>(size));
    });

    group.bench_with_input(BenchmarkId::new("LEcuyer", size), &size, |b, &size| {
        b.iter(|| nth_park_miller::<LEcuyer>(size));
    });

    group.bench_with_input(BenchmarkId::new("NaiveU32", size), &size, |b, &size| {
        b.iter(|| nth_park_miller::<NaiveU32>(size));
    });

    group.bench_with_input(BenchmarkId::new("FastU32", size), &size, |b, &size| {
        b.iter(|| nth_park_miller::<FastU32>(size));
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
