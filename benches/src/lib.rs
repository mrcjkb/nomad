#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

#[cfg(feature = "collab")]
mod collab;
mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}

#[cfg_attr(not(any(feature = "collab")), allow(unused_variables))]
pub fn run(criterion: &mut criterion::Criterion) {
    #[cfg(feature = "collab")]
    collab::run(criterion);
}
