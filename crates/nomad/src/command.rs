use crate::command_args::CommandArgs;
use crate::diagnostics::DiagnosticMessage;
use crate::Action;

/// TODO: docs.
pub trait Command:
    Action<
    Args: Clone
              + for<'a> TryFrom<
        &'a mut CommandArgs,
        Error: Into<DiagnosticMessage>,
    >,
    Return = (),
>
{
}

impl<T> Command for T where
    T: Action<
        Args: Clone
                  + for<'a> TryFrom<
            &'a mut CommandArgs,
            Error: Into<DiagnosticMessage>,
        >,
        Return = (),
    >
{
}
