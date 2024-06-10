use rand_core::{impls::{fill_bytes_via_next, next_u64_via_u32}, RngCore, SeedableRng};

macro_rules! lehmer_impl {
    ($name:ident, $state:ty, $operating:ty) => {
        pub struct $name<const A: $state, const M: $state> {
            state: $state,
        }

        impl<const A: $state, const M: $state> $name<A, M> {
            pub fn new(state: $state) -> $name<A, M> {
                $name { state }
            }

            pub fn next(&mut self) -> $state {
                self.state = ((self.state as $operating) * (A as $operating) % (M as $operating)) as $state;
                self.state
            }
        }
    };
}

lehmer_impl!(Lehmer32, u32, u64);
lehmer_impl!(Lehmer64, u64, u128);

impl<const A: u32, const M: u32> RngCore for Lehmer32<A, M> {
    fn next_u32(&mut self) -> u32 {
        self.next()
    }

    fn next_u64(&mut self) -> u64 {
        next_u64_via_u32(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        fill_bytes_via_next(self, dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        Ok(self.fill_bytes(dest))
    }
}

impl<const A: u32, const M: u32> SeedableRng for Lehmer32<A, M> {
    type Seed = [u8; 4];

    fn from_seed(seed: Self::Seed) -> Self {
        Lehmer32::new(u32::from_le_bytes(seed))
    }
}

impl<const A: u64, const M: u64> RngCore for Lehmer64<A, M> {
    fn next_u32(&mut self) -> u32 {
        self.next() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.next()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        fill_bytes_via_next(self, dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        Ok(self.fill_bytes(dest))
    }
}

impl<const A: u64, const M: u64> SeedableRng for Lehmer64<A, M> {
    type Seed = [u8; 8];

    fn from_seed(seed: Self::Seed) -> Self {
        Lehmer64::new(u64::from_le_bytes(seed))
    }
}


macro_rules! lehmer_params {
    ($name:ident, $lehmer_name:ident, $A:expr, $M:expr, $state:ty) => {
        pub struct $name {
            rng : $lehmer_name<{$A}, {$M}>
        }

        impl $name {
            pub fn new(state: $state) -> $name {
                $name {
                    rng: $lehmer_name::new(state)
                }
            }

            pub fn next(&mut self) -> $state {
                self.rng.next()
            }
        }


    };
}

macro_rules! lehmer32_params {
    ($name:ident, $A:expr, $M:expr) => {
        lehmer_params!($name, Lehmer32, $A, $M, u32);
        impl RngCore for $name {
            fn next_u32(&mut self) -> u32 {
                self.next()
            }

            fn next_u64(&mut self) -> u64 {
                next_u64_via_u32(self)
            }

            fn fill_bytes(&mut self, dest: &mut [u8]) {
                fill_bytes_via_next(self, dest);
            }

            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
                Ok(self.fill_bytes(dest))
            }
        }

        impl SeedableRng for $name {
            type Seed = [u8; 4];

            fn from_seed(seed: Self::Seed) -> Self {
                $name::new(u32::from_le_bytes(seed))
            }
        }
    };
}

macro_rules! lehmer64_params {
    ($name:ident, $A:expr, $M:expr) => {
        lehmer_params!($name, Lehmer64, $A, $M, u64);
        impl RngCore for $name {
            fn next_u32(&mut self) -> u32 {
                self.next() as u32
            }

            fn next_u64(&mut self) -> u64 {
                self.next()
            }

            fn fill_bytes(&mut self, dest: &mut [u8]) {
                fill_bytes_via_next(self, dest);
            }

            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
                Ok(self.fill_bytes(dest))
            }
        }

        impl SeedableRng for $name {
            type Seed = [u8; 8];

            fn from_seed(seed: Self::Seed) -> Self {
                $name::new(u64::from_le_bytes(seed))
            }
        }
    };
}


// TODO: funky bithacks
lehmer32_params!(NaiveParkMillerOld, u32::pow(7, 5), u32::pow(2, 31) - 1);
lehmer32_params!(NaiveParkMiller, 48_271, u32::pow(2, 31) - 1);
lehmer32_params!(Randu, 65_539, u32::pow(2, 31));
lehmer32_params!(Crawford, 69_070, 2^32 - 5);
lehmer64_params!(CrayRanf, 44_485_709_377_909, u64::pow(2, 48));
