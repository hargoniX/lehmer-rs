#![feature(portable_simd)]
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

pub mod c_ffi;
pub mod core;
pub mod monte;
pub mod test_data;
