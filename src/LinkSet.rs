use crate::Agent::Agent;
use crate::AgentSet::AgentSet;
use crate::GetRng::GetRng;
use crate::Link::Link;
use fastrand::*;

use crate::World::*;

use crate::common::concat;
use crate::LinkRef::LinkRef;
use crate::MapType::HashMap;
use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::iter::FromIterator;
use std::ops::{Add, AddAssign, Deref, DerefMut};
use std::rc::Rc;

type LinkCollection = HashMap<(usize, usize), LinkRef>;
pub struct LinkSet {
    w: Option<WorldRef>,
    pub(crate) raw: LinkCollection,
}

impl Deref for LinkSet {
    type Target = LinkCollection;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}
impl DerefMut for LinkSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raw
    }
}
impl LinkSet {
    pub fn set_w(&mut self, w: Option<WorldRef>) {
        self.w = w;
    }
}

impl Debug for LinkSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("LinkSet").field("raw", &self.raw).finish()
    }
}

impl LinkSet {
    pub fn w(&self) -> &Option<WorldRef> {
        &self.w
    }
    pub fn world(&self) -> Rc<RefCell<World>> {
        self.w.as_ref().unwrap().upgrade().unwrap()
    }
    pub fn raw(&self) -> &LinkCollection {
        &self.raw
    }
}

impl LinkSet {
    pub fn new() -> Self {
        let v: LinkCollection = HashMap::default();

        LinkSet { w: None, raw: v }
    }
}

impl From<LinkCollection> for LinkSet {
    fn from(raw: LinkCollection) -> Self {
        let w = raw.values().next().unwrap().borrow().world();
        LinkSet {
            w: Some(WorldRef::new(&w)),
            raw,
        }
    }
}

impl From<Vec<LinkRef>> for LinkSet {
    fn from(raw: Vec<LinkRef>) -> Self {
        let w = raw.iter().next().unwrap().borrow().world();
        LinkSet {
            w: Some(WorldRef::new(&w)),
            raw: raw
                .iter()
                .map(|t| (t.borrow().who(), t.clone()))
                .collect::<LinkCollection>(),
        }
    }
}

impl GetRng for LinkSet {
    fn get_rng(&self) -> Rng {
        self.w().as_ref().unwrap().get_rng()
    }
}

impl Clone for LinkSet {
    fn clone(&self) -> Self {
        LinkSet {
            w: Some(self.w().as_ref().unwrap().clone()),
            raw: self.raw.clone(),
        }
    }
}

impl LinkSet {
    pub fn ask<F: FnMut(&LinkRef)>(&self, f: F) {
        self.values().for_each(f);
    }

    pub fn report<T, F: FnMut(&LinkRef) -> T>(&self, f: F) -> Vec<T> {
        self.values().map(f).collect()
    }
}

impl FromIterator<((usize, usize), LinkRef)> for LinkSet {
    fn from_iter<T: IntoIterator<Item = ((usize, usize), LinkRef)>>(iter: T) -> Self {
        iter.into_iter().collect::<LinkCollection>().into()
    }
}

impl AgentSet<Link> for LinkSet {}

impl Add for LinkSet {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        concat(&self, &rhs)
    }
}

impl AddAssign for LinkSet {
    fn add_assign(&mut self, rhs: Self) {
        self.extends(&rhs);
    }
}
