// TODO: Think about Lehmer generators of different widths like u64
pub struct Lehmer<const A: u32, const M: u32> {
    state: u32,
}

impl<const A: u32, const M: u32> Lehmer<A, M> {
    pub fn new(state: u32) -> Lehmer<A, M> {
        Lehmer { state }
    }

    pub fn next(&mut self) -> u32 {
        self.state = ((self.state as u64) * (A as u64) % (M as u64)) as u32;
        self.state
    }
}

// TODO: use a macro for this stuff

pub struct NaiveParkMillerOld(Lehmer<{ u32::pow(7, 5) }, { u32::pow(2, 31) - 1 }>);

impl NaiveParkMillerOld {
    pub fn new(state: u32) -> NaiveParkMillerOld {
        NaiveParkMillerOld(Lehmer::new(state))
    }

    pub fn next(&mut self) -> u32 {
        self.0.next()
    }
}

pub struct NaiveParkMiller(Lehmer<48271, { u32::pow(2, 31) - 1 }>);

impl NaiveParkMiller {
    pub fn new(state: u32) -> NaiveParkMiller {
        NaiveParkMiller(Lehmer::new(state))
    }

    pub fn next(&mut self) -> u32 {
        self.0.next()
    }
}
