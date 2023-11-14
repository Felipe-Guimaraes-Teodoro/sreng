use glfw::*;
use crate::{renderer::*, ui::Imgui, physics::*}; 
use cgmath::*;
use rand::prelude::*;

pub struct App {
    // todo: renderer, ui
    window: Window,
    glfw: Glfw,
    pub ui: Imgui,
    renderer: Renderer,

    obj: Vec<PPoint>,
    controller: Controller,
}

fn rand_vec3() -> Vector3<f32> {
    let mut g = rand::thread_rng();
    let x = g.gen_range(-10.0..=10.0);
    let y = g.gen_range(0.0..=900.0);
    let z = g.gen_range(-10.0..=10.0);

    vec3(x, y, z)
}

impl App {
    pub fn new(window: Window, mut glfw: Glfw, ui: Imgui) -> Self {
        let mut renderer = Renderer::new();

        // glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        // renderer.camera.set_projection(ProjectionType::Orthographic);
        
        renderer.push_mesh(
            Shapes::floor(500.0, 500.0),
        );

        let mut obj = vec![];
        for i in 0..1 {
            obj.push(
                PPoint::new(
                    rand_vec3(),
                    rand_vec3(),
                    1.0,
                    1.0,
                )
            );
            let mut cube = Shapes::cube(vec3(0.0, 0.99, 0.0));
            cube.model += Matrix4::from_scale(100.0);
            renderer.push_mesh_with_shader(
                cube,
                Shader::new_pipeline(VS2, FS2)
            );
        }

        let controller = Controller::new();

        Self {
            window,
            glfw,
            ui,
            renderer,

            obj,
            controller,
        }
    }

    pub fn update(&mut self) {
        // let cube = &self.renderer.meshes[2].0;

        // self.renderer.get_mesh(0);
        self.renderer.update(&mut self.window, &mut self.glfw);
        
        // println!("{:.2}", 1.0 / dt);
        let frame = self.ui.frame(&mut self.window);

        frame.text("Hello, world!");

        // for i in 0..self.obj.len() {
        //     self.obj[i].update();
        //     self.renderer.meshes[i+1].0.set_translation(
        //         self.obj[i].pos
        //     );
        // }

        self.controller.input(&mut self.window, &mut self.renderer.camera);
        self.controller.update();
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
