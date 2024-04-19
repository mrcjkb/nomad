//! Utilities for testing.

use core::ops::Range;

use rand::distributions::{DistString, Standard};
use rand::Rng;
use rand_chacha::ChaChaRng;
use rand_distr::{Distribution, Normal};

use crate::{ByteOffset, Replacement};

/// A generator of random values.
pub struct Generator {
    rng: ChaChaRng,
}

impl Generator {
    /// Generates a new value.
    pub fn generate<Ctx, T: Generate<Ctx>>(&mut self, ctx: Ctx) -> T {
        T::generate(self, ctx)
    }

    /// Returns a RNG.
    pub fn rng(&mut self) -> &mut impl Rng {
        &mut self.rng
    }
}

/// A trait for values that can be generated randomly.
pub trait Generate<Ctx> {
    /// Generates a random value from the context using the generator.
    fn generate(generator: &mut Generator, ctx: Ctx) -> Self;
}

impl<D: Distribution<f64>> Generate<D> for usize {
    fn generate(gen: &mut Generator, distr: D) -> Self {
        distr.sample(gen.rng()) as Self
    }
}

/// A context for generating values with an exact length.
#[derive(Debug, Clone, Copy)]
pub struct ExactLen(pub usize);

/// A context for generating values with an average length.
#[derive(Debug, Clone, Copy)]
pub struct MeanLen(pub usize);

impl Generate<MeanLen> for ExactLen {
    fn generate(gen: &mut Generator, len: MeanLen) -> Self {
        let mean = len.0 as f64;
        let std_dev = mean * 0.3;
        let distr = Normal::new(mean, std_dev).expect("valid inputs");
        let len = distr.sample(gen.rng()) as usize;
        ExactLen(len)
    }
}

impl Generate<ExactLen> for String {
    fn generate(gen: &mut Generator, len: ExactLen) -> Self {
        Standard.sample_string(gen.rng(), len.0)
    }
}

impl Generate<MeanLen> for String {
    fn generate(gen: &mut Generator, len: MeanLen) -> Self {
        let exact_len: ExactLen = gen.generate(len);
        gen.generate(exact_len)
    }
}

impl Generate<&str> for ByteOffset {
    fn generate(gen: &mut Generator, s: &str) -> Self {
        let offset: usize = gen.rng().gen_range(0..=s.len());
        let offset = ceil_char_boundary(s, offset);
        Self::new(offset)
    }
}

/// A context for generating byte ranges.
pub struct ByteRangeCtx<'a, Len> {
    /// The string the byte range is relative to.
    pub string: &'a str,

    /// The length of the range.
    pub len: Len,
}

impl Generate<ByteRangeCtx<'_, ExactLen>> for Range<ByteOffset> {
    fn generate(gen: &mut Generator, ctx: ByteRangeCtx<ExactLen>) -> Self {
        let start: ByteOffset = gen.generate(ctx.string);
        let end = (usize::from(start) + ctx.len.0).min(ctx.string.len());
        let end = ByteOffset::new(ceil_char_boundary(ctx.string, end));
        start..end
    }
}

impl Generate<ByteRangeCtx<'_, MeanLen>> for Range<ByteOffset> {
    fn generate(gen: &mut Generator, ctx: ByteRangeCtx<MeanLen>) -> Self {
        let exact_len: ExactLen = gen.generate(ctx.len);
        gen.generate(ByteRangeCtx { len: exact_len, string: ctx.string })
    }
}

/// A context for generating edits.
pub struct ReplacementCtx<'a, RangeLen, TextLen> {
    /// The string the edit acts on.
    pub string: &'a str,

    /// The length of the deleted range.
    pub range_len: RangeLen,

    /// The length of the replacement text.
    pub text_len: TextLen,
}

impl<'a, RangeLen, TextLen> ReplacementCtx<'a, RangeLen, TextLen> {
    /// Creates a new [`EditCtx`].
    pub fn new(
        string: &'a str,
        range_len: RangeLen,
        text_len: TextLen,
    ) -> Self {
        Self { string, range_len, text_len }
    }
}

impl<'a, RangeLen, TextLen> Generate<ReplacementCtx<'a, RangeLen, TextLen>>
    for Replacement<ByteOffset>
where
    Range<ByteOffset>: Generate<ByteRangeCtx<'a, RangeLen>>,
    String: Generate<TextLen>,
{
    fn generate(
        gen: &mut Generator,
        ctx: ReplacementCtx<'a, RangeLen, TextLen>,
    ) -> Self {
        let ReplacementCtx { string, range_len, text_len } = ctx;

        let range_ctx = ByteRangeCtx { string, len: range_len };
        let range: Range<ByteOffset> = gen.generate(range_ctx);
        let replacement: String = gen.generate(text_len);

        Replacement::new(range, replacement)
    }
}

/// Result value for tests.
pub type TestResult = Result<(), Box<dyn std::error::Error>>;

#[inline]
fn ceil_char_boundary(s: &str, mut offset: usize) -> usize {
    loop {
        if s.is_char_boundary(offset) {
            return offset;
        }
        offset += 1;
    }
}
