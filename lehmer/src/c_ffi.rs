use crate::core::FastU32;
use rand_core::RngCore;
use std::ffi::{c_char, CString};
use std::sync::atomic::AtomicU32;
use testu01_sys::{unif01_CreateExternGenBits, unif01_DeleteExternGenBits, unif01_Gen};

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

pub fn crush_it(
    gen_func: unsafe extern "C" fn() -> u32,
    crush: unsafe extern "C" fn(*mut unif01_Gen),
) {
    let gen_name = CString::new("FastU32").unwrap();
    unsafe {
        let gen = unif01_CreateExternGenBits(gen_name.as_ptr() as *mut c_char, Some(gen_func));
        crush(gen);
        unif01_DeleteExternGenBits(gen);
    }
}
