use common::{nvim, Rectangle};
use nvim::api::types::WindowBorder;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub window: Rectangle,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window: Rectangle::default()
                .at_x(0.325)
                .at_y(0.15)
                .with_width(0.35)
                .with_height(0.45),
        }
    }
}
