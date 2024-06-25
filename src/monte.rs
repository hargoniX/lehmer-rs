use core::f64;
use rand::Fill;
use rand::Rng;
use rand_core::SeedableRng;
use std::f64::consts::PI;
use std::simd::cmp::SimdPartialOrd;
use std::simd::Simd;

const SEED: u64 = 333;
const PRECISION: f64 = 0.000000001;

// note:
// lehmer rng works well with this simple monte carlo example
// "cristalline" stucture of generation generally problematic for monte carlo though
// see https://www.pnas.org/doi/pdf/10.1073/pnas.61.1.25

pub fn estimate_pi_simd<R: Rng + SeedableRng>() -> f64 {
    fn is_precision_reached(count: u32, iterations: u32) -> bool {
        let estimate: f64 = (count as f64) * 4.0 / (iterations as f64);
        f64::abs(estimate - PI) < PRECISION
    }

    let mut rng = R::seed_from_u64(SEED);
    let mut iterations = 0;
    let mut count = 0;

    let mut x_buff: [f32; 64] = [0.0; 64];
    let mut y_buff: [f32; 64] = [0.0; 64];

    let splat = Simd::splat(1.0);

    while !is_precision_reached(count, iterations) {
        Fill::try_fill(&mut x_buff, &mut rng).unwrap();
        Fill::try_fill(&mut y_buff, &mut rng).unwrap();

        let x = Simd::from(x_buff);
        let y = Simd::from(y_buff);
        let p = x * x + y * y;

        iterations += 64;
        count += p.simd_lt(splat).to_bitmask().count_ones();
    }
    (count as f64) * 4.0 / (iterations as f64)
}

pub fn estimate_pi<R: Rng + SeedableRng>() -> f64 {
    fn is_precision_reached(count: u32, iterations: u32) -> bool {
        let estimate: f64 = (count as f64) * 4.0 / (iterations as f64);
        f64::abs(estimate - PI) < PRECISION
    }

    let mut rng = R::seed_from_u64(SEED);
    let mut iterations = 0;
    let mut count = 0;

    while !is_precision_reached(count, iterations) {
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
