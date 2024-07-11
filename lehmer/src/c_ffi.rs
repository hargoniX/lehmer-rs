use crate::core::{CrayRanf, FastU32, Lemire};
use rand_core::RngCore;
use std::ffi::{c_char, CString};
use testu01_sys::{unif01_CreateExternGenBits, unif01_DeleteExternGenBits, unif01_Gen};

static mut SEED: u128 = 333;

#[no_mangle]
pub unsafe extern "C" fn lehmer_next() -> u32 {
    let mut rng = <Lemire>::new(SEED);
    let out = rng.next_u32();
    SEED = rng.state;
    out
}

pub fn crush_it(
    gen_func: unsafe extern "C" fn() -> u32,
    crush: unsafe extern "C" fn(*mut unif01_Gen),
) {
    let gen_name = CString::new("Lemire").unwrap();
    unsafe {
        let gen = unif01_CreateExternGenBits(gen_name.as_ptr() as *mut c_char, Some(gen_func));
        crush(gen);
        unif01_DeleteExternGenBits(gen);
    }
}
