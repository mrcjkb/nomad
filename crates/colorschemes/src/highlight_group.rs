use crate::Color;

/// TODO: docs
#[derive(Default)]
pub struct HighlightGroup {
    link: Option<&'static str>,
    foreground: Option<Color>,
    background: Option<Color>,
}

impl HighlightGroup {
    pub fn into_some(self) -> Option<Self> {
        Some(self)
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    pub fn with_foreground(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }
}

impl From<HighlightGroup> for common::nvim::api::opts::SetHighlightOpts {
    fn from(group: HighlightGroup) -> Self {
        let mut builder = Self::builder();

        if let Some(link) = &group.link {
            builder.link(link);
            return builder.build();
        }

        if let Some(foreground) = &group.foreground {
            builder.foreground(foreground.as_hex_string().as_str());
        }

        if let Some(background) = &group.background {
            builder.background(background.as_hex_string().as_str());
        }

        builder.build()
    }
}
