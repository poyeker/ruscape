use crate::Agent::Agent;
use crate::PatchSet::*;
use crate::World::{World, WorldRef};

use macroquad::drawing::*;

use crate::AgentSet::AgentSet;

use crate::Position::Position;

use crate::TurtleRef::TurtleRef;

use crate::TurtleSet::TurtleSet;

use crate::MapType::VariableMap;
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct Patch {
    w: Option<WorldRef>,
    pxcor: i64,
    pycor: i64,
    pcolor: Color,
    is_periodic: bool,
    pub(crate) neighbors: PatchSet,
    pub(crate) neighborhood: PatchSet,
    pub(crate) neighborhood4: PatchSet,
    pub(crate) neighbors4: PatchSet,
    variables: VariableMap,
    turtles_on: TurtleSet,
}
impl Deref for Patch {
    type Target = VariableMap;

    fn deref(&self) -> &Self::Target {
        &self.variables
    }
}

impl DerefMut for Patch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.variables
    }
}

impl Default for Patch {
    fn default() -> Self {
        Self::new(0, 0, true)
    }
}
impl Debug for Patch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("patch")
            .field("pxcor", &self.pxcor)
            .field("pycor", &self.pycor)
            .finish()
    }
}

impl PartialEq for Patch {
    fn eq(&self, other: &Self) -> bool {
        self.pxcor == other.pxcor && self.pycor == other.pycor
    }
}

impl Patch {
    pub fn neighbors(&self) -> &PatchSet {
        &self.neighbors
    }
    pub fn neighbors4(&self) -> &PatchSet {
        &self.neighbors4
    }
    pub fn neighborhood4(&self) -> &PatchSet {
        &self.neighborhood4
    }
    pub fn neighborhood(&self) -> &PatchSet {
        &self.neighborhood
    }
    pub(crate) fn turtles_on(&self) -> TurtleSet {
        self.turtles_on.clone()
    }
}

impl Patch {
    pub(crate) fn set_neighbors(&mut self, neighbors: PatchSet) {
        self.neighbors = neighbors;
    }
    pub(crate) fn set_neighbors4(&mut self, neighbors4: PatchSet) {
        self.neighbors4 = neighbors4;
    }
    pub(crate) fn add_turtle_on(&mut self, turtle: &TurtleRef) {
        self.turtles_on.append(turtle)
    }
    pub(crate) fn remove_turtle_on(&mut self, turtle: &TurtleRef) {
        self.turtles_on.delete(turtle);
    }
}

impl Patch {
    pub fn other(&self, patch_set: &PatchSet) -> PatchSet {
        let mut patches = patch_set.clone();
        patches.remove(&(self.pxcor, self.pycor));
        patches
    }
}

impl Agent for Patch {
    type IDX = (i64, i64);
    fn who(&self) -> Self::IDX {
        (self.pxcor, self.pycor)
    }
    fn world(&self) -> Rc<RefCell<World>> {
        self.w.as_ref().unwrap().upgrade().unwrap()
    }
}
impl Patch {
    pub(crate) fn set_world(&mut self, w: &WorldRef) {
        self.w = Some(w.clone());
    }
    pub(crate) fn set_pcolor(&mut self, pcolor: Color) {
        self.pcolor = pcolor;
    }
}

impl Patch {
    pub(crate) fn w(&self) -> &Option<WorldRef> {
        &self.w
    }
    pub(crate) fn pxcor(&self) -> i64 {
        self.pxcor
    }
    pub(crate) fn pycor(&self) -> i64 {
        self.pycor
    }
    pub(crate) fn pcolor(&self) -> Color {
        self.pcolor
    }
    pub(crate) fn is_periodic(&self) -> bool {
        self.is_periodic
    }
}

impl Patch {
    pub(crate) fn new(pxcor: i64, pycor: i64, is_periodic: bool) -> Self {
        Patch {
            w: None,
            pxcor,
            pycor,
            pcolor: BLACK,
            is_periodic,
            neighbors: Default::default(),
            neighborhood: Default::default(),
            neighborhood4: Default::default(),
            neighbors4: Default::default(),
            variables: VariableMap::default(),
            turtles_on: Default::default(),
        }
    }
}

impl Position for Patch {
    fn position(&self) -> (f64, f64) {
        (self.pxcor as f64, self.pycor as f64)
    }
}

impl Clone for Patch {
    fn clone(&self) -> Self {
        Self {
            w: Some(WorldRef::new(&self.world())),
            pxcor: self.pxcor,
            pycor: self.pycor,
            pcolor: self.pcolor,
            is_periodic: self.is_periodic,
            neighbors: self.neighbors.clone(),
            neighborhood: self.neighborhood.clone(),
            neighborhood4: self.neighborhood4.clone(),
            neighbors4: self.neighbors4.clone(),
            variables: self.variables.clone(),
            turtles_on: self.turtles_on.clone(),
        }
    }
}
