use fastrand::*;

pub trait GetRng {
    fn get_rng(&self) -> Rng;
}
