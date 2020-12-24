#![feature(test)]
use itertools::Itertools;
#[cfg(test)]
use ruscape::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
extern crate test;
#[test]
fn run() {
    Model::new().run(10);
}

#[bench]
fn bench2(b: &mut test::Bencher) {
    let mut m = Model::new();
    m.setup();
    b.iter(|| m.go())
}

struct Model {
    _w: Rc<RefCell<World>>,
    patches: PatchSet,
}

impl Model {
    fn new() -> Model {
        let w = World::init(0, 100, 100, Corner, true);
        let patches = w.borrow().patches();
        Model { _w: w, patches }
    }

    pub fn setup(&mut self) {
        self.patches.own(vec!["type", "payoff"]);
        self.patches.ask(|p| {
            p.set("type", if fastrand::f64() <= 0.5 { "c" } else { "d" });
            p.set("payoff", 0);
        });
    }

    pub fn go(&mut self) {
        self.patches.ask(|p| {
            p.borrow().neighborhood4().report(|neighbor| {
                neighbor
                    .borrow()
                    .neighborhood4()
                    .values()
                    .filter(|p| p.get("type") == "c")
                    .count()
            });
        })
    }
    pub fn run(&mut self, steps: usize) {
        self.setup();
        for _ in 0..steps {
            self.go();
        }
    }
}
