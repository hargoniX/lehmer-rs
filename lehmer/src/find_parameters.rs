use modinverse::modinverse;
use rand_core::{RngCore, SeedableRng};

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while a != 0 {
        let x = b % a;
        b = a;
        a = x;
    }
    b
}

fn find_determinant(i: usize, j: usize, xs: &[u32]) -> u64 {
    let a_1 = xs[i] as i64 - xs[0] as i64;
    let b_1 = xs[i + 1] as i64 - xs[1] as i64;
    let a_2 = xs[j] as i64 - xs[0] as i64;
    let b_2 = xs[j + 1] as i64 - xs[1] as i64;

    i64::abs((a_1 * b_2) as i64 - (a_2 * b_1) as i64) as u64
}

pub fn find_lehmer_parameters<R: RngCore + SeedableRng>(rng: &mut R) -> (u32, u32) {
    // find m
    // George Marsaglia way:
    // - research paper: https://www.pnas.org/doi/pdf/10.1073/pnas.61.1.25
    // - blog entry for algorithm: https://srmore.io/posts/breaking-linear-congruential-generator/
    // exploit:
    // if n-tuples (z_i, ..., z_i+n) are viewed as points in unit cube of n dimensions, all points
    // lie in relatively small numbers of hyperplanes

    let xs: [u32; 10] = core::array::from_fn(|_| rng.next_u32());

    let mut ds = Vec::new();
    ds.push(find_determinant(1, 2, &xs));
    ds.push(find_determinant(2, 3, &xs));
    ds.push(find_determinant(3, 4, &xs));
    ds.push(find_determinant(4, 5, &xs));

    let m = ds.into_iter().reduce(gcd).unwrap() as u32;

    // find a by solving z_n+1 = a * z_n mod m -> a = z_n+1 * inverse(z_n) mod m
    let a = (xs[1] as i64 * modinverse(xs[0] as i64, m as i64).unwrap()) % m as i64;

    (m, a as u32)
}
