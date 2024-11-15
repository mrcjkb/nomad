/// TODO: docs.
pub trait IntoModuleName {
    /// TODO: docs.
    const NAME: Option<&'static str>;
}

impl IntoModuleName for () {
    const NAME: Option<&'static str> = None;
}
