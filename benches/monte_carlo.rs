use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use core::f64;
use lehmer::core::Crawford;
use rand::{Rng, RngCore};
use rand_core::SeedableRng;

const SEED: u64 = 333;
const PI_5_DECIMALS: u32 = 314159;

fn estimate_pi<R: Rng + RngCore + SeedableRng>() -> u32 {
    fn is_precision_reached(count: u32, iterations: u32) -> bool {
        let estimate: f64 = (count as f64) * 4.0 / (iterations as f64);
        println!("{}", estimate);
        (estimate * (u32::pow(10, 5)) as f64).trunc() as u32 == PI_5_DECIMALS
    }

    let mut rng = R::seed_from_u64(SEED);
    let mut iterations = 0;
    let mut count = 0;

    while !is_precision_reached(count, iterations) {
        // TODO: implement and use rand::distributions::Uniform? - more efficient when same range
        println!("{}", rng.next_u32());
        let x: f32 = rng.gen_range(0.0..1.0);
        let y: f32 = rng.gen_range(0.0..1.0);
        let p: f32 = x * x + y * y;
        println!("{}, {} = {}", x, y, p);

        iterations += 1;
        if p < 1.0 {
            count += 1;
        }
    }
    iterations
}

fn criterion_benchmark(c: &mut Criterion) {
    let param = 0;
    let mut group = c.benchmark_group("monte_carlo");

    group.bench_function(BenchmarkId::new("Crawford", param), |b| {
        b.iter(|| estimate_pi::<Crawford>());
    });
    group.finish();
}

criterion_group!(monte_carlo, criterion_benchmark);
criterion_main!(monte_carlo);
