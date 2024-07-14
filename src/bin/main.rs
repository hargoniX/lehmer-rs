use lehmer::core::FastU32;
use lehmer::mc_pi::check_difference;
use lehmer::mc_pi::check_difference_print;
use lehmer::mc_pi::estimate_fixed_iterations;
use lehmer::mc_pi::estimate_pi_n;

fn main() {
    // check_difference();
    // check_difference();
    // println!(
    //     "pi: {:?}",
    //     estimate_fixed_iterations::<FastU32>(10, 43, 500_000_000)
    // );
    println!("pi: {:?}", estimate_pi_n::<FastU32>(10, 43));
}
