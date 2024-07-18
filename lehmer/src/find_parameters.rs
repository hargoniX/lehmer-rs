use gcd::Gcd;
use modinverse::modinverse;
use rand_core::{RngCore, SeedableRng};

// find m
// George Marsaglia way:
// - research paper: https://www.pnas.org/doi/pdf/10.1073/pnas.61.1.25
// - blog entry for algorithm: https://srmore.io/posts/breaking-linear-congruential-generator/
// exploit:
// if n-tuples (z_i, ..., z_i+n) are viewed as points in unit cube of n dimensions, all points
// lie in relatively small numbers of hyperplanes
// modulus must be prime:
// https://www.scribd.com/document/65283191/How-to-crack-a-Linear-Congruential-Generator

fn find_determinant_u32(i: usize, j: usize, xs: &[u32]) -> u64 {
    let a_1 = xs[i] as i64 - xs[0] as i64;
    let b_1 = xs[i + 1] as i64 - xs[1] as i64;
    let a_2 = xs[j] as i64 - xs[0] as i64;
    let b_2 = xs[j + 1] as i64 - xs[1] as i64;

    i64::abs((a_1 * b_2) as i64 - (a_2 * b_1) as i64) as u64
}

pub fn find_lehmer_parameters_u32<R: RngCore + SeedableRng>(rng: &mut R) -> (u32, u32) {
    let xs: [u32; 6] = core::array::from_fn(|_| rng.next_u32());

    let mut ds = Vec::new();
    ds.push(find_determinant_u32(0, 1, &xs));
    ds.push(find_determinant_u32(1, 2, &xs));
    ds.push(find_determinant_u32(2, 3, &xs));
    ds.push(find_determinant_u32(3, 4, &xs));

    let m = ds.into_iter().reduce(|x, y| x.gcd(y)).unwrap() as u32;

    // find a by solving z_n+1 = a * z_n mod m -> a = z_n+1 * inverse(z_n) mod m
    let a = (xs[1] as i64 * modinverse(xs[0] as i64, m as i64).unwrap()) % m as i64;
    (m as u32, a as u32)
}

fn find_determinant_u64(i: usize, j: usize, xs: &[u64]) -> u128 {
    let a_1 = xs[i] as i128 - xs[0] as i128;
    let b_1 = xs[i + 1] as i128 - xs[1] as i128;
    let a_2 = xs[j] as i128 - xs[0] as i128;
    let b_2 = xs[j + 1] as i128 - xs[1] as i128;

    i128::abs((a_1 * b_2) as i128 - (a_2 * b_1) as i128) as u128
}

pub fn find_lehmer_parameters_u64<R: RngCore + SeedableRng>(rng: &mut R) -> (u64, u64) {
    let xs: [u64; 6] = core::array::from_fn(|_| rng.next_u64());

    let mut ds = Vec::new();
    ds.push(find_determinant_u64(0, 1, &xs));
    ds.push(find_determinant_u64(1, 2, &xs));
    ds.push(find_determinant_u64(2, 3, &xs));
    ds.push(find_determinant_u64(3, 4, &xs));

    let m = ds.into_iter().reduce(|x, y| x.gcd(y)).unwrap() as u64;

    // find a by solving z_n+1 = a * z_n mod m -> a = z_n+1 * inverse(z_n) mod m
    let a = (xs[1] as i128 * modinverse(xs[0] as i128, m as i128).unwrap()) % m as i128;

    (m as u64, a as u64)
}
