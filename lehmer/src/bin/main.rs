use clap::{Parser, Subcommand};

use lehmer::c_ffi::{crush_it, lehmer_next};
#[allow(unused_imports)]
use lehmer::core::{
    BoroshNiederreiter, Crawford, CrayRanf, FastU32, Fishman18, LEcuyer, Lehmer32, NaiveParkMiller,
    NaiveParkMillerOld, NaiveU32, ParkMillerEfficient, Randu, Waterman, INMOS,
};

use lehmer::find_parameters::find_lehmer_parameters;
#[allow(unused_imports)]
use lehmer::monte_carlo_pi::{check_difference, estimate_fixed_iterations};
use lehmer::test_data::generate_all;
use testu01_sys::{bbattery_BigCrush, bbattery_Crush, bbattery_SmallCrush};

// Can be overwritten with the CLI Flag
const SEED: u64 = 333;
const DIMENSION: usize = 2;
const PATH: &str = "./comparison.png";
const NUMBER_SEEDS: u32 = 10;

#[derive(Subcommand)]
enum Action {
    Generate,
    Pi,
    CompareRngPi,
    SmallCrush,
    Crush,
    BigCrush,
    Break,
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
    /// Dimension for monte carlo pi estimation
    dimension: Option<usize>,
    /// Path for comparison graph
    path: Option<String>,
    /// Number of different seeds to for the comparison
    number_seeds: Option<u32>,
}

fn main() {
    let args = Cli::parse();
    let seed = args.seed.unwrap_or(SEED);
    let path = args.path.unwrap_or(PATH.to_string());

    match args.command {
        Action::Generate => {
            let iterations = args.iterations.unwrap_or(1);
            generate_all(iterations, seed)
        }
        Action::Pi => {
            let iterations = args.iterations.unwrap_or(1) as usize;
            let dimension = args.dimension.unwrap_or(DIMENSION);
            estimate_fixed_iterations::<FastU32>(&path, dimension, seed, iterations).unwrap();
            println!("placeholder");
        }
        Action::CompareRngPi => {
            let number_seeds = args.number_seeds.unwrap_or(NUMBER_SEEDS);
            println!("{:?}", check_difference(&path, number_seeds));
        }
        Action::SmallCrush => crush_it(lehmer_next, bbattery_SmallCrush),
        Action::Crush => crush_it(lehmer_next, bbattery_Crush),
        Action::BigCrush => crush_it(lehmer_next, bbattery_BigCrush),
        Action::Break => {
            let mut rng = NaiveParkMillerOld::new(5);
            let (m, a) = find_lehmer_parameters(&mut rng);
            println!("Found m: {m} and a: {a}");
        }
    }
}
