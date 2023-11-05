/*
 * sreng: simple rust engine 
 */

/*
 * TODO: 
 *
 * RENDERER 
 *  imgui implementation
 *
 * ENGINE 
 *  physics
 *  turn event_loop into lib.rs
 *
 */

mod event_loop;
mod application;
mod renderer;
mod events;

use crate::event_loop::run;

fn main() {
    pollster::block_on(run());
}

