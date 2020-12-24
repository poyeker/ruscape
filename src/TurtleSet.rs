use crate::GetRng::GetRng;
use crate::Turtle::*;
use crate::World::*;

use crate::AgentSet::AgentSet;

use crate::Agent::Agent;

use crate::TurtleRef::TurtleRef;

use fastrand::Rng;

use crate::common::concat;
use crate::MapType::HashMap;
use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::iter::FromIterator;
use std::ops::{Add, AddAssign, Deref, DerefMut};
use std::rc::Rc;

pub(crate) type TurtleCollection = HashMap<usize, TurtleRef>;
pub struct TurtleSet {
    w: Option<WorldRef>,
    pub(crate) turtles: TurtleCollection,
}
impl Default for TurtleSet {
    fn default() -> Self {
        Self::new(0, 0, 0, true, 0., 0., 0., 0.)
    }
}
impl Deref for TurtleSet {
    type Target = TurtleCollection;

    fn deref(&self) -> &Self::Target {
        &self.turtles
    }
}
impl DerefMut for TurtleSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.turtles
    }
}
impl TurtleSet {
    pub fn set_w(&mut self, w: Option<WorldRef>) {
        self.w = w;
    }
}

impl Debug for TurtleSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("TurtleSet")
            .field("turtles", &self.turtles)
            .finish()
    }
}

impl TurtleSet {
    pub fn turtles(&self) -> &TurtleCollection {
        &self.turtles
    }
}

impl TurtleSet {
    pub fn new(
        amount: usize,
        max_pxcor: i64,
        max_pycor: i64,

        is_periodic: bool,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
    ) -> Self {
        let mut v: TurtleCollection = HashMap::default();
        for x in 0..amount {
            v.insert(
                x,
                TurtleRef::new(Turtle::new(
                    max_pxcor,
                    max_pycor,
                    is_periodic,
                    x_min,
                    x_max,
                    y_min,
                    y_max,
                )),
            );
        }
        TurtleSet {
            w: None,
            turtles: v,
        }
    }
}

impl From<TurtleCollection> for TurtleSet {
    fn from(turtles: TurtleCollection) -> Self {
        if turtles.len() == 0 {
            Default::default()
        }
        let w = turtles.values().next().unwrap().borrow().world();
        TurtleSet {
            w: Some(WorldRef::new(&w)),
            turtles,
        }
    }
}

impl FromIterator<(usize, TurtleRef)> for TurtleSet {
    #[inline]
    fn from_iter<T: IntoIterator<Item = (usize, TurtleRef)>>(iter: T) -> Self {
        iter.into_iter().collect::<TurtleCollection>().into()
    }
}

impl From<Vec<TurtleRef>> for TurtleSet {
    fn from(turtles: Vec<TurtleRef>) -> Self {
        if turtles.len() == 0 {
            return Default::default();
        }
        let w = turtles.iter().next().unwrap().borrow().world().clone();
        TurtleSet {
            w: Some(WorldRef::new(&w)),
            turtles: turtles
                .iter()
                .map(|t| (t.borrow().who(), t.clone()))
                .collect::<TurtleCollection>(),
        }
    }
}

impl TurtleSet {
    pub fn world(&self) -> Rc<RefCell<World>> {
        let world = self.w.as_ref().unwrap().upgrade().unwrap().clone();
        world
    }
}

impl GetRng for TurtleSet {
    fn get_rng(&self) -> Rng {
        self.world().borrow().rng().clone()
    }
}

impl Clone for TurtleSet {
    fn clone(&self) -> Self {
        if self.len() == 0 {
            TurtleSet {
                w: None,
                turtles: self.turtles.clone(),
            }
        } else {
            TurtleSet {
                w: Some(WorldRef::new(&self.world())),
                turtles: self.turtles.clone(),
            }
        }
    }
}

impl AgentSet<Turtle> for TurtleSet {}

impl Add for TurtleSet {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        concat(&self, &rhs)
    }
}

impl AddAssign for TurtleSet {
    fn add_assign(&mut self, rhs: Self) {
        self.extends(&rhs);
    }
}
