#![allow(missing_docs)]

fn benches() {
    #[cfg(any(feature = "collab"))]
    let mut criterion = criterion::Criterion::default().configure_from_args();

    #[cfg(feature = "collab")]
    benches::collab(&mut criterion);
}

criterion::criterion_main!(benches);
