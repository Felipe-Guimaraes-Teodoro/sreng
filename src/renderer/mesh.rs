use crate::renderer::Shader;
use crate::renderer::util::{DVS, DFS};
use crate::renderer::{BufferConstruct, UniformData};
use crate::cstr;
use cgmath::*;
use gl::*;

use lazy_static::lazy_static;
use once_cell::sync::Lazy;

lazy_static!{
    static ref DEFAULT_SHADER: Shader = Shader::new_pipeline(DVS, DFS);
}

#[derive(Debug, Copy, Clone)]
pub struct Mesh {
    // shader: Shader,
    buffer_construct: BufferConstruct,

    // vertices: Vec<f32>,
    len: usize,

    pub model: Matrix4<f32>,
    pub count: u32,

    is_instance: bool,
    can_render: bool,
}

impl Mesh {
    /*
     *  initializer functions
     */
    pub fn new(vertices: Vec<f32>, indices: Vec<u32>) -> Self {
        let buffer_construct = unsafe {
            BufferConstruct::new_indexed(&vertices, &indices)
        };

        // let model = Matrix4::from_translation(vec3(0.0, 0.0, 0.0));
        let model = 
            Matrix4::from_angle_y(Deg(90.0)) 
            * Matrix4::from_translation(vec3(0.0, 0.0, 2.0));

        Self {
            // shader,
            buffer_construct,

            // vertices,
            len: indices.len(),

            model,
            count: 1,

            is_instance: false,
            can_render: true,
        }
    }

    pub fn new_instanced(
        vertices: Vec<f32>, 
        indices: Vec<u32>, 
        instance_data: Vec<UniformData>,
        count: u32
    ) -> Self {
        let buffer_construct = unsafe {
            BufferConstruct::new_instanced(&vertices, &indices, &instance_data)
        };

        // let model = Matrix4::from_translation(vec3(0.0, 0.0, 0.0));
        let model = 
            Matrix4::from_angle_y(Deg(90.0)) 
            * Matrix4::from_translation(vec3(0.0, 0.0, 2.0));

        Self {
            // shader,
            buffer_construct,

            // vertices,
            len: indices.len(),

            model,
            count,

            is_instance: true,
            can_render: true,
        }
    }

    pub fn new_with_shader(vertices: Vec<f32>, indices: Vec<u32>) -> Self {
        let buffer_construct = unsafe {
            BufferConstruct::new_indexed(&vertices, &indices)
        };

        // let model = Matrix4::from_translation(vec3(0.0, 0.0, 0.0));
        let model = 
            Matrix4::from_angle_y(Deg(90.0)) 
            * Matrix4::from_translation(vec3(0.0, 0.0, 2.0));

        Self {
            // shader,
            buffer_construct,

            // vertices,
            len: indices.len(),

            model,
            count: 1,

            is_instance: false,
            can_render: true,
        }
    }

    pub fn with_translation(mut self, v: Vector3<f32>) -> Self {
        let model = self.model;
        self.model = model * Matrix4::from_translation(v);

        self
    }

    pub fn set_translation(&mut self, v: Vector3<f32>) {
        self.model = Matrix4::from_translation(v);
    }

    pub fn rect(x: f32, y:f32) -> Self {
        let (vertices, indices) = (
            vec![
                x, -y, 0.0,
                x, y, 0.0,
                -x, y, 0.0,
                -x, -y, 0.0

            ],
            vec![0, 1, 2, 0, 2, 3],
        );

        Self::new(vertices, indices)
    }

    /*
     *  INSTANT FUNCTIONS
     */

    pub unsafe fn draw(&mut self, shader: &Shader) {
        // send uniforms to the shader (assuming its the default shader)
        // todo!(): better way to do this ^^ 
        if self.is_instance == true { // its an instanced mesh
            shader.use_shader();
            // ubo alias: instance buffer
            // draw the mesh
            BindVertexArray(self.buffer_construct.vao);
            BindBufferBase(UNIFORM_BUFFER, 0, self.buffer_construct.ubo
               .expect("failed to bind ubo")); // bind ubo
            DrawElementsInstanced(
                TRIANGLES, 
                self.len as i32, 
                UNSIGNED_INT,
                std::ptr::null(),
                self.count as i32,
            );
            BindBufferBase(UNIFORM_BUFFER, 0, 0); //unbind ubo
            BindVertexArray(0); // unbind vao
        } else {
            shader.use_shader();
            shader.uniform_mat4fv(
                cstr!("model"),
                &self.model
            );

            // draw the mesh
            BindVertexArray(self.buffer_construct.vao);
            DrawElements(
                TRIANGLES, 
                self.len as i32, 
                UNSIGNED_INT, 
                std::ptr::null()
            );
            BindVertexArray(0);
        }
    }

    pub fn update_instance_buf(&mut self, new_instance_buffer: &Vec<UniformData>) {
        unsafe {
            self.buffer_construct.update_ubo(new_instance_buffer);
        }
    }

    pub unsafe fn draw_default_shader(&mut self) {
        self.draw(&DEFAULT_SHADER);
    }

    pub fn translate(&mut self, v: Vector3<f32>) {
        self.model = self.model * Matrix4::from_translation(v);
    }
}


