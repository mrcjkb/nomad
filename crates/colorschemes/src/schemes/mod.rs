mod ayu_mirage;

use ayu_mirage::AyuMirage;
pub(crate) use colorschemes::colorschemes;

mod colorschemes {
    use super::*;

    pub(crate) fn colorschemes() -> Vec<Box<dyn crate::LoadableColorscheme>> {
        vec![Box::<AyuMirage>::default()]
    }
}
