use crate::core::FastU32;
use rand_core::RngCore;
use std::sync::atomic::AtomicU32;

// Two variants for the rust ffi to be able to generate a new number each time:
// 1. static mut the generator
//   1. Mark stuff as const
//   2. Some funky Lazylock stuff
// 2. static mut a seed and the function uses that
static mut SEED: AtomicU32 = AtomicU32::new(333);

// FIXME: Cant really use this approach for u64 since testu01 doesnt support u64 chunks
#[no_mangle]
pub unsafe extern "C" fn lehmer_next() -> u32 {
    let seed = SEED.get_mut();
    let mut rng = <FastU32>::new(*seed);
    *seed = rng.next_u32();
    *seed
}
