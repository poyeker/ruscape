use crate::GetRng::GetRng;
use crate::LinkRef::LinkRef;
use crate::PatchSet::{OriginLocation, PatchSet};
use crate::TurtleRef::TurtleRef;
use crate::TurtleSet::*;
use fastrand::*;

use petgraph::graph::{DiGraph, UnGraph};
use petgraph_gen::classic::*;
use std::cell::RefCell;

use crate::Agent::Agent;
use crate::AgentSet::AgentSet;

use crate::PatchRef::PatchRef;

use crate::Turtle::Turtle;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::rc::{Rc, Weak};

pub struct World {
    pub(crate) max_pxcor: i64,
    pub(crate) max_pycor: i64,
    pub(crate) origin_location: OriginLocation,
    is_periodic: bool,
    pub(crate) x_min: i64,
    pub(crate) x_max: i64,
    pub(crate) y_min: i64,
    pub(crate) y_max: i64,
    rng: Rng,
    pub(crate) turtles: TurtleSet,
    pub(crate) ug: UnGraph<TurtleRef, LinkRef>,
    pub(crate) dg: DiGraph<TurtleRef, LinkRef>,
    patches: PatchSet,
}

impl World {
    pub fn is_periodic(&self) -> bool {
        self.is_periodic
    }
}

impl World {
    pub fn turtles(&self) -> TurtleSet {
        self.turtles.clone()
    }
    pub fn patches(&self) -> PatchSet {
        self.patches.clone()
    }
    pub fn patch(&self, xcor: f64, ycor: f64) -> PatchRef {
        let pxcor = xcor.round().min(self.x_max as f64).max(self.x_min as f64) as i64 - self.x_min;
        let pycor = ycor.round().min(self.y_max as f64).max(self.y_min as f64) as i64 - self.y_min;
        self.patches[(pycor + pxcor * (self.y_max + 1 - self.y_min)) as usize].clone()
    }
    pub fn turtle(&self, who: usize) -> TurtleRef {
        self.turtles.index(who)
    }
}

impl Debug for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("World")
            .field("max_pxcor", &self.max_pxcor)
            .field("max_pycor", &self.max_pycor)
            .field("origin_location", &self.origin_location)
            .field("is_periodic", &self.is_periodic)
            .finish()
    }
}

impl World {
    pub fn rng(&self) -> &Rng {
        &self.rng
    }
}

impl World {
    fn new(
        amount: usize,
        max_pxcor: i64,
        max_pycor: i64,
        origin_location: OriginLocation,
        is_periodic: bool,
    ) -> Rc<RefCell<World>> {
        let (x_min, x_max, y_min, y_max) = match origin_location {
            OriginLocation::Center => (-max_pxcor, max_pxcor, -max_pycor, max_pycor),
            OriginLocation::Corner => (0, max_pxcor, 0, max_pycor),
        };
        let rng = Rng::new();
        let mut turtles = TurtleSet::new(
            amount,
            max_pxcor,
            max_pycor,
            is_periodic,
            x_min as f64 - 0.5,
            x_max as f64 + 0.5,
            y_min as f64 - 0.5,
            y_max as f64 + 0.5,
        );
        let ug = {
            let mut g = empty_graph::<TurtleRef, LinkRef>(0);
            turtles.values().for_each(|turtle| {
                turtle
                    .borrow_mut()
                    .set_who(g.add_node(turtle.clone()).index());
            });
            g
        };
        let dg = {
            let mut g = empty_digraph::<TurtleRef, LinkRef>(0);
            turtles.values().for_each(|turtle| {
                let nid = g.add_node(turtle.clone());
                debug_assert_eq!(nid.index(), turtle.borrow().who());
            });
            g
        };
        turtles.clear();
        ug.node_indices().for_each(|nid| {
            turtles.insert(nid.index(), ug.node_weight(nid).unwrap().clone());
        });
        let patches = PatchSet::new(max_pxcor, max_pycor, origin_location, is_periodic);

        Rc::new(RefCell::new(World {
            max_pxcor,
            max_pycor,
            origin_location,
            is_periodic,
            x_min,
            x_max,
            y_min,
            y_max,
            rng,
            turtles,
            ug,
            dg,
            patches,
        }))
    }

    pub fn init(
        amount: usize,
        max_pxcor: i64,
        max_pycor: i64,
        origin_location: OriginLocation,
        is_periodic: bool,
    ) -> Rc<RefCell<World>> {
        let world_ref = Self::new(amount, max_pxcor, max_pycor, origin_location, is_periodic);
        world_ref.as_ref().borrow_mut().register_world(&world_ref);
        world_ref
            .borrow()
            .turtles
            .values()
            .for_each(|t| t.patch_here().borrow_mut().add_turtle_on(t));
        world_ref.borrow().patches.values().for_each(|p| {
            p.borrow_mut().neighborhood = p.neighbors().concat(p);
            p.borrow_mut().neighborhood4 = p.neighbors4().concat(p);
        });
        world_ref
    }

    fn register_world(&mut self, world_ref: &Rc<RefCell<World>>) {
        self.turtles.set_w(Some(WorldRef::new(world_ref)));
        self.patches.set_w(Some(WorldRef::new(world_ref)));
        self.patches.values().for_each(|p| {
            p.borrow_mut()
                .neighbors
                .set_w(Some(WorldRef::new(world_ref)))
        });
        self.patches.values().for_each(|p| {
            p.borrow_mut()
                .neighbors4
                .set_w(Some(WorldRef::new(world_ref)))
        });
        self.turtles
            .values()
            .for_each(|turtle| turtle.borrow_mut().set_world(&WorldRef::new(world_ref)));
        self.patches
            .values()
            .for_each(|patch| patch.borrow_mut().set_world(&WorldRef::new(world_ref)));
    }
}

pub struct WorldRef {
    world_ref: Weak<RefCell<World>>,
}

impl WorldRef {
    pub fn new(world_ref: &Rc<RefCell<World>>) -> Self {
        WorldRef {
            world_ref: Rc::downgrade(&world_ref),
        }
    }
}

impl Deref for WorldRef {
    type Target = Weak<RefCell<World>>;

    fn deref(&self) -> &Self::Target {
        &self.world_ref
    }
}

impl GetRng for WorldRef {
    fn get_rng(&self) -> Rng {
        self.world_ref.upgrade().unwrap().borrow().rng().clone()
    }
}

impl Clone for WorldRef {
    fn clone(&self) -> Self {
        WorldRef {
            world_ref: Weak::clone(&self.world_ref),
        }
    }
}

impl World {
    pub fn crt(&mut self, amount: usize) -> TurtleSet {
        let current_count = self.turtles.len();
        let new_turtles: TurtleSet = (current_count..(amount + current_count))
            .map(|who| {
                (who, {
                    let t = TurtleRef::new(Turtle::new(
                        self.max_pxcor,
                        self.max_pycor,
                        self.is_periodic(),
                        self.x_min as f64 - 0.5,
                        self.x_max as f64 + 0.5,
                        self.y_min as f64 - 0.5,
                        self.y_max as f64 + 0.5,
                    ));
                    t.borrow_mut()
                        .set_world(&WorldRef::new(&self.turtles.world()));
                    t
                })
            })
            .collect::<TurtleCollection>()
            .into();
        new_turtles.values().for_each(|t| {
            let unid = self.ug.add_node(t.clone());
            let dnid = self.dg.add_node(t.clone());
            debug_assert_eq!(unid, dnid);
            t.borrow_mut().set_who(unid.index());
        });
        self.turtles.extends(&new_turtles);
        new_turtles
    }

    pub fn clear_links(&mut self) {
        self.ug.clear_edges();
        self.dg.clear_edges();
    }
}
