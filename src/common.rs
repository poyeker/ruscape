use crate::Agent::{Agent, AgentRef};
use crate::AgentSet::AgentSet;
use fastrand::Rng;

use std::fmt::Debug;
use std::hash::Hash;

pub fn n_values<T, F>(amount: usize, key: F) -> Vec<T>
where
    F: Fn() -> T,
{
    let mut v = Vec::new();
    for _ in 0..amount {
        v.push(key())
    }
    v
}

#[inline]
pub fn one_of<A, AS>(agent_set: &AS) -> AgentRef<A>
where
    A: Agent,
    A::IDX: Eq + Hash + Copy + Debug,
    AS: AgentSet<A>,
{
    agent_set.one_of()
}

#[inline]
pub fn n_of_with_repeats<A, AS>(agent_set: AS, amount: usize) -> AS
where
    A: Agent,
    A::IDX: Eq + Hash + Copy + Debug,
    AS: AgentSet<A>,
{
    agent_set.n_of_with_repeats(amount)
}
#[inline]
pub fn one_of_weighted_by<A, K, F, AS>(agent_set: AS, key: F) -> AgentRef<A>
where
    A: Agent,
    A::IDX: Eq + Hash + Copy + Debug,
    K: Ord
        + Clone
        + Default
        + rand_distr::uniform::SampleUniform
        + for<'a> std::ops::AddAssign<&'a K>,
    F: FnMut(&AgentRef<A>) -> K,
    AS: AgentSet<A>,
{
    agent_set.one_of_weighted_by(key)
}
#[inline]
pub fn n_of_weighted_by<A, K, F, AS>(agent_set: AS, amount: usize, key: F) -> AS
where
    A: Agent,
    A::IDX: Eq + Hash + Copy + Debug,
    K: Ord
        + Clone
        + Default
        + rand_distr::uniform::SampleUniform
        + for<'a> std::ops::AddAssign<&'a K>,
    F: FnMut(&AgentRef<A>) -> K,
    AS: AgentSet<A>,
{
    agent_set.n_of_weighted_by(amount, key)
}
#[inline]
pub fn max_one_of<A, T, F, AS>(agents: AS, key: F) -> AgentRef<A>
where
    A: Agent,
    A::IDX: Eq + Hash + Copy + Debug,
    T: Ord,
    AS: AgentSet<A>,
    F: Fn(&AgentRef<A>) -> T,
{
    agents.max_one_of(key)
}
#[inline]
pub fn min_one_of<A, T, F, AS>(agents: AS, key: F) -> AgentRef<A>
where
    A: Agent,
    A::IDX: Eq + Hash + Copy + Debug,
    T: Ord,
    AS: AgentSet<A>,
    F: FnMut(&AgentRef<A>) -> T,
{
    agents.min_one_of(key)
}
#[inline]
pub fn ask<A, AS, F>(agent_set: &mut AS, f: F)
where
    A: Agent,
    A::IDX: Eq + Hash + Copy + Debug,
    AS: AgentSet<A>,
    F: FnMut(&AgentRef<A>),
{
    agent_set.ask(f)
}
#[inline]
pub fn report<A, AS, T, F>(agent_set: AS, f: F) -> Vec<(A::IDX, T)>
where
    A: Agent,
    A::IDX: Eq + Hash + Copy + Debug,
    AS: AgentSet<A>,
    F: FnMut(&AgentRef<A>) -> T,
{
    agent_set.report(f)
}

pub fn concat<A, AS>(left: &AS, right: &AS) -> AS
where
    A: Agent,
    A::IDX: Eq + Hash + Copy + Debug,
    AS: AgentSet<A>,
{
    left.values()
        .chain(right.values())
        .map(|agent| {
            let who = agent.borrow().who();
            (who, agent.clone())
        })
        .collect()
}

pub fn random_float(rng: &Rng, min: f64, max: f64) -> f64 {
    rng.f64() * (max - min) + min
}
