mod config;
mod mad;
mod runtime;

use common::nvim;
use mad::Mad;

#[nvim::module]
fn mad() -> nvim::Result<nvim::Dictionary> {
    Ok(Mad::new()
        // .with_plugin::<completion::Completion>()
        // .with_plugin::<lsp::Lsp>()
        .with_plugin::<seph::Seph>()
        .api())
}
