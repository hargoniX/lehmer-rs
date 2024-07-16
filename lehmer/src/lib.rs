#![feature(portable_simd)]
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

pub mod c_ffi;
pub mod core;
pub mod find_parameters;
pub mod monte_carlo_pi;
pub mod test_data;
