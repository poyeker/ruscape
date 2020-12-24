use crate::Agent::{Agent, AgentRef};

use crate::LinkSet::LinkSet;

use crate::AgentSet::AgentSet;
use crate::Position::Position;
use crate::Turtle::Turtle;
use crate::TurtleSet::TurtleSet;
use crate::World::World;

use crate::PatchRef::PatchRef;
use itertools::Itertools;

use macroquad::Color;

use crate::common::random_float;

use fastrand::Rng;

use crate::PatchSet::PatchSet;
use petgraph::graph::NodeIndex;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) type TurtleRef = AgentRef<Turtle>;

impl TurtleRef {
    #[inline]
    pub fn who(&self) -> usize {
        self.borrow().who()
    }
    #[inline]
    pub fn xcor(&self) -> f64 {
        self.borrow().xcor()
    }
    #[inline]
    pub fn ycor(&self) -> f64 {
        self.borrow().ycor()
    }
    #[inline]
    pub fn shape(&self) -> String {
        self.borrow().shape().into()
    }
    #[inline]
    pub fn heading(&self) -> f64 {
        self.borrow().heading()
    }
    #[inline]
    pub fn size(&self) -> f64 {
        self.borrow().size()
    }
    #[inline]
    pub fn color(&self) -> Color {
        self.borrow().color()
    }
    #[inline]
    pub(crate) fn world(&self) -> Rc<RefCell<World>> {
        self.borrow().world()
    }
    #[inline]
    pub fn patch_here(&self) -> PatchRef {
        self.borrow().patch_here()
    }
    #[inline]
    pub fn turtles_here(&self) -> TurtleSet {
        self.borrow().patch_here().turtles_on()
    }
}

impl TurtleRef {
    #[inline]
    pub fn distance(&self, other: &TurtleRef) -> f64 {
        self.borrow().distance(&*other.borrow())
    }
    #[inline]
    pub fn distancexy(&self, x: f64, y: f64) -> f64 {
        self.borrow().distancexy(x, y)
    }
}

impl TurtleRef {
    #[inline]
    pub fn random_xcor(&self) -> f64 {
        random_float(
            &mut Rng::new(),
            self.borrow().x_min(),
            self.borrow().x_max(),
        )
    }
    #[inline]
    pub fn random_ycor(&self) -> f64 {
        random_float(
            &mut Rng::new(),
            self.borrow().y_min(),
            self.borrow().y_max(),
        )
    }
    #[inline]
    pub fn random_pxcor(&self) -> i64 {
        self.rng()
            .i64(self.borrow().x_min().ceil() as i64..self.borrow().x_max().ceil() as i64)
    }
    #[inline]
    pub fn random_pycor(&self) -> i64 {
        self.rng()
            .i64(self.borrow().y_min().ceil() as i64..self.borrow().y_max().ceil() as i64)
    }
}

impl TurtleRef {
    #[inline]
    pub fn setxy(&self, x: f64, y: f64) -> &TurtleRef {
        let current_patch = self.patch_here();
        self.borrow_mut().setxy(x, y);
        let next_patch = self.patch_here();
        if current_patch != next_patch {
            current_patch.borrow_mut().remove_turtle_on(self);
            next_patch.borrow_mut().add_turtle_on(self)
        }
        self
    }
    #[inline]
    pub fn fd(&self, distance: f64) -> &TurtleRef {
        self.setxy(
            self.xcor() + distance * (90. - self.heading()).to_radians().cos(),
            self.ycor() + distance * (90. - self.heading()).to_radians().sin(),
        )
    }
    #[inline]
    pub fn move_to<A: Position + Agent>(&self, agent: &AgentRef<A>) -> &TurtleRef {
        let (x, y) = agent.borrow().position();
        self.setxy(x, y)
    }

    pub fn home(&self) -> &TurtleRef {
        self.setxy(0.0, 0.0)
    }
    #[inline]
    pub fn left(&self, degree: f64) -> &TurtleRef {
        self.borrow_mut().left(degree);
        self
    }
    #[inline]
    pub fn right(&self, degree: f64) -> &TurtleRef {
        self.borrow_mut().right(degree);
        self
    }
    #[inline]
    pub fn towardsxy(&self, x: f64, y: f64) -> f64 {
        self.borrow().towardsxy(x, y)
    }
    #[inline]
    pub fn towards(&self, other: &TurtleRef) -> f64 {
        self.borrow().towardsxy(other.xcor(), other.ycor())
    }
    #[inline]
    pub fn facexy(&self, x: f64, y: f64) -> &TurtleRef {
        self.borrow_mut().facexy(x, y);
        self
    }
    #[inline]
    pub fn face(&self, other: &TurtleRef) -> &TurtleRef {
        self.borrow_mut().facexy(other.xcor(), other.ycor());
        self
    }
    #[inline]
    pub fn move_in_radius(&self, distance: f64) -> &TurtleRef {
        let r = distance * self.rng().f64().sqrt();
        let theta = self.rng().f64() * std::f64::consts::PI * 2.;
        self.setxy(self.xcor() + r * theta.cos(), self.ycor() + r * theta.sin());
        self
    }
    #[inline]
    pub fn random_headings(&self) -> &TurtleRef {
        self.borrow_mut()
            .set_heading(random_float(self.rng(), 0., 360.));
        self
    }
}

impl TurtleRef {
    #[inline]
    pub fn neighbors(&self) -> PatchSet {
        self.patch_here().neighbors()
    }
    #[inline]
    pub fn neighbors4(&self) -> PatchSet {
        self.patch_here().neighbors4()
    }
    #[inline]
    pub fn turtles_in_radius(&self, distance: f64) -> TurtleSet {
        self.world()
            .borrow()
            .turtles()
            .with(|t| t.distance(&self) <= distance)
    }
    #[inline]
    pub fn in_radius(&self, turtles: &TurtleSet, distance: f64) -> TurtleSet {
        turtles.with(|t| t.distance(&self) <= distance)
    }
}

impl TurtleRef {
    pub fn other(&self, turtles: &TurtleSet) -> TurtleSet {
        let mut turtles = turtles.clone();
        turtles.delete(self);
        turtles
    }
    #[inline]
    pub fn link_neighbors(&self) -> TurtleSet {
        self.borrow().link_neighbors()
    }
    #[inline]
    pub fn in_link_neighbors(&self) -> TurtleSet {
        self.borrow().in_link_neighbors()
    }
    #[inline]
    pub fn out_link_neighbors(&self) -> TurtleSet {
        self.borrow().out_link_neighbors()
    }
}

impl TurtleRef {
    #[inline]
    pub fn create_link_with(&self, other: &TurtleRef) {
        self.borrow().create_link_with(other)
    }
    #[inline]
    pub fn create_link_from(self, other: &TurtleRef) {
        self.borrow().create_link_from(other)
    }
    #[inline]
    pub fn create_link_to(&self, other: &TurtleRef) {
        self.borrow().create_link_to(other)
    }
    #[inline]
    pub fn create_links_with(&self, others: &mut TurtleSet) {
        self.borrow().create_links_with(others)
    }
    #[inline]
    pub fn create_links_from(&self, others: &mut TurtleSet) {
        self.borrow().create_links_from(others)
    }
    #[inline]
    pub fn create_links_to(&self, others: &mut TurtleSet) {
        self.borrow().create_links_to(others)
    }
}

impl TurtleRef {
    #[inline]
    pub fn my_links(&self) -> LinkSet {
        self.borrow().my_links()
    }
    #[inline]
    pub fn my_in_links(&self) -> LinkSet {
        self.borrow().my_in_links()
    }
    #[inline]
    pub fn my_out_links(&self) -> LinkSet {
        self.borrow().my_out_links()
    }
}

impl TurtleRef {
    pub fn hatch(&self, amount: usize) -> TurtleSet {
        let current_count = self.world().borrow().turtles().len();
        let new_turtles = (current_count..amount + current_count)
            .map(|_| TurtleRef::new(self.borrow().clone()))
            .collect_vec();
        new_turtles.iter().for_each(|t| {
            let unid = self.world().borrow_mut().ug.add_node(self.clone());
            t.borrow_mut().set_who(unid.index());
            self.world()
                .borrow_mut()
                .turtles
                .insert(unid.index(), t.clone());
            let dnid = self.world().borrow_mut().dg.add_node(self.clone());
            debug_assert_eq!(dnid, unid)
        });
        new_turtles.into()
    }
    #[inline]
    pub fn sprout(&self, amount: usize) -> TurtleSet {
        self.patch_here().sprout(amount)
    }
}

impl TurtleRef {
    pub fn die(&self) {
        let who = self.who();
        let nid = NodeIndex::new(who);
        self.world().borrow_mut().turtles.delete(self);
        self.world()
            .borrow_mut()
            .turtles
            .values()
            .last()
            .unwrap()
            .borrow_mut()
            .set_who(who);
        self.world().borrow_mut().ug.remove_node(nid);
        self.world().borrow_mut().dg.remove_node(nid);
        self.patch_here().borrow_mut().remove_turtle_on(self)
    }
}

impl TurtleRef {
    pub fn set_random_color(&self) -> &TurtleRef {
        let color = Color::new(
            self.borrow().world().borrow().rng().f32(),
            self.borrow().world().borrow().rng().f32(),
            self.borrow().world().borrow().rng().f32(),
            self.borrow().world().borrow().rng().f32(),
        );
        self.borrow_mut().set_color(color);
        self
    }
}
