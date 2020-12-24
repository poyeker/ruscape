use crate::Agent::Agent;
use crate::AgentSet::AgentSet;

use crate::LinkSet::LinkSet;

use crate::PatchRef::PatchRef;

use crate::Position::Position;
use crate::TurtleRef::TurtleRef;
use crate::TurtleSet::{TurtleCollection, TurtleSet};
use crate::World::{World, WorldRef};

use itertools::Itertools;
use macroquad::drawing::*;
use petgraph::graph::NodeIndex;
use petgraph::Direction;
use std::cell::RefCell;
use std::f64;
use std::fmt;
use std::fmt::{Debug, Formatter};

use crate::Toroidal::*;

use crate::MapType::VariableMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct Turtle {
    w: Option<WorldRef>,
    who: usize,
    color: Color,
    heading: f64,
    xcor: f64,
    ycor: f64,
    max_pxcor: i64,
    max_pycor: i64,
    is_periodic: bool,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    shape: String,
    size: f64,
    variables: VariableMap,
}

impl Turtle {
    pub fn x_min(&self) -> f64 {
        self.x_min
    }
    pub fn x_max(&self) -> f64 {
        self.x_max
    }
    pub fn y_min(&self) -> f64 {
        self.y_min
    }
    pub fn y_max(&self) -> f64 {
        self.y_max
    }
}

impl Turtle {
    pub fn new(
        max_pxcor: i64,
        max_pycor: i64,

        is_periodic: bool,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
    ) -> Self {
        Turtle {
            max_pxcor,
            max_pycor,
            x_min,
            x_max,
            y_min,
            y_max,
            is_periodic,
            ..Default::default()
        }
    }
}

impl Debug for Turtle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Turtle")
            .field("who", &self.who)
            .field("color", &self.color)
            .field("heading", &self.heading)
            .field("xcor", &self.xcor)
            .field("ycor", &self.ycor)
            .field("shape", &self.shape)
            .field("size", &self.size)
            .finish()
    }
}

impl Deref for Turtle {
    type Target = VariableMap;

    fn deref(&self) -> &Self::Target {
        &self.variables
    }
}

impl DerefMut for Turtle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.variables
    }
}

impl Turtle {
    pub(crate) fn set_world(&mut self, w: &WorldRef) {
        self.w = Some(w.clone());
    }
    pub(crate) fn set_who(&mut self, who: usize) {
        self.who = who;
    }
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    pub fn set_heading(&mut self, heading: f64) {
        self.heading = heading;
    }
    pub fn set_xcor(&mut self, xcor: f64) {
        self.xcor = xcor;
    }
    pub fn set_ycor(&mut self, ycor: f64) {
        self.ycor = ycor;
    }
    pub fn set_shape(&mut self, shape: String) {
        self.shape = shape;
    }
    pub fn set_size(&mut self, size: f64) {
        self.size = size;
    }

    pub(crate) fn setxy(&mut self, x: f64, y: f64) {
        if self.world().borrow().is_periodic() {
            self.set_xcor(toroidal_transform(x, self.x_min, self.x_max));
            self.set_ycor(toroidal_transform(y, self.y_min, self.y_max));
        } else {
            self.set_xcor(x.min(self.x_max).max(self.x_min));
            self.set_ycor(y.min(self.y_max).max(self.y_min));
        }
    }

    pub(crate) fn left(&mut self, degree: f64) {
        self.set_heading(self.heading - degree);
    }
    pub(crate) fn right(&mut self, degree: f64) {
        self.set_heading(self.heading + degree);
    }
    pub(crate) fn facexy(&mut self, x: f64, y: f64) {
        self.set_heading(self.towardsxy(x, y));
    }
    pub(crate) fn towardsxy(&self, x: f64, y: f64) -> f64 {
        let dx = if self.is_periodic {
            toroidal_distance(x, self.xcor, self.x_max - self.x_min)
        } else {
            x - self.xcor
        };
        let dy = if self.is_periodic {
            toroidal_distance(y, self.ycor, self.y_max - self.y_min)
        } else {
            y - self.ycor
        };
        90. - dy.atan2(dx).to_degrees()
    }
}

impl Turtle {
    pub(crate) fn color(&self) -> Color {
        self.color
    }
    pub(crate) fn heading(&self) -> f64 {
        self.heading
    }
    pub(crate) fn xcor(&self) -> f64 {
        self.xcor
    }
    pub(crate) fn ycor(&self) -> f64 {
        self.ycor
    }
    pub(crate) fn shape(&self) -> &str {
        &self.shape
    }
    pub(crate) fn size(&self) -> f64 {
        self.size
    }
    pub fn nid(&self) -> NodeIndex {
        NodeIndex::new(self.who)
    }

    pub fn patch_here(&self) -> PatchRef {
        self.world().borrow().patch(self.xcor(), self.ycor())
    }
}

impl Default for Turtle {
    fn default() -> Self {
        Turtle {
            w: None,
            who: Default::default(),
            color: BLUE,
            heading: 0.0,
            xcor: 0.0,
            ycor: 0.0,
            max_pxcor: 0,
            max_pycor: 0,
            is_periodic: false,
            x_min: 0.0,
            x_max: 0.0,
            y_min: 0.0,
            y_max: 0.0,
            shape: "default".to_string(),
            size: 1.0,
            variables: VariableMap::default(),
        }
    }
}

impl Agent for Turtle {
    type IDX = usize;

    fn who(&self) -> Self::IDX {
        self.who
    }

    fn world(&self) -> Rc<RefCell<World>> {
        self.w.as_ref().unwrap().upgrade().unwrap()
    }
}

impl Turtle {
    pub fn link_neighbors(&self) -> TurtleSet {
        self.world()
            .borrow()
            .ug
            .neighbors(self.nid())
            .map(|nid| {
                (
                    nid.index(),
                    self.world().borrow().ug.node_weight(nid).unwrap().clone(),
                )
            })
            .collect::<TurtleCollection>()
            .into()
    }

    fn di_link_neighbors(&self, dir: Direction) -> TurtleSet {
        self.world()
            .borrow()
            .dg
            .neighbors_directed(self.nid(), dir)
            .map(|nid| {
                (
                    nid.index(),
                    self.world().borrow().dg.node_weight(nid).unwrap().clone(),
                )
            })
            .collect::<TurtleCollection>()
            .into()
    }

    pub fn in_link_neighbors(&self) -> TurtleSet {
        self.di_link_neighbors(Direction::Incoming)
    }

    pub fn out_link_neighbors(&self) -> TurtleSet {
        self.di_link_neighbors(Direction::Outgoing)
    }
    pub fn create_link_with(&self, other: &TurtleRef) {
        self.world()
            .borrow_mut()
            .ug
            .add_edge(self.nid(), other.borrow().nid(), Default::default());
    }

    pub fn create_links_with(&self, other: &mut TurtleSet) {
        other.ask(|t| self.create_link_with(t));
    }

    pub fn create_link_to(&self, other: &TurtleRef) {
        self.world()
            .borrow_mut()
            .dg
            .add_edge(self.nid(), other.borrow().nid(), Default::default());
    }

    pub fn create_links_to(&self, others: &mut TurtleSet) {
        others.ask(|t| self.create_link_to(t));
    }

    pub fn create_link_from(&self, other: &TurtleRef) {
        self.world()
            .borrow_mut()
            .dg
            .add_edge(other.borrow().nid(), self.nid(), Default::default());
    }

    pub fn create_links_from(&self, others: &mut TurtleSet) {
        others.ask(|t| self.create_link_from(t));
    }

    pub fn my_links(&self) -> LinkSet {
        self.world()
            .borrow()
            .ug
            .edges(self.nid())
            .map(|e| e.weight().clone())
            .collect_vec()
            .into()
    }
    fn my_di_links(&self, dir: Direction) -> LinkSet {
        self.world()
            .borrow()
            .dg
            .edges_directed(self.nid(), dir)
            .map(|e| e.weight().clone())
            .collect_vec()
            .into()
    }
    pub fn my_in_links(&self) -> LinkSet {
        self.my_di_links(Direction::Incoming)
    }

    pub fn my_out_links(&self) -> LinkSet {
        self.my_di_links(Direction::Outgoing)
    }
}

impl Turtle {
    pub(crate) fn distance(&self, other: &Turtle) -> f64 {
        let dx = if self.is_periodic {
            toroidal_distance(self.xcor, other.xcor, self.x_max - self.x_min)
        } else {
            self.xcor - other.xcor
        };
        let dy = if self.is_periodic {
            toroidal_distance(self.ycor, other.ycor, self.y_max - self.y_min)
        } else {
            self.ycor - other.ycor
        };
        (dx * dx + dy * dy).sqrt()
    }
    pub(crate) fn distancexy(&self, x: f64, y: f64) -> f64 {
        let dx = if self.is_periodic {
            toroidal_distance(x, self.xcor, self.x_max - self.x_min)
        } else {
            self.xcor - x
        };
        let dy = if self.is_periodic {
            toroidal_distance(y, self.ycor, self.y_max - self.y_min)
        } else {
            self.ycor - y
        };
        (dx * dx + dy * dy).sqrt()
    }
}

impl Position for Turtle {
    fn position(&self) -> (f64, f64) {
        (self.xcor, self.ycor)
    }
}

impl Clone for Turtle {
    fn clone(&self) -> Self {
        Self {
            w: Some(WorldRef::new(&self.world())),
            who: self.who,
            color: self.color,
            heading: self.heading,
            xcor: self.xcor,
            max_pxcor: self.max_pxcor,
            max_pycor: self.max_pycor,
            is_periodic: self.is_periodic,
            x_min: self.x_min,
            x_max: self.x_max,
            y_min: self.y_min,
            y_max: self.y_max,
            ycor: self.ycor,
            shape: self.shape.clone(),
            size: self.size,
            variables: self.variables.clone(),
        }
    }
}

impl PartialEq for Turtle {
    fn eq(&self, other: &Self) -> bool {
        self.who() == other.who()
    }
}
impl Eq for Turtle {}
