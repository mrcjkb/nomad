use rand::{Rng, SeedableRng};
use rand_chacha::ChaChaRng;

use super::Generate;

/// A generator of random values.
pub struct Generator {
    rng: ChaChaRng,
}

impl Generator {
    /// Generates a new value.
    pub fn generate<Ctx, T: Generate<Ctx>>(&mut self, ctx: Ctx) -> T {
        T::generate(self, ctx)
    }

    #[doc(hidden)]
    pub fn new(seed: u64) -> Self {
        Self { rng: ChaChaRng::seed_from_u64(seed) }
    }

    /// Returns a RNG.
    pub fn rng(&mut self) -> &mut impl Rng {
        &mut self.rng
    }
}
