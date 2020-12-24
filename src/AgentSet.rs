use crate::Agent::{Agent, AgentRef};
use crate::GetRng::GetRng;
use itertools::Itertools;

use crate::LinkSet::LinkSet;
use crate::PatchSet::PatchSet;
use crate::TurtleSet::TurtleSet;
use crate::World::{World, WorldRef};

use std::cell::RefCell;

use crate::MapType::HashMap;
use rand::{thread_rng, Rng};

use std::fmt::Debug;
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

pub trait AgentSet<A: Agent>:
    FromIterator<(A::IDX, AgentRef<A>)>
    + GetRng
    + Deref<Target = HashMap<A::IDX, AgentRef<A>>>
    + DerefMut<Target = HashMap<A::IDX, AgentRef<A>>>
    + HasWorld
    + Debug
    + Clone
where
    A::IDX: Eq + Hash + Copy + Debug,
{
    fn count(&self) -> usize {
        self.len()
    }

    fn any<F>(&self, key: F) -> bool
    where
        F: Fn(&AgentRef<A>) -> bool,
    {
        self.values().any(|x| key(x))
    }

    fn all<F>(&self, key: F) -> bool
    where
        F: Fn(&AgentRef<A>) -> bool,
    {
        self.values().all(|x| key(x))
    }

    fn min_one_of<T, F>(&self, mut key: F) -> AgentRef<A>
    where
        T: Ord,
        F: FnMut(&AgentRef<A>) -> T,
    {
        self.values()
            .min_by_key(|&agent| key(agent))
            .unwrap()
            .clone()
    }

    fn max_one_of<T, F>(&self, mut key: F) -> AgentRef<A>
    where
        T: Ord,
        F: FnMut(&AgentRef<A>) -> T,
    {
        self.values()
            .max_by_key(|&agent| key(agent))
            .unwrap()
            .clone()
    }

    fn min_n_of<T, F>(&self, amount: usize, mut key: F) -> Self
    where
        T: Ord,
        F: FnMut(&AgentRef<A>) -> T,
    {
        let mut agents = self.values().collect_vec();
        agents.sort_by_cached_key(|&x| key(x));
        agents
            .iter()
            .take(amount)
            .map(|&x| {
                let who = x.borrow().who();
                (who, x.clone())
            })
            .collect()
    }

    fn max_n_of<T, F>(&self, amount: usize, mut key: F) -> Self
    where
        T: Ord,
        F: FnMut(&AgentRef<A>) -> T,
    {
        let mut agents = self.values().collect_vec();
        agents.sort_by_cached_key(|&x| key(x));
        agents
            .iter()
            .rev()
            .take(amount)
            .map(|&x| {
                let who = x.borrow().who();
                (who, x.clone())
            })
            .collect()
    }

    fn one_of(&self) -> AgentRef<A> {
        let rng = self.get_rng();
        let idx = rng.usize(0..self.len());
        self.values().nth(idx).unwrap().clone()
    }

    #[inline]
    fn one_of_weighted_by<K, F>(&self, key: F) -> AgentRef<A>
    where
        K: Ord
            + Clone
            + Default
            + rand_distr::uniform::SampleUniform
            + for<'a> std::ops::AddAssign<&'a K>,
        F: FnMut(&AgentRef<A>) -> K,
    {
        self.n_of_weighted_by(1, key)[0].clone()
    }

    #[inline]
    fn n_of(&self, amount: usize) -> Self {
        let mut agents = self.clone();
        (0..amount)
            .map(|_| {
                let agent = agents.one_of();
                agents.delete(&agent);
                let who = agent.borrow().who();
                (who, agent)
            })
            .collect()
    }

    #[inline]
    fn n_of_with_repeats(&self, amount: usize) -> Self
    where
        Self: Sized,
    {
        self.n_of_weighted_by(amount, |_| 1)
    }

    fn n_of_weighted_by<K, F>(&self, amount: usize, mut key: F) -> Self
    where
        K: Ord
            + Clone
            + Default
            + rand_distr::uniform::SampleUniform
            + for<'a> std::ops::AddAssign<&'a K>,
        F: FnMut(&AgentRef<A>) -> K,
    {
        let keys = (0..self.len()).map(|idx| key(&self[idx]));
        let weights = rand::distributions::WeightedIndex::new(keys).unwrap();
        (0..amount)
            .map(|_| {
                let agent = self[thread_rng().sample(&weights)].clone();
                let who = agent.borrow().who();
                (who, agent)
            })
            .collect()
    }

    fn ask<F: FnMut(&AgentRef<A>)>(&self, mut f: F) {
        let mut keys = (0..self.len()).collect_vec();
        let rng = self.get_rng();
        rng.shuffle(&mut keys);
        for &key in keys.iter() {
            f(&self[key]);
        }
    }

    fn ask_each<F: FnMut(&AgentRef<A>)>(&self, mut f: F) {
        self.values().for_each(|agent| f(agent));
    }

    fn report<T, F: FnMut(&AgentRef<A>) -> T>(&self, mut f: F) -> Vec<(A::IDX, T)> {
        self.iter().map(|(&key, agent)| (key, f(agent))).collect()
    }

    fn own(&mut self, owns: Vec<&'static str>) {
        owns.iter().for_each(|&own| {
            self.ask(|p| {
                p.borrow_mut().insert(own, Default::default());
            })
        })
    }

    fn index(&self, idx: A::IDX) -> AgentRef<A> {
        self.get(&idx).unwrap().clone()
    }

    fn with<F: FnMut(&AgentRef<A>) -> bool>(&self, f: F) -> Self {
        self.values()
            .cloned()
            .filter(f)
            .map(|agent| {
                let who = agent.borrow().who();
                (who, agent)
            })
            .collect()
    }

    fn append(&mut self, agent: &AgentRef<A>) {
        if self.len() == 0 {
            self.set_world(&WorldRef::new(&agent.borrow().world()));
        }
        self.insert(agent.borrow().who(), agent.clone());
    }

    fn concat(&self, agent: &AgentRef<A>) -> Self {
        let mut agents = self.clone();
        agents.append(agent);
        agents
    }

    fn extends(&mut self, agents: &Self) {
        if self.len() == 0 {
            self.set_world(&WorldRef::new(&agents.get_world()));
        }
        self.extend(agents.iter().map(|(&k, v)| (k, v.clone())));
    }

    fn delete(&mut self, agent: &AgentRef<A>) {
        let who = agent.borrow().who();
        self.swap_remove(&who);
    }
}

pub trait HasWorld {
    fn get_world(&self) -> Rc<RefCell<World>>;
    fn set_world(&mut self, w: &WorldRef);
}

impl HasWorld for TurtleSet {
    fn get_world(&self) -> Rc<RefCell<World>> {
        self.world()
    }
    fn set_world(&mut self, w: &WorldRef) {
        self.set_w(Some(w.clone()))
    }
}

impl HasWorld for PatchSet {
    fn get_world(&self) -> Rc<RefCell<World>> {
        self.world()
    }
    fn set_world(&mut self, w: &WorldRef) {
        self.set_w(Some(w.clone()))
    }
}
impl HasWorld for LinkSet {
    fn get_world(&self) -> Rc<RefCell<World>> {
        self.world()
    }
    fn set_world(&mut self, w: &WorldRef) {
        self.set_w(Some(w.clone()))
    }
}
