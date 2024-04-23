//! Utilities for testing.

mod emoji;
mod generate;
mod generator;
mod letter;
mod text;

pub use emoji::Emoji;
pub use generate::*;
pub use generator::Generator;
pub use letter::Letter;
pub use text::Text;

/// Error value for tests.
pub type TestError = Box<dyn std::error::Error>;

/// Result value for tests.
pub type TestResult = Result<(), TestError>;

/// Creates a random seed.
pub fn random_seed() -> u64 {
    use rand::Rng;
    rand::thread_rng().gen()
}
