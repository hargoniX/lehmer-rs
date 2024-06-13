use lehmer::core::{Crawford, FastU32, NaiveU32};
use lehmer::monte::{estimate_pi, estimate_pi_slow};

fn main() {
    //println!("{}", estimate_pi_slow::<FastU32>());
    println!("{}", estimate_pi::<FastU32>());
}
