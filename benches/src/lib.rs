#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated.rs"));
}
#[cfg(feature = "collab")]
mod collab;

#[cfg(feature = "collab")]
pub fn collab(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("collab");
    collab::start::benches(&mut group);
    group.finish();
}
