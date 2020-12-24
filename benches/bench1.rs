#![feature(test)]
#[cfg(test)]

mod tests {
    use ruscape::prelude::*;

    extern crate test;
    #[bench]
    fn bench1(b: &mut test::Bencher) {
        let w = World::init(10000, 10, 10, Corner, true);
        let patches = w.borrow().patches();
        let n = w.borrow().patches().one_of().neighbors();
        let turtles = w.borrow().turtles();
        b.iter(|| {
            turtles.ask(|t| {
                t.fd(3.);
            });
            patches.report(|p| p.turtles_on());
        });
    }

    #[bench]
    fn bench2(b: &mut test::Bencher) {
        let w = World::init(10000, 10, 10, Corner, true);
        let turtles = w.borrow().turtles();

        b.iter(|| {
            turtles.ask(|t| {
                t.random_headings().fd(t.rng().f64() * 6.);
            })
        });
    }

    #[bench]
    fn bench3(b: &mut test::Bencher) {
        let w = World::init(10000, 10, 10, Corner, true);
        let mut turtles = w.borrow().turtles();
        turtles.own(vec!["strategy"]);
        b.iter(|| {
            for _ in 0..100 {
                turtles.report(|t| "dd");
            }
        });
    }

    #[bench]
    fn bench4(b: &mut test::Bencher) {
        let w = World::init(1000, 10, 10, Corner, true);

        b.iter(|| {
            for _ in 0..1000 {
                let turtles = w.borrow().turtles();
                turtles.n_of(10).ask(|t| t.die());
                let patch = w.borrow().patch(0., 0.);
                w.borrow_mut().crt(20);
            }
        });
    }
}
