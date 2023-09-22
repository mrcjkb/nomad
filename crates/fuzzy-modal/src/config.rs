use common::WindowConfig;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default)]
    pub window: WindowConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window: WindowConfig::new().x(0.3).y(0.2).width(0.4).height(0.3),
        }
    }
}
