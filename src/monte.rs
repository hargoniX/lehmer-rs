use core::f64;
use rand::Rng;
use rand_core::SeedableRng;
use std::f64::consts::PI;

const SEED: u64 = 333;

pub fn estimate_pi<R: Rng + SeedableRng>() -> f64 {
    fn is_precision_reached(count: u32, iterations: u32) -> bool {
        let estimate: f64 = (count as f64) * 4.0 / (iterations as f64);
        f64::abs(estimate - PI) < 0.000000001
    }

    let mut rng = R::seed_from_u64(SEED);
    let mut iterations = 0;
    let mut count = 0;

    while !is_precision_reached(count, iterations) {
        // TODO: implement and use rand::distributions::Uniform? - more efficient when same range
        let x: f32 = rng.gen_range(0.0..1.0);
        let y: f32 = rng.gen_range(0.0..1.0);
        let p: f32 = x * x + y * y;

        iterations += 1;
        if p < 1.0 {
            count += 1;
        }
    }
    (count as f64) * 4.0 / (iterations as f64)
}
