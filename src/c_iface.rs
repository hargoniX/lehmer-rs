use crate::core::FastU32;
use rand_core::{RngCore, SeedableRng};

#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!");
}

const SEED: u64 = 333;

// FIXME: need a way to init the generator globally?
#[no_mangle]
pub extern "C" fn lehmer_next() -> u32 {
    let mut rng = <FastU32>::seed_from_u64(SEED);
    rng.next_u32()
}
