use crate::Agent::{Agent, AgentRef};

use crate::Patch::Patch;
use crate::PatchSet::PatchSet;

use crate::TurtleSet::TurtleSet;
use crate::World::World;

use macroquad::Color;
use std::cell::RefCell;
use std::rc::Rc;

pub type PatchRef = AgentRef<Patch>;

impl PatchRef {
    #[inline]
    pub fn who(&self) -> (i64, i64) {
        self.borrow().who()
    }
    #[inline]
    pub fn pcolor(&self) -> Color {
        self.borrow().pcolor()
    }
    #[inline]
    pub fn pxcor(&self) -> i64 {
        self.borrow().pxcor()
    }
    #[inline]
    pub fn pycor(&self) -> i64 {
        self.borrow().pycor()
    }
}

impl PatchRef {
    #[inline]
    fn world(&self) -> Rc<RefCell<World>> {
        self.borrow().world()
    }
}

impl PatchRef {
    #[inline]
    pub fn set_pcolor(&self, color: Color) {
        self.borrow_mut().set_pcolor(color)
    }
    pub fn set_random_pcolor(&self) {
        let color = Color::new(
            self.borrow().world().borrow().rng().f32(),
            self.borrow().world().borrow().rng().f32(),
            self.borrow().world().borrow().rng().f32(),
            self.borrow().world().borrow().rng().f32(),
        );
        self.borrow_mut().set_pcolor(color);
    }
}

impl PatchRef {
    #[inline]
    pub fn other(&self, patches: &PatchSet) -> PatchSet {
        self.borrow().other(patches)
    }
    #[inline]
    pub fn neighbors(&self) -> PatchSet {
        self.borrow().neighbors.clone()
    }
    #[inline]
    pub fn neighborhood(&self) -> PatchSet {
        self.borrow().neighborhood.clone()
    }
    #[inline]
    pub fn neighborhood4(&self) -> PatchSet {
        self.borrow().neighborhood4.clone()
    }
    #[inline]
    pub fn neighbors4(&self) -> PatchSet {
        self.borrow().neighbors4.clone()
    }
    #[inline]
    pub fn turtles_on(&self) -> TurtleSet {
        self.borrow().turtles_on()
    }
}

impl PatchRef {
    #[inline]
    pub fn sprout(&self, amount: usize) -> TurtleSet {
        let new_turtles = self.world().borrow_mut().crt(amount);
        new_turtles.values().for_each(|t| {
            t.setxy(self.pxcor() as f64, self.pycor() as f64);
        });
        new_turtles
    }
}
