use crate::{ActionName, ModuleName, PluginName};

/// TODO: docs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Source {
    /// TODO: docs.
    pub plugin_name: &'static PluginName,

    /// TODO: docs.
    pub module_name: Option<&'static ModuleName>,

    /// TODO: docs.
    pub action_name: Option<&'static ActionName>,
}
