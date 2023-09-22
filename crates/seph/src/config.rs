use common::WindowConfig;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub(crate) window: WindowConfig,
}
