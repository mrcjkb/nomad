use nvimx2::backend::{Backend, BufferId};
use nvimx2::fs::AbsPathBuf;
use nvimx2::{AsyncCtx, notify};

/// TODO: docs.
pub trait CollabBackend: Backend {
    /// TODO: docs.
    type FindProjectRootError: notify::Error;

    /// TODO: docs.
    fn project_root(
        buffer_id: BufferId<Self>,
        ctx: &mut AsyncCtx<'_, Self>,
    ) -> impl Future<Output = Result<AbsPathBuf, Self::FindProjectRootError>>;
}

#[cfg(feature = "neovim")]
mod neovim {
    use nvimx2::neovim::{Neovim, NeovimBuffer, oxi};
    use oxi::mlua::{Function, Table};

    use super::*;

    impl CollabBackend for Neovim {
        type FindProjectRootError = core::convert::Infallible;

        async fn project_root(
            buffer: NeovimBuffer,
            _ctx: &mut AsyncCtx<'_, Self>,
        ) -> Result<AbsPathBuf, Self::FindProjectRootError> {
            let _root = lsp_rootdir(buffer);
            todo!()
        }
    }

    /// Returns the root directory of the first language server attached to the
    /// given buffer, if any.
    fn lsp_rootdir(buffer: NeovimBuffer) -> Option<String> {
        let lua = oxi::mlua::lua();

        let get_clients = lua
            .globals()
            .get::<Table>("vim")
            .ok()?
            .get::<Table>("lsp")
            .ok()?
            .get::<Function>("get_clients")
            .ok()?;

        let opts = lua.create_table().ok()?;
        opts.set("bufnr", buffer).ok()?;

        get_clients
            .call::<Table>(opts)
            .ok()?
            .get::<Table>(1)
            .ok()?
            .get::<Table>("config")
            .ok()?
            .get::<String>("root_dir")
            .ok()
    }
}
