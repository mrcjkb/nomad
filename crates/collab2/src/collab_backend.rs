use nvimx2::backend::Backend;

/// TODO: docs.
pub trait CollabBackend: Backend {}

#[cfg(feature = "neovim")]
mod neovim {
    use nvimx2::neovim::Neovim;

    use super::*;

    impl CollabBackend for Neovim {}
}
