use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use lehmer::core::NaiveParkMiller;

fn nth_park_miller(n: u64) {
    let seed = 1234;
    let mut rng = NaiveParkMiller::new(seed);
    for _ in 0..n {
        rng.next();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let size = 8096 * 128;
    let mut group = c.benchmark_group("from_elem");
    group.throughput(Throughput::Elements(size));
    group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
        b.iter(|| nth_park_miller(size));
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
