use crate::Agent::Agent;
use crate::GetRng::GetRng;
use crate::Patch::*;
use crate::World::{World, WorldRef};
use itertools::Itertools;

use crate::AgentSet::AgentSet;
use crate::PatchRef::PatchRef;
use fastrand::Rng;

use crate::common::concat;
use crate::MapType::HashMap;
use macroquad::drawing::*;
use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::iter::FromIterator;
use std::ops::{Add, AddAssign, Deref, DerefMut};
use std::rc::Rc;

#[derive(Copy, Clone, Debug)]
pub enum OriginLocation {
    Center,
    Corner,
}
type PatchCollection = HashMap<(i64, i64), PatchRef>;
pub struct PatchSet {
    w: Option<WorldRef>,
    pub(crate) patches: PatchCollection,
}

impl PatchSet {
    pub(crate) fn set_w(&mut self, w: Option<WorldRef>) {
        self.w = w;
    }
}

impl Debug for PatchSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("PatchSet")
            .field(
                "patches",
                &self
                    .patches
                    .keys()
                    .map(|(x, y)| format!("Patch {} {}", x, y))
                    .collect::<Vec<String>>(),
            )
            .finish()
    }
}
impl Deref for PatchSet {
    type Target = PatchCollection;

    fn deref(&self) -> &Self::Target {
        &self.patches
    }
}

impl DerefMut for PatchSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.patches
    }
}

impl PatchSet {
    pub(crate) fn world(&self) -> Rc<RefCell<World>> {
        let world = self.w.as_ref().unwrap().upgrade().unwrap().clone();
        world
    }
}

impl PatchSet {
    pub fn new(
        max_pxcor: i64,
        max_pycor: i64,
        origin_location: OriginLocation,
        is_periodic: bool,
    ) -> Self {
        match origin_location {
            OriginLocation::Center => {
                let patch_iter = ((-max_pxcor)..=max_pxcor)
                    .cartesian_product((-max_pycor)..=max_pycor)
                    .map(|(pxcor, pycor)| PatchRef::new(Patch::new(pxcor, pycor, is_periodic)));
                let patches: PatchCollection = patch_iter
                    .clone()
                    .map(|patch| (patch.borrow().pxcor(), patch.borrow().pycor()))
                    .zip(patch_iter)
                    .collect();
                Self::set_neighbors(max_pxcor, max_pycor, origin_location, &patches);
                Self::set_neighbors4(max_pxcor, max_pycor, origin_location, &patches);
                PatchSet { w: None, patches }
            }
            OriginLocation::Corner => {
                let patch_iter = (0..=max_pxcor)
                    .cartesian_product(0..=max_pycor)
                    .map(|(pxcor, pycor)| PatchRef::new(Patch::new(pxcor, pycor, is_periodic)));
                let patches = patch_iter
                    .clone()
                    .map(|patch| (patch.borrow().pxcor(), patch.borrow().pycor()))
                    .zip(patch_iter)
                    .collect();
                Self::set_neighbors(max_pxcor, max_pycor, origin_location, &patches);
                Self::set_neighbors4(max_pxcor, max_pycor, origin_location, &patches);
                PatchSet { w: None, patches }
            }
        }
    }

    fn set_neighbors(
        max_pxcor: i64,
        max_pycor: i64,
        origin_location: OriginLocation,
        patches: &HashMap<(i64, i64), PatchRef>,
    ) {
        patches.iter().for_each(|(_, patch)| {
            let pxcor = patch.borrow().pxcor();
            let pycor = patch.borrow().pycor();
            let (x_min, x_max, y_min, y_max) = match origin_location {
                OriginLocation::Center => (-max_pxcor, max_pxcor, -max_pycor, max_pycor),
                OriginLocation::Corner => (0, max_pxcor, 0, max_pycor),
            };
            let xcors = (pxcor - 1)..=(pxcor + 1);
            let ycors = (pycor - 1)..=(pycor + 1);

            let cors = xcors.cartesian_product(ycors);

            let xrange = match origin_location {
                OriginLocation::Center => max_pxcor * 2 + 1,
                OriginLocation::Corner => max_pxcor + 1,
            };
            let yrange = match origin_location {
                OriginLocation::Center => max_pycor * 2 + 1,
                OriginLocation::Corner => max_pycor + 1,
            };

            let neighbor_vec = if patch.borrow().is_periodic() {
                cors.filter(|(x, y)| !(*x == (pxcor - x_min) && *y == (pycor - y_min)))
                    .map(|(x, y)| {
                        patches[&(
                            (x - x_min).rem_euclid(xrange) + x_min,
                            (y - y_min).rem_euclid(yrange) + y_min,
                        )]
                            .clone()
                    })
                    .collect::<Vec<PatchRef>>()
            } else {
                cors.filter(|(x, y)| !(*x == pxcor - x_min && *y == pycor - y_min))
                    .filter(|(x, y)| *x >= x_min && *x <= x_max && *y >= y_min && *y <= y_max)
                    .map(|(x, y)| {
                        patches[&(
                            (x - x_min).rem_euclid(xrange) + x_min,
                            (y - y_min).rem_euclid(yrange) + y_min,
                        )]
                            .clone()
                    })
                    .collect::<Vec<PatchRef>>()
            };
            let neighbors: PatchCollection = neighbor_vec
                .iter()
                .map(|patch| (patch.borrow().pxcor(), patch.borrow().pycor()))
                .zip(neighbor_vec.iter().cloned())
                .collect();
            let neighbors_set: PatchSet = neighbors.into();
            patch.borrow_mut().set_neighbors(neighbors_set);
        });
    }

    fn set_neighbors4(
        max_pxcor: i64,
        max_pycor: i64,
        origin_location: OriginLocation,
        patches: &HashMap<(i64, i64), PatchRef>,
    ) {
        patches.iter().for_each(|(_, patch)| {
            let pxcor = patch.borrow().pxcor();
            let pycor = patch.borrow().pycor();
            let (x_min, x_max, y_min, y_max) = match origin_location {
                OriginLocation::Center => (-max_pxcor, max_pxcor, -max_pycor, max_pycor),
                OriginLocation::Corner => (0, max_pxcor, 0, max_pycor),
            };
            let cors = vec![
                (pxcor - 1, pycor),
                (pxcor + 1, pycor),
                (pxcor, pycor - 1),
                (pxcor, pycor + 1),
            ];
            let xrange = match origin_location {
                OriginLocation::Center => max_pxcor * 2 + 1,
                OriginLocation::Corner => max_pxcor + 1,
            };
            let yrange = match origin_location {
                OriginLocation::Center => max_pycor * 2 + 1,
                OriginLocation::Corner => max_pycor + 1,
            };

            let neighbor_vec = if patch.borrow().is_periodic() {
                cors.iter()
                    .map(|(x, y)| {
                        patches[&(
                            (x - x_min).rem_euclid(xrange) + x_min,
                            (y - y_min).rem_euclid(yrange) + y_min,
                        )]
                            .clone()
                    })
                    .collect::<Vec<PatchRef>>()
            } else {
                cors.iter()
                    .filter(|(x, y)| *x >= x_min && *x <= x_max && *y >= y_min && *y <= y_max)
                    .map(|(x, y)| {
                        patches[&(
                            (x - x_min).rem_euclid(xrange) + x_min,
                            (y - y_min).rem_euclid(yrange) + y_min,
                        )]
                            .clone()
                    })
                    .collect::<Vec<PatchRef>>()
            };
            let neighbors: PatchCollection = neighbor_vec
                .iter()
                .map(|patch| (patch.borrow().pxcor(), patch.borrow().pycor()))
                .zip(neighbor_vec.iter().cloned())
                .collect();
            let neighbors_set: PatchSet = neighbors.into();
            patch.borrow_mut().set_neighbors4(neighbors_set);
        });
    }
}

impl From<HashMap<(i64, i64), PatchRef>> for PatchSet {
    fn from(patches: PatchCollection) -> Self {
        if patches.len() == 0 {
            return PatchSet::default();
        }
        let (_, patch1) = patches.iter().next().unwrap().clone();
        let w = Option::clone(patch1.borrow().w());
        PatchSet { w, patches }
    }
}

impl From<Vec<PatchRef>> for PatchSet {
    fn from(patches: Vec<PatchRef>) -> Self {
        let w = patches.iter().next().unwrap().borrow().world();
        PatchSet {
            w: Some(WorldRef::new(&w)),
            patches: patches
                .iter()
                .map(|p| (p.borrow().who(), p.clone()))
                .collect(),
        }
    }
}
impl GetRng for PatchSet {
    fn get_rng(&self) -> Rng {
        self.world().borrow().rng().clone()
    }
}

impl Default for PatchSet {
    fn default() -> Self {
        PatchSet {
            w: None,
            patches: HashMap::<(i64, i64), PatchRef>::default(),
        }
    }
}

impl World {
    pub fn clear_patches(&mut self) {
        self.patches()
            .patches
            .iter()
            .for_each(|(_, patch)| patch.borrow_mut().set_pcolor(BLACK));
    }
}

impl Clone for PatchSet {
    fn clone(&self) -> Self {
        if self.len() == 0 {
            PatchSet {
                w: None,
                patches: self.patches.clone(),
            }
        } else {
            PatchSet {
                w: Some(WorldRef::new(&self.world())),
                patches: self.patches.clone(),
            }
        }
    }
}

impl FromIterator<((i64, i64), PatchRef)> for PatchSet {
    fn from_iter<T: IntoIterator<Item = ((i64, i64), PatchRef)>>(iter: T) -> Self {
        iter.into_iter().collect::<PatchCollection>().into()
    }
}

impl AgentSet<Patch> for PatchSet {}

impl Add for PatchSet {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        concat(&self, &rhs)
    }
}

impl AddAssign for PatchSet {
    fn add_assign(&mut self, rhs: Self) {
        self.extends(&rhs);
    }
}
