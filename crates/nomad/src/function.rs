use nvim_oxi::{Function as NvimFunction, Object as NvimObject};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use crate::diagnostics::{DiagnosticSource, Level};
use crate::maybe_result::MaybeResult;
use crate::{Action, Module};

/// TODO: docs.
pub trait Function: Action<Args: DeserializeOwned, Return: Serialize> {
    fn into_function(self) -> NvimFunction<NvimObject, NvimObject>;
}

impl<T> Function for T
where
    T: Action<Args: DeserializeOwned, Return: Serialize>,
{
    fn into_function(self) -> NvimFunction<NvimObject, NvimObject> {
        NvimFunction::from_fn(move |args| {
            let args = match crate::serde::deserialize(args) {
                Ok(args) => args,
                Err(err) => {
                    let mut source = DiagnosticSource::new();
                    source
                        .push_segment(T::Module::NAME.as_str())
                        .push_segment(T::NAME.as_str());
                    err.into_msg().emit(Level::Warning, source);
                    return NvimObject::nil();
                },
            };
            let ret = match self.execute(args).into_result() {
                Ok(ret) => ret,
                Err(err) => {
                    let mut source = DiagnosticSource::new();
                    source
                        .push_segment(T::Module::NAME.as_str())
                        .push_segment(T::NAME.as_str());
                    err.into().emit(Level::Warning, source);
                    return NvimObject::nil();
                },
            };
            crate::serde::serialize(&ret)
        })
    }
}
