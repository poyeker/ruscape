use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ruscape::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    let w = World::init(10000, 10, 10, Corner, true);
    let patches = w.borrow().patches();
    let mut turtles = w.borrow().turtles();

    c.bench_function("fib 20", |b| {
        b.iter(|| {
            turtles.ask(|t| {
                t.fd(3.);
            });
            patches.report(|p| p.turtles_on());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
