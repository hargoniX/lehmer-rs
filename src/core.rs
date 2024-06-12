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

macro_rules! lehmer32_traits {
    ($name:ident) => {
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

macro_rules! lehmer32_params {
    ($name:ident, $A:expr, $M:expr) => {
        lehmer_params!($name, Lehmer32, $A, $M, u32);
        lehmer32_traits!($name);
    };
}

macro_rules! lehmer64_traits {
    ($name:ident) => {
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

macro_rules! lehmer64_params {
    ($name:ident, $A:expr, $M:expr) => {
        lehmer_params!($name, Lehmer64, $A, $M, u64);
        lehmer64_traits!($name);
    };
}

pub struct ParkMillerEfficient {
    state: u32
}

impl ParkMillerEfficient {
    pub fn new(state: u32) -> Self {
        Self { state }
    }

    pub fn next(&mut self) -> u32 {
        let product = (self.state as u64) * 48271;
        let x = ((product & 0x7fffffff) + (product >> 31)) as u32;
        self.state = (x & 0x7fffffff) + (x >> 31);
        self.state
    }
}


// TODO: Wikipedia recommends this set, check where it comes from to give it a proper name.
pub struct FastU32 {
    state: u32
}

impl FastU32 {
    pub fn new(state: u32) -> Self {
        Self { state }
    }

    pub fn next(&mut self) -> u32 {
        let mut product = (self.state as u64) * 279470273;
        product = (product & 0xffffffff) + ((5 * ((product >> 32) as u32)) as u64);
        product = product + 4;
        let x = (product as u32) + 5 * ((product >> 32) as u32);
        self.state = x - 4;
        self.state
    }
}


lehmer32_params!(NaiveParkMillerOld, u32::pow(7, 5), u32::pow(2, 31) - 1);
// The same as Fishman 20: see Knuths Seminumerical Algorithms, 3rd Ed., pages 106-108
lehmer32_params!(NaiveParkMiller, 48_271, u32::pow(2, 31) - 1);
lehmer32_params!(Fishman18, 62_089_911, u32::pow(2, 31) - 1);
lehmer32_params!(LEcuyer, 40692, u32::pow(2, 31) - 249);
lehmer32_params!(Randu, 65_539, u32::pow(2, 31));
lehmer32_params!(Crawford, 69_070, 2^32 - 5);
lehmer64_params!(CrayRanf, 44_485_709_377_909, u64::pow(2, 48));
lehmer64_params!(BoroshNiederreiter, 1_812_433_253, u64::pow(2, 32));
lehmer64_params!(INMOS, 1_664_525, u64::pow(2, 32));
lehmer64_params!(Waterman, 1_566_083_941, u64::pow(2, 32));
lehmer32_traits!(ParkMillerEfficient);
lehmer32_params!(NaiveU32, 279_470_273, 0xfffffffb);
lehmer32_traits!(FastU32);

#[cfg(test)]
mod tests {
  use crate::core::FastU32;
  use crate::core::NaiveU32;
  use crate::core::ParkMillerEfficient;
  use crate::core::NaiveParkMiller;

  /*
   * We have a formal proof by bit blasting that this works but let's add a quickcheck test to make
   * sure nobody ends up changing the implementation such that it breaks.
   */
  quickcheck! {
      fn optimized_park_miller_correct(seed: u32) -> bool {
          let mut r1 = NaiveParkMiller::new(seed);
          let mut r2 = ParkMillerEfficient::new(seed);
          r1.next() == r2.next()
      }

      fn optimized_u32(seed : u32) -> bool {
          let mut r1 = NaiveU32::new(seed);
          let mut r2 = FastU32::new(seed);
          r1.next() == r2.next()
      }
  }
}
