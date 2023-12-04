use glfw::*;
use std::sync::mpsc::channel;
use crate::{renderer::*, ui::Imgui, physics::*, world}; 
use cgmath::*;
use rand::prelude::*;
use threadpool::ThreadPool;

use std::sync::{Mutex, Arc};
use once_cell::sync::Lazy;

// static TEST: Lazy<Mutex<Renderer>> = Lazy::new(|| Renderer::new().into() );

pub struct App {
    // todo: renderer, ui
    window: Window,
    glfw: Glfw,
    pub ui: Imgui,
    renderer: Renderer,

    controller: Controller,

    world: world::World,

    frametimes: Vec<f32>,
}

impl App {
    pub fn new(window: Window, mut glfw: Glfw, ui: Imgui) -> Self {
        let mut renderer = Renderer::new();

        // glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        // renderer.camera.set_projection(ProectionType::Orthographic);
        
        let controller = Controller::new();

        let mut world = world::World::new();

        // for x in 0..16 {
        //     GLOBAL_POOL.execute(move || {
        //         // push_global_mesh(world::create_chunk(x, 0));        
        //         // GLOBAL_MESHES.write().unwrap().push(world::create_chunk(x, 0));
        //         GLOBAL_MESHES
        //             .get_mut()
        //             .expect("fail")
        //             .get_mut()
        //             .push(world::create_chunk(x, 0));
        //     });
        // }
        // for i in 0..GLOBAL_MESHES.write().unwrap().len() {
            // renderer.push_mesh(GLOBAL_MESHES.write().unwrap()[i]);
        // }
        //
        
        for x in 0..2 {
            for y in 0..2 {
                renderer.push_mesh(world.create_chunk(x, y));
            }
        }

        Self {
            window,
            glfw,
            ui,
            renderer, 

            controller,

            world,

            frametimes: vec![],
        }
    }

    pub fn update(&mut self) {
        self.renderer.update(&mut self.window, &mut self.glfw);

        let frame = self.ui.frame(&mut self.window);

        self.frametimes.push( 1.0 / self.renderer.camera.dt );
        if self.frametimes.len() > 128 {
            self.frametimes.remove(0);
        }

        frame.text(format!(
                "framerate: {:.0}",
                self.frametimes.iter().sum::<f32>() / 128.0
            ));

        let t = frame.plot_lines("fps", &self.frametimes)
            .graph_size([150.0, 50.0]);
        t.build();

        let mut fdl = frame.get_foreground_draw_list();
        
        fdl.add_text([0.0, 0.0], (1.0, 1.0, 0.0), "Hello, world!");
        fdl.add_text([0.0, 12.0], (0.7, 0.7, 0.7), "build UNSTABLE v0.2");
        fdl.add_text([128.0, 12.0], (0.5, 0.5, 1.0), TUTORIAL_TXT);
        fdl.add_text([0.0, 36.0], (1.0, 0.2, 0.2), format!("AMM: {:?}", self.renderer.meshes.len()));

        if self.renderer.meshes.len() > 256 {
            self.renderer.meshes.remove(
                self.world.get_mesh_idx_far_away(
                    -self.renderer.camera.pos.z, 
                    -self.renderer.camera.pos.x,
                )
            );
            // self.world.make_far_away_chunk_inviz();
        }
        self.world.update(
            -self.renderer.camera.pos.z, 
            -self.renderer.camera.pos.x,
            &mut self.renderer,
            &mut fdl,
            frame.frame_count(),
        );

    }

    pub unsafe fn draw(&mut self) {
        self.renderer.draw();
        self.ui.draw();
    }

    pub fn window_mut(&mut self) -> &mut Window {
        &mut self.window
    }
    pub fn glfw_mut(&mut self) -> &mut Glfw {
        &mut self.glfw
    }

    pub fn mouse(&mut self, x: f32, y: f32,) {
        self.renderer.mouse(x, y, &mut self.window);
    }
}

const TUTORIAL_TXT: &str = "F1: Dots
F2: Lines
F3: Fill
";
