use clap::{Parser, Subcommand};

use lehmer::c_ffi::crush_it;
#[allow(unused_imports)]
use lehmer::core::{
    BoroshNiederreiter, Crawford, CrayRanf, FastU32, Fishman18, LEcuyer, Lehmer32, Lemire,
    NaiveParkMiller, NaiveParkMillerOld, NaiveU32, ParkMillerEfficient, Randu, Waterman, INMOS,
};

#[allow(unused_imports)]
use lehmer::find_parameters::{find_lehmer_parameters_u32, find_lehmer_parameters_u64};
use lehmer::monte_carlo_pi::{check_difference, estimate_fixed_iterations};
use lehmer::test_data::generate_all;
use testu01_sys::{bbattery_BigCrush, bbattery_Crush, bbattery_SmallCrush};

// Can be overwritten with the CLI Flag
const SEED: u64 = 333;
const DIMENSION: usize = 5;
const DIMENSIONS: usize = 16;
const NUMBER_SEEDS: u32 = 50;
const PATH: &str = "./figure.png";

#[derive(Subcommand)]
enum Action {
    /// Generate testvector files for all generator variants for analysis with NIST sts
    Generate,
    /// Estimate pi via monte carlo approximation for the currently selected generator
    Pi,
    /// Compare the amount of iterations needed to estimate pi for some generators
    CompareRngPi,
    /// Execute SmallCrush on the currently selected generator
    SmallCrush,
    /// Execute Crush on the currently selected generator
    Crush,
    /// Execute BigCrush on the currently selected generator
    BigCrush,
    /// Find the parameters of the currently selected generator
    Break,
}

/// Suite around Lehmer RNGs.
/// See the subcommands for some applications, statistical tests and breakage.
/// `cargo bench` invokes criterion benchmarks for a fixed iteration count and a monte carlo approximation.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Action,
    /// Either chunk-multiples for NIST or fixed iteration count for monte carlo
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
            let iterations = args.iterations.unwrap_or(300_000) as usize;
            let dimension = args.dimension.unwrap_or(DIMENSION);
            estimate_fixed_iterations::<FastU32>(&path, dimension, seed, iterations).unwrap();
        }
        Action::CompareRngPi => {
            let number_seeds = args.number_seeds.unwrap_or(NUMBER_SEEDS);
            let dimensions = args.dimension.unwrap_or(DIMENSIONS);
            check_difference(&path, number_seeds, dimensions);
        }
        Action::SmallCrush => crush_it(bbattery_SmallCrush),
        Action::Crush => crush_it(bbattery_Crush),
        Action::BigCrush => crush_it(bbattery_BigCrush),
        Action::Break => {
            let mut rng = Crawford::new(5);
            let (m, a) = find_lehmer_parameters_u32(&mut rng);
            println!("Found m: {m} and a: {a}");
        }
    }
}
