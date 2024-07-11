use lehmer::core::FastU32;
use lehmer::monte::estimate_pi;

fn main() {
    println!("{}", estimate_pi::<FastU32>());
}
