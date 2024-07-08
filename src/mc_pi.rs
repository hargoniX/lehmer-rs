use core::f64;
use rand::Fill;
use rand::Rng;
use rand_core::SeedableRng;
use statrs::function::gamma::gamma;
use std::f64::consts::PI;
use std::simd::cmp::SimdPartialOrd;
use std::simd::num::SimdFloat;
use std::simd::LaneCount;
use std::simd::Simd;
use std::simd::SupportedLaneCount;

const SEED: u64 = 333;
const PRECISION: f64 = 0.00001;

// FIXME: only check every 100 iterations

// note:
// lehmer rng works well with this simple monte carlo example
// "cristalline" stucture of generation generally problematic for monte carlo though
// see https://www.pnas.org/doi/pdf/10.1073/pnas.61.1.25

pub fn estimate_pi<R: Rng + SeedableRng>() -> f64 {
    fn is_precision_reached(estimate: f64) -> bool {
        f64::abs(estimate - PI) < PRECISION
    }
    fn estimate(count: u32, iterations: u32) -> f64 {
        (count as f64) * 4.0 / (iterations as f64)
    }

    let mut rng = R::seed_from_u64(SEED);
    let mut iterations = 0;
    let mut count = 0;

    while iterations % 64 != 0 || !is_precision_reached(estimate(count, iterations)) {
        let x: f32 = rng.gen_range(0.0..1.0);
        let y: f32 = rng.gen_range(0.0..1.0);
        let p: f32 = x * x + y * y;

        iterations += 1;
        if p < 1.0 {
            count += 1;
        }
    }
    estimate(count, iterations)
}

pub fn estimate_pi_n<R: Rng + SeedableRng>(n: usize) -> f64 {
    fn is_precision_reached(estimate: f64) -> bool {
        f64::abs(estimate - PI) < PRECISION
    }
    fn estimate(count: u32, iterations: u32, n: usize) -> f64 {
        // area of n-sphere / area of n-cube -> ratio of inside/outside of n-sphere
        // check whether generated point is outside of unit sphere using euclidian norm
        // (root canceled out)
        let nominator: f64 =
            count as f64 * gamma(n as f64 / 2.0 + 1.0) * u64::pow(2, n as u32) as f64;
        let demoniator: f64 = iterations as f64;
        f64::powf(nominator / demoniator, 2.0 / n as f64)
    }

    let mut rng = R::seed_from_u64(SEED);
    let mut iterations = 0;
    let mut count = 0;

    let mut vec = vec![0.0; n];
    let rns = vec.as_mut_slice();

    while iterations % 64 != 0 || !is_precision_reached(estimate(count, iterations, n)) {
        for i in 0..n {
            rns[i] = rng.gen_range(0.0..1.0);
        }

        let p: f32 = rns.iter().map(|rn| rn * rn).sum();

        iterations += 1;
        if p < 1.0 {
            count += 1;
        }
    }
    estimate(count, iterations, n)
}

pub fn estimate_pi_simd<R: Rng + SeedableRng>() -> f64 {
    fn is_precision_reached(estimate: f64) -> bool {
        f64::abs(estimate - PI) < PRECISION
    }
    fn estimate(count: u32, iterations: u32) -> f64 {
        (count as f64) * 4.0 / (iterations as f64)
    }

    let mut rng = R::seed_from_u64(SEED);
    let mut iterations = 0;
    let mut count = 0;

    let mut x_buff: [f32; 64] = [0.0; 64];
    let mut y_buff: [f32; 64] = [0.0; 64];

    let splat = Simd::splat(1.0);

    while !is_precision_reached(estimate(count, iterations)) {
        Fill::try_fill(&mut x_buff, &mut rng).unwrap();
        Fill::try_fill(&mut y_buff, &mut rng).unwrap();

        let x = Simd::from(x_buff);
        let y = Simd::from(y_buff);
        let p = x * x + y * y;

        iterations += 64;
        count += p.simd_lt(splat).to_bitmask().count_ones();
    }

    estimate(count, iterations)
}

// for n <= 64
pub fn estimate_pi_n_simd<R, const N: usize>() -> f64
where
    R: Rng + SeedableRng,
    LaneCount<N>: SupportedLaneCount,
{
    fn is_precision_reached(estimate: f64) -> bool {
        f64::abs(estimate - PI) < PRECISION
    }
    fn estimate(count: u32, iterations: u32, n: usize) -> f64 {
        // area of n-sphere / area of n-cube -> ratio of inside/outside of n-sphere
        // check whether generated point is outside of unit sphere using euclidian norm
        // (root canceled out)
        let nominator: f64 =
            count as f64 * gamma(n as f64 / 2.0 + 1.0) * u128::pow(2, n as u32) as f64;
        let demoniator: f64 = iterations as f64;
        f64::powf(nominator / demoniator, 2.0 / n as f64)
    }

    let mut rng = R::seed_from_u64(SEED);
    let mut iterations = 0;
    let mut count = 0;

    let mut rns: [f32; N] = [0.0; N];

    while iterations % 64 != 0 || !is_precision_reached(estimate(count, iterations, N)) {
        Fill::try_fill(&mut rns[0..N], &mut rng).unwrap();

        let xs = Simd::from(rns);
        let p = (xs * xs).reduce_sum();

        iterations += 1;
        if p < 1.0 {
            count += 1;
        }
    }

    estimate(count, iterations, N)
}
