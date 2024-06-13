use lehmer::core::{Crawford, FastU32, NaiveU32};
use lehmer::monte::estimate_pi;

fn main() {
    println!("{}", estimate_pi::<Crawford>());
}
