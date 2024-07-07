use lehmer::core::{FastU32, NaiveParkMillerOld};
use lehmer::mc_pi::estimate_pi_n;
use lehmer::mc_pi::estimate_pi_n_simd;

fn main() {
    println!("pi: {:?}", estimate_pi_n_simd::<FastU32>(3));
}
