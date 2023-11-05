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
mod events;
mod ui;


use crate::event_loop::run;

fn main() {
    pollster::block_on(run());
}

