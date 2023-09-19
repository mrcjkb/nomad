use common::nvim;

use crate::{Colorscheme, HighlightGroup};

/// TODO: docs
pub trait LoadableColorscheme: Send + Sync {
    /// TODO: docs
    fn name(&self) -> &'static str;

    /// TODO: docs
    fn api_name(&self) -> &'static str;

    /// TODO: docs
    fn load(&self) -> Result<(), Box<dyn std::error::Error>>;
}

impl<C> LoadableColorscheme for C
where
    C: Send + Sync + Colorscheme,
{
    fn name(&self) -> &'static str {
        C::NAME
    }

    fn api_name(&self) -> &'static str {
        C::NAME
            .chars()
            .map(|c| if c == ' ' { '_' } else { c })
            .filter(|c| c.is_ascii_alphanumeric() || *c == '_')
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>()
            .leak()
    }

    fn load(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(normal) = self.normal() {
            set_hl("Normal", normal)?;
        }
        if let Some(color_column) = self.color_column() {
            set_hl("ColorColumn", color_column)?;
        }
        Ok(())
    }
}

fn set_hl(hl_name: &str, hl_group: HighlightGroup) -> nvim::Result<()> {
    nvim::api::set_hl(0, hl_name, &hl_group.into()).map_err(Into::into)
}
