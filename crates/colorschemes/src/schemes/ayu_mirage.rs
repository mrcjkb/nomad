use crate::*;

#[derive(Default)]
pub(crate) struct AyuMirage;

impl colorscheme::Palette for AyuMirage {
    const PALETTE: palette::Palette = palette::Palette {
        foreground: hex!("#252935"),
        background: hex!("#cccac3"),
    };
}

impl Colorscheme for AyuMirage {
    const NAME: &'static str = "Ayu Mirage";
}

impl BaseColorscheme for AyuMirage {}

impl DiagnosticColorscheme for AyuMirage {}

impl LspColorscheme for AyuMirage {}

impl TreeSitterColorscheme for AyuMirage {}

impl NomadColorscheme for AyuMirage {}

impl TelescopeColorscheme for AyuMirage {}
