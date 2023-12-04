/*
 * sreng: simple rust engine 
 */

/*
 * TODO: 
 *
 * RENDERER 
 *  imgui implementation (keyboard input implementation)
 *
 * ENGINE 
 *  physics
 *  turn event_loop into lib.rs
 *
 */

mod event_loop;
mod application;
mod renderer;
mod physics;
mod events;
mod ui;
mod math;
mod world;


use crate::event_loop::run;

use std::sync::Mutex;
use core::cell::Cell;
use once_cell::sync::Lazy;
use threadpool::ThreadPool;

pub static GLOBAL_POOL: Lazy<ThreadPool> = Lazy::new(|| {
    ThreadPool::new(std::thread::available_parallelism().unwrap().get())
});

// lazy_static::lazy_static!{
//     pub static ref GLOBAL_MESHES: Mutex<Cell<Vec<crate::renderer::Mesh>>> = Mutex::new(Cell::new(vec![]));
// }
//
// impl std::ops::Deref for GLOBAL_MESHES {
//     type Target = Vec<crate::renderer::Mesh>; 
//     fn deref(&self) -> &mut Self::Target {
//         self
//     }
// }
//
// impl std::ops::DerefMut for GLOBAL_MESHES {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         self
//     }
// }

fn main() {
    pollster::block_on(run());

}
