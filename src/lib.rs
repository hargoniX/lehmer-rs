#![feature(portable_simd)]
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

pub mod core;
pub mod find_parameters;
pub mod mc_pi;
