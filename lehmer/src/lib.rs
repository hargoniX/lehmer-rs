#![feature(portable_simd)]
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

pub mod core;
pub mod monte;
pub mod test_data;
