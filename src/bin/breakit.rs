use lehmer::breakit::{breakit, make_float};
use lehmer::core::FastU32;

fn main() {
    let max = 0xfffffffb;
    let mut rand = FastU32::new(6858475);
    breakit::<1>(&mut rand, max);
    println!("Real next: {}", make_float(&mut rand, max))
}
