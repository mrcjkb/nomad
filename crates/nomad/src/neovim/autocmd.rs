use core::fmt;

use nvim_oxi::api::{self, opts, types};

use crate::action::{Action, ActionName};
use crate::diagnostics::{DiagnosticSource, Level};
use crate::maybe_result::MaybeResult;
use crate::module::Module;
use crate::ModuleName;

/// TODO: docs.
pub trait Autocmd<M: Module>: Sized {
    /// TODO: docs.
    type Action: Action<
        Module = M,
        Args: From<types::AutocmdCallbackArgs>,
        Docs: fmt::Display,
        Return: Into<ShouldDetach>,
    >;

    /// TODO: docs.
    fn into_action(self) -> Self::Action;

    /// TODO: docs.
    fn on_events(&self) -> impl IntoIterator<Item = &str>;

    /// TODO: docs.
    fn register(self) -> AutocmdId {
        let module_name = M::NAME;
        let action_name = Self::Action::NAME;

        let augroup_name =
            AugroupName { module_name, action_name }.to_string();

        let opts = opts::CreateAugroupOpts::builder().clear(true).build();
        let augroup_id = api::create_augroup(&augroup_name, &opts)
            .expect("all arguments are valid");

        let events = self
            .on_events()
            .into_iter()
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();

        let mut action = self.into_action();
        let opts = opts::CreateAutocmdOpts::builder()
            .group(augroup_id)
            .desc(action.docs().to_string())
            .callback(nvim_oxi::Function::from_fn_mut(
                move |args: types::AutocmdCallbackArgs| match action
                    .execute(args.into())
                    .into_result()
                    .map(Into::into)
                    .map_err(Into::into)
                {
                    Ok(ShouldDetach::Yes) => true,
                    Ok(ShouldDetach::No) => false,
                    Err(diagnostic_msg) => {
                        let mut source = DiagnosticSource::new();
                        source
                            .push_segment(module_name.as_str())
                            .push_segment(action_name.as_str());
                        diagnostic_msg.emit(Level::Error, source);
                        false
                    },
                },
            ))
            .build();
        let events = &events;
        api::create_autocmd(events.iter().map(AsRef::as_ref), &opts)
            .expect("all arguments are valid")
            .into()
    }
}

/// TODO: docs.
pub enum ShouldDetach {
    /// TODO: docs.
    Yes,
    /// TODO: docs.
    No,
}

/// TODO: docs.
#[derive(Debug, Clone, Copy)]
pub struct AutocmdId(u32);

struct AugroupName {
    module_name: ModuleName,
    action_name: ActionName,
}

impl From<()> for ShouldDetach {
    fn from(_: ()) -> Self {
        Self::No
    }
}

impl From<bool> for ShouldDetach {
    fn from(b: bool) -> Self {
        if b {
            Self::Yes
        } else {
            Self::No
        }
    }
}

impl From<u32> for AutocmdId {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

impl fmt::Display for AugroupName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nomad-{}/{}", self.module_name, self.action_name)
    }
}
