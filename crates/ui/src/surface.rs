use api::types::*;
use nvim::api;

pub(crate) struct Surface {
    /// TODO: docs.
    buffer: api::Buffer,

    /// TODO: docs.
    window: api::Window,
}

impl Surface {
    #[inline]
    pub(crate) fn is_hidden(&self) -> bool {
        self.window
            .get_config()
            .map(|config| config.hide.unwrap_or(false))
            .unwrap_or(false)
    }

    /// TODO: docs
    #[inline]
    pub(crate) fn new_hidden() -> Self {
        let buffer = api::create_buf(false, true).expect("never fails(?)");

        let config = WindowConfig::builder()
            .relative(WindowRelativeTo::Editor)
            .height(1)
            .width(1)
            .row(0)
            .col(0)
            .hide(true)
            .style(WindowStyle::Minimal)
            .build();

        let window = api::open_win(&buffer, false, &config)
            .expect("the config is valid");

        Self { buffer, window }
    }
}
