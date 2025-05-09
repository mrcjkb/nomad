pub(crate) mod start;

pub(crate) fn run(c: &mut criterion::Criterion) {
    let mut group = c.benchmark_group("collab");
    start::run(&mut group);
    group.finish();
}
