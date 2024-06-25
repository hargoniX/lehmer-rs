#![feature(portable_simd)]
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate modinverse;

pub mod core;
pub mod find_parameters;
pub mod monte;
