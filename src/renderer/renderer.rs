use crate::renderer::*;


use glfw::{Glfw, Window};
use gl::*;

pub struct Renderer {
    pub shaders: Vec<Shader>,
    pub meshes: Vec<(Mesh, usize)>,
    pub camera: Camera,
}

impl Renderer {
    pub fn new() -> Self {
        let meshes = vec![];
        let mut shaders = vec![];
        let camera = Camera::new();

        // use the default shader
        shaders.push(
            Shader::new_pipeline(&DVS, &DFS)
        );
        unsafe {
            shaders[0].use_shader();
        }

        Self {
            meshes,
            shaders,
            camera,
        }
    }

    pub unsafe fn draw(&mut self) {
        Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        Enable(DEPTH_TEST);
        PointSize(2.0);
        ClearColor(0.03, 0.09, 0.16, 1.0);

        for (mesh, shader_idx) in self.meshes.iter_mut() {
            self.camera.send_uniforms(&self.shaders[*shader_idx]);
            mesh.draw(&self.shaders[*shader_idx]);
        }
        // self.camera.send_uniforms(&self.shaders[0]);
    }

    pub fn update(&mut self, window: &mut Window, glfw: &mut Glfw) {
        self.camera.update();
        self.camera.input(window, glfw);
    }

    pub fn mouse(&mut self, x: f32, y: f32, window: &mut Window) {
        self.camera.mouse_callback(x, y, window);
    }

    pub fn push_mesh(&mut self, mesh: Mesh) {
        //use the default shader index for rendering this mesh
        self.meshes.push((mesh, 0));
    }

    pub fn push_mesh_with_shader(&mut self, mesh: Mesh, shader: Shader) {
        self.shaders.push(shader);
        let shader_idx = self.shaders.len() - 1; // account for the default shader
        unsafe {
            self.shaders[shader_idx].use_shader()
        }
        self.meshes.push((mesh, shader_idx));
    }

    pub fn _push_shader(&mut self, shader: Shader) {
        self.shaders.push(shader);
    }

    pub fn get_mesh(&mut self, idx: usize) -> &mut Mesh {
        &mut self.meshes[idx].0
    }
}
