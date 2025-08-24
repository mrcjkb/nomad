use core::any;

use crate::Editor;
use crate::context::{self, Context, StateHandle};
use crate::module::{self, Module, PanicInfo};

pub(crate) const NO_COMMAND_NAME: &str = "�";

/// TODO: docs.
pub trait Plugin<Ed: Editor>: Module<Ed> {
    /// TODO: docs.
    const COMMAND_NAME: &str = NO_COMMAND_NAME;

    /// TODO: docs.
    const CONFIG_FN_NAME: &str = "setup";

    /// TODO: docs.
    fn handle_panic(
        &self,
        panic_info: PanicInfo,
        ctx: &mut Context<Ed, context::Borrowed<'_>>,
    );

    #[doc(hidden)]
    #[track_caller]
    fn api(self, editor: Ed) -> Ed::Api {
        StateHandle::new(editor).with_mut(|s| module::build_api(self, s))
    }

    #[inline]
    #[doc(hidden)]
    #[allow(private_interfaces)]
    fn id() -> PluginId {
        PluginId { type_id: any::TypeId::of::<Self>() }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct PluginId {
    pub(crate) type_id: any::TypeId,
}
