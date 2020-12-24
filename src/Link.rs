use crate::Agent::Agent;
use crate::LinkRef::LinkRef;
use crate::LinkSet::LinkSet;
use crate::World::{World, WorldRef};

use crate::MapType::VariableMap;
use petgraph::graph::NodeIndex;
use std::cell::RefCell;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub struct Link {
    w: Option<WorldRef>,
    nid1: NodeIndex,
    nid2: NodeIndex,
    variables: VariableMap,
}

impl Deref for Link {
    type Target = VariableMap;

    fn deref(&self) -> &Self::Target {
        &self.variables
    }
}

impl DerefMut for Link {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.variables
    }
}

impl Default for Link {
    fn default() -> Self {
        Link {
            w: None,
            nid1: Default::default(),
            nid2: Default::default(),
            variables: Default::default(),
        }
    }
}

impl Debug for Link {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Link")
            .field("nid1", &self.nid1)
            .field("nid2", &self.nid2)
            .finish()
    }
}

impl Agent for Link {
    type IDX = (usize, usize);

    fn who(&self) -> Self::IDX {
        (self.nid1.index(), self.nid2.index())
    }
    fn world(&self) -> Rc<RefCell<World>> {
        self.w.as_ref().unwrap().upgrade().unwrap()
    }
}

impl Link {
    pub fn other(&self, others: LinkSet) -> LinkSet {
        let mut links = others.clone();
        links.remove(&self.who());
        links
    }
}

impl Link {
    pub fn to_ref(&self) -> LinkRef {
        unimplemented!()
    }
}
impl Clone for Link {
    fn clone(&self) -> Self {
        Self {
            w: Some(WorldRef::new(&self.world())),
            nid1: self.nid1,
            nid2: self.nid2,
            variables: self.variables.clone(),
        }
    }
}
