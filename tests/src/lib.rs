#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

#[cfg(all(test, feature = "collab"))]
mod collab;
#[cfg(test)]
mod ed;
#[cfg(test)]
mod mock;
#[cfg(all(test, feature = "neovim"))]
mod neovim;
#[cfg(all(test, feature = "walkdir"))]
mod walkdir;
