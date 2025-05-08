#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}
#[cfg(feature = "collab")]
mod collab;

#[cfg_attr(not(any(feature = "collab")), allow(unused_variables))]
pub fn run(criterion: &mut criterion::Criterion) {
    #[cfg(feature = "collab")]
    collab(criterion);
}

#[cfg(feature = "collab")]
fn collab(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("collab");
    collab::start::benches(&mut group);
    group.finish();
}
