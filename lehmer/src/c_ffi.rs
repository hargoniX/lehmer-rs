#[allow(unused_imports)]
use crate::core::{
    BoroshNiederreiter, Crawford, CrayRanf, FastU32, Fishman18, LEcuyer, Lehmer32, Lemire,
    NaiveParkMiller, NaiveParkMillerOld, NaiveU32, ParkMillerEfficient, Randu, Waterman, INMOS,
};
use rand_core::RngCore;
use std::ffi::{c_char, CString};
#[allow(unused_imports)]
use testu01_sys::{
    unif01_CreateExternGenBits, unif01_CreateTruncGen, unif01_DeleteExternGenBits, unif01_Gen,
};

static mut SEED_U32: u32 = 333;
static mut SEED_U64: u64 = 333;
static mut SEED_U128: u128 = 333;

// Sadly, cant deduplicate this code since its parameters have to be like that for the ffi.
// Also, since the testu01 API is restricted, we have to keep track of the state in an unpleasent way.
// Could use atomics for u32 and u64 (u128 not possible), but no real benefit.
#[no_mangle]
pub unsafe extern "C" fn lehmer_next_u32() -> u32 {
    let mut rng = <FastU32>::new(SEED_U32);
    let out = rng.next_u32();
    // The reversed version is also very much interesting since testu01 is not symmetric.
    //let out = u32::reverse_bits(out);

    // This distinction between manual and macro implementations is unfortunate.
    SEED_U32 = rng.state;
    out
}

#[no_mangle]
pub unsafe extern "C" fn lehmer_next_u64() -> u32 {
    let mut rng = <CrayRanf>::new(SEED_U64);
    let out = rng.next_u32();
    // The reversed version is also very much interesting since testu01 is not symmetric.
    //let out = u32::reverse_bits(out);

    // This distinction between manual and macro implementations is unfortunate.
    SEED_U64 = rng.rng.state;
    out
}

#[no_mangle]
pub unsafe extern "C" fn lehmer_next_u128() -> u32 {
    let mut rng = <Lemire>::new(SEED_U128);
    let out = rng.next_u32();
    // The reversed version is also very much interesting since testu01 is not symmetric.
    //let out = u32::reverse_bits(out);
    SEED_U128 = rng.state;
    out
}

pub fn crush_it(crush: unsafe extern "C" fn(*mut unif01_Gen)) {
    let gen_name = CString::new("Lemire").unwrap();
    let gen_func = lehmer_next_u128;
    unsafe {
        let gen = unif01_CreateExternGenBits(gen_name.as_ptr() as *mut c_char, Some(gen_func));
        // If the generator doesnt generate 32 bits with each call,
        // one can truncate the expected output to the n most significant bits.
        // Since we operate on u32s, this is only useful if we reverse the bitorder.
        //let gen = unif01_CreateTruncGen(gen, 31);
        crush(gen);
        unif01_DeleteExternGenBits(gen);
    }
}
