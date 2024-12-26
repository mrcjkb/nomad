//! TODO: docs

#![cfg_attr(docsrs, feature(doc_cfg))]

pub use nvim_oxi::print;
#[doc(inline)]
pub use nvimx_common::*;
#[doc(inline)]
pub use nvimx_ctx as ctx;
#[doc(inline)]
pub use nvimx_diagnostics as diagnostics;
#[doc(inline)]
pub use nvimx_emit as emit;
#[doc(inline)]
pub use nvimx_event as event;
#[doc(inline)]
pub use nvimx_executor as executor;
#[doc(inline)]
pub use nvimx_fs as fs;
#[doc(inline)]
pub use nvimx_macros as macros;
#[doc(inline)]
pub use nvimx_macros::test;
#[doc(inline)]
pub use nvimx_plugin as plugin;
#[doc(inline)]
pub use nvimx_ui as ui;
