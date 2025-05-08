#![allow(missing_docs)]

criterion::criterion_group!(benches, benches::run);
criterion::criterion_main!(benches);
