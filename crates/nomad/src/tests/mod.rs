//! Utilities for testing.

mod generate;
mod generator;

pub use generate::*;
pub use generator::Generator;

/// Result value for tests.
pub type TestResult = Result<(), Box<dyn std::error::Error>>;

/// Creates a random seed.
pub fn random_seed() -> u64 {
    use rand::Rng;
    rand::thread_rng().gen()
}
