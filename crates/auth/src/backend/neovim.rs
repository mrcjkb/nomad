use ed::AsyncCtx;
use ed::neovim::Neovim;

use crate::AuthInfos;
use crate::backend::AuthBackend;

impl AuthBackend for Neovim {
    type LoginError = core::convert::Infallible;

    #[allow(clippy::manual_async_fn)]
    fn credential_builder(
        _: &mut ed::EditorCtx<Self>,
    ) -> impl Future<Output = Box<keyring::CredentialBuilder>> + Send + 'static
    {
        async move { keyring::builtin_credential_builder() }
    }

    async fn login(
        _: &mut AsyncCtx<'_, Self>,
    ) -> Result<AuthInfos, Self::LoginError> {
        todo!()
    }
}
