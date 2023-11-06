use glfw::*;
use crate::{renderer::*, ui::Imgui}; 
use cgmath::*;

pub struct App {
    // todo: renderer, ui
    window: Window,
    glfw: Glfw,

    pub ui: Imgui,

    renderer: Renderer,
}

impl App {
    pub fn new(window: Window, mut glfw: Glfw, ui: Imgui) -> Self {
        let mut renderer = Renderer::new();

        // glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        // renderer.camera.set_projection(ProjectionType::Orthographic);
        
        renderer.push_mesh_with_shader(Shapes::grid(256), Shader::new_pipeline(DVS, MANDELBROT_FS));
        renderer.push_mesh(Shapes::grid(1000000).with_translation(vec3(0.0, 0.0, -30.0)));

        let cubes = {
            let mut verts = vec![];
            let mut inds = vec![];
            let mut num = 0;
            for x in 0..32 {
                for y in 0..32 {
                    for z in 0..32 {
                        let cube = Shapes::cube_raw(vec3(x as f32, y as f32, z as f32), num);
                        verts.extend_from_slice(&cube.0);
                        inds.extend_from_slice(&cube.1);

                        num += 1;
                    }
                }
            }
            Mesh::new(verts, inds)
        };
        renderer.push_mesh_with_shader(
            cubes.with_translation(vec3(64.0, 0.0, 0.0)),
            Shader::new_pipeline(VS2, FS2)
        );


        Self {
            window,
            glfw,
            ui,
            renderer,
        }
    }

    pub fn update(&mut self) {
        // let cube = &self.renderer.meshes[2].0;


        // self.renderer.get_mesh(0);
        self.renderer.update(&mut self.window, &mut self.glfw);
        // println!("{:.2}", 1.0 / dt);
        let frame = self.ui.frame(&mut self.window);
        frame.show_demo_window(&mut true);

        frame.text("hellO!");
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
