use crate::World::World;
use fastrand::*;

use crate::MapType::VariableMap;
use crate::Variable::Variable;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub trait Agent: Clone + Deref<Target = VariableMap> + DerefMut<Target = VariableMap> {
    type IDX: PartialEq + PartialOrd;
    fn who(&self) -> Self::IDX;
    fn get(&self, key: &str) -> Variable {
        self.deref().get(key).unwrap().clone()
    }
    fn set<T: Into<Variable>>(&mut self, key: &str, value: T) {
        *self.get_mut(key).unwrap() = value.into();
    }
    fn world(&self) -> Rc<RefCell<World>>;
}

#[derive(Default)]
pub struct AgentRef<A: Agent> {
    rng: Rng,
    pub(crate) agent_ref: Rc<RefCell<A>>,
}

impl<A: Agent> AgentRef<A> {
    pub fn rng(&self) -> &Rng {
        &self.rng
    }
}

impl<A: Agent> AgentRef<A> {
    pub fn new(a: A) -> AgentRef<A> {
        AgentRef {
            rng: Rng::new(),
            agent_ref: Rc::new(RefCell::new(a)),
        }
    }
}

impl<A: Agent + Debug> Debug for AgentRef<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.agent_ref.borrow(), f)
    }
}

impl<A: Agent> Clone for AgentRef<A> {
    fn clone(&self) -> Self {
        AgentRef {
            rng: Rng::new(),
            agent_ref: Rc::clone(&self.agent_ref),
        }
    }
}
impl<A: Agent> Deref for AgentRef<A> {
    type Target = RefCell<A>;

    fn deref(&self) -> &Self::Target {
        &self.agent_ref.as_ref()
    }
}

impl<A: Agent> PartialEq for AgentRef<A> {
    fn eq(&self, other: &Self) -> bool {
        self.borrow().who() == other.borrow().who()
    }
}
impl<A: Agent> Eq for AgentRef<A> {}

impl<A: Agent> PartialOrd for AgentRef<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.borrow().who() == other.borrow().who() {
            Some(Ordering::Equal)
        } else if other.borrow().who() > self.borrow().who() {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}
impl<A: Agent> Ord for AgentRef<A> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.borrow().who() == other.borrow().who() {
            Ordering::Equal
        } else if other.borrow().who() > self.borrow().who() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl<A: Agent> AgentRef<A> {
    #[inline]
    pub fn get(&self, key: &'static str) -> Variable {
        self.agent_ref.borrow().get(key)
    }
    #[inline]
    pub fn set<T: Into<Variable>>(&self, key: &'static str, value: T) {
        *self.agent_ref.borrow_mut().get_mut(key).unwrap() = value.into();
    }
}
