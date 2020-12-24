#![allow(non_snake_case)]
#![feature(trait_alias)]
mod Agent;
mod AgentSet;
mod AsSlice;
mod GetRng;
mod Link;
mod LinkRef;
mod LinkSet;
mod MapType;
mod Model;
mod Patch;
mod PatchRef;
mod PatchSet;
mod Position;
mod Toroidal;
mod Turtle;
mod TurtleRef;
mod TurtleSet;
mod Variable;
mod World;
mod common;
pub mod prelude;

#[macro_export]
macro_rules! init {
    ($turtle_count:literal, $max_pxcor:literal, $max_pycor:literal, $origin:ident, $is_periodic:literal) => {
        let w = World::init($turtle_count, $max_pxcor, $max_pycor, $origin, $is_periodic);
        let tsw = w.clone();
        let tw = w.clone();
        let psw = w.clone();
        let patches = move || w.borrow().patches();
        let turtles = move || tsw.borrow().turtles();
        let turtle = move |who: usize| tw.borrow().turtle(who);
        let patch = move |x, y| psw.borrow().patch(x, y);

        let rng = fastrand::Rng::new();
        let random = move |high: i64| rng.i64(0..high);
    };
}

#[macro_export]
macro_rules! model {
    (name = $model_name:ident,exporter = $data_name:ident, turtle_count = $turtle_count:literal, max_pxcor = $max_pxcor:literal, max_pycor = $max_pycor:literal, origin = $origin:ident, is_periodic = $is_periodic:literal, $(const $name:ident:$t:ty = $l:literal),* ,$setup:item, $go:item) => {
        mod $model_name{
        use serde::{Deserialize, Serialize};
        use std::cell::RefCell;
        use std::rc::Rc;
        use ruscape::prelude::*;
        use super::$data_name;

        $(const $name:$t = $l;)*

        pub struct RuscapeModel {
            world: Rc<RefCell<World>>,
        }
        impl RuscapeModel {
            pub fn new() -> Self {
                RuscapeModel {
                    world: World::init(
                        $turtle_count,
                        $max_pxcor,
                        $max_pycor,
                        $origin,
                        $is_periodic,
                    ),
                }
            }
            pub fn patches(&self) -> PatchSet {
                self.world.borrow().patches()
            }

            pub fn turtles(&self) -> TurtleSet {
                self.world.borrow().turtles()
            }
        }

        impl Model<$data_name> for RuscapeModel {
            $setup
            $go
        }
    }
    }
}
