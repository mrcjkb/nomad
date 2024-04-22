use core::num::NonZeroUsize;

use rand::distributions::{DistString, Distribution, WeightedIndex};
use rand::Rng;

use super::{Emoji, Letter};

/// Average english word length according to [this paper][sauce].
///
/// [sauce]: https://math.wvu.edu/~hdiamond/Math222F17/Sigurd_et_al-2004-Studia_Linguistica.pdf
const AVG_WORD_LEN: f64 = 4.60;

const AVG_LINE_LEN: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(50) };

const AVG_EMOJIS_PER_LINE: usize = 3;

/// A [`Distribution`] that generates text with spaces, newlines, emojis, and
/// random letters with the same relative probabilities as natural english
/// text,
pub struct Text;

impl Distribution<char> for Text {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        let avg_line_len = AVG_LINE_LEN.get() as f64;

        let prob_newline = 1.0 / avg_line_len;
        let prob_space = 1.0 / AVG_WORD_LEN;
        let prob_emoji = AVG_EMOJIS_PER_LINE as f64 / avg_line_len;
        let prob_letter = 1.0 - prob_newline - prob_space - prob_emoji;

        let weights = [prob_newline, prob_space, prob_emoji, prob_letter];

        let weighted = WeightedIndex::new(weights).expect("weights are valid");

        match weighted.sample(rng) {
            0 => '\n',
            1 => ' ',
            2 => Emoji.sample(rng),
            3 => Letter.sample(rng),
            _ => unreachable!(),
        }
    }
}

impl DistString for Text {
    #[inline]
    fn append_string<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        string: &mut String,
        len: usize,
    ) {
        for _ in 0..len {
            string.push(self.sample(rng));
        }
    }
}
