//! TODO: docs

#![cfg_attr(docsrs, feature(doc_cfg))]

pub use nvim_oxi as oxi;
#[cfg(feature = "executor")]
#[cfg_attr(docsrs, doc(cfg(feature = "executor")))]
pub use nvimx_executor as executor;
#[cfg(feature = "project")]
#[cfg_attr(docsrs, doc(cfg(feature = "project")))]
pub use nvimx_project as project;
#[cfg(feature = "ui")]
#[cfg_attr(docsrs, doc(cfg(feature = "ui")))]
pub use nvimx_ui as ui;
