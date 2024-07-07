#![feature(portable_simd)]
#[cfg(test)]
#[macro_use]
extern crate quickcheck;
extern crate modinverse;
extern crate statrs;

pub mod core;
pub mod find_parameters;
pub mod mc_pi;
