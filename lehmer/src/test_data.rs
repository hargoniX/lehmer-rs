use rand_core::{RngCore, SeedableRng};
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use crate::core::{
    BoroshNiederreiter, Crawford, CrayRanf, FastU32, Fishman18, LEcuyer, Lemire, NaiveParkMiller,
    NaiveParkMillerOld, NaiveU32, ParkMillerEfficient, Randu, Waterman, INMOS,
};

pub fn generate_all(iterations: u64, seed: u64) {
    create_dir_all("testdata").expect("Unable to create dir");
    generate::<BoroshNiederreiter>(
        iterations,
        seed,
        format!("BoroshNiederreiter_{}.binstr", iterations),
    );
    generate::<Crawford>(iterations, seed, format!("Crawford_{}.binstr", iterations));
    generate::<CrayRanf>(iterations, seed, format!("CrayRanf_{}.binstr", iterations));
    generate::<FastU32>(iterations, seed, format!("FastU32_{}.binstr", iterations));
    generate::<Fishman18>(iterations, seed, format!("Fishman18_{}.binstr", iterations));
    generate::<LEcuyer>(iterations, seed, format!("LEcuyer_{}.binstr", iterations));
    generate::<NaiveParkMiller>(
        iterations,
        seed,
        format!("NaiveParkMiller_{}.binstr", iterations),
    );
    generate::<NaiveParkMillerOld>(
        iterations,
        seed,
        format!("NaiveParkMillerOld_{}.binstr", iterations),
    );
    generate::<NaiveU32>(iterations, seed, format!("NaiveU32_{}.binstr", iterations));
    generate::<ParkMillerEfficient>(
        iterations,
        seed,
        format!("ParkMillerEfficient_{}.binstr", iterations),
    );
    generate::<Randu>(iterations, seed, format!("Randu_{}.binstr", iterations));
    generate::<Waterman>(iterations, seed, format!("Waterman_{}.binstr", iterations));
    generate::<INMOS>(iterations, seed, format!("INMOS_{}.binstr", iterations));
    generate::<Lemire>(iterations, seed, format!("Lemire_{}.binstr", iterations));
}

pub fn generate<R: RngCore + SeedableRng>(iterations: u64, seed: u64, filepath: String) {
    let mut path = PathBuf::new();
    path.push("testdata");
    path.push(&filepath);
    let f = File::create(&path).expect("Unable to create file");
    let mut writer = BufWriter::new(f);
    let mut rng = R::seed_from_u64(seed);

    // Each loop iteration generates 2^20 bits,
    // which is the minimum bitstreamchunk length for one iteration of STS.
    for _ in 0..iterations {
        for _ in 0..u64::pow(2, 15) {
            let x = rng.next_u32();
            let binstr = format!("{x:032b}");
            writer.write(binstr.as_bytes()).unwrap();
        }
    }
    println!("Wrote to {path:?}!");
}
