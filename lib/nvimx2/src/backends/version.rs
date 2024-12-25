/// TODO: docs.
pub trait NeovimVersion {}

/// TODO: docs.
pub struct ZeroDotTen;

/// TODO: docs.
pub struct Nightly;

impl NeovimVersion for ZeroDotTen {}

impl NeovimVersion for Nightly {}
