use lehmer::core::NaiveParkMillerOld;
use lehmer::find_parameters::find_lehmer_parameters;

fn main() {
    let mut rng = NaiveParkMillerOld::new(5);
    find_lehmer_parameters(&mut rng);
}
