use clap::{Parser, Subcommand};

#[allow(unused_imports)]
use lehmer::core::{
    BoroshNiederreiter, Crawford, CrayRanf, FastU32, Fishman18, LEcuyer, Lehmer32, NaiveParkMiller,
    NaiveParkMillerOld, NaiveU32, ParkMillerEfficient, Randu, Waterman, INMOS,
};

#[allow(unused_imports)]
use lehmer::monte::{estimate_pi, estimate_pi_simd};
use lehmer::test_data::generate_all;

// Can be overwritten with the CLI Flag
const SEED: u64 = 333;

#[derive(Subcommand)]
enum Action {
    Generate,
    Bench,
}

/// Either generate some testdata or do some manual monte carlo
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    /// Generate Testvector or do some manual benchmarking
    command: Action,
    /// Amount of multiples of min. bitstreamchunk length 2**20
    iterations: Option<u64>,
    /// Seed for the RNG
    seed: Option<u64>,
}

fn main() {
    let args = Cli::parse();
    let seed = args.seed.unwrap_or(SEED);

    match args.command {
        Action::Generate => {
            let iterations = args.iterations.unwrap_or(1);
            generate_all(iterations, seed)
        }
        Action::Bench => println!("{}", estimate_pi::<FastU32>()),
        //println!("{}", estimate_pi_simd::<FastU32>()),
    }
}
