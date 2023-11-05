use crate::renderer::Shader;
use crate::renderer::BufferConstruct;
use crate::cstr;
use cgmath::*;
use gl::*;

#[derive(Debug)]
pub struct Mesh {
    // shader: Shader,
    buffer_construct: BufferConstruct,

    vertices: Vec<f32>,
    indices: Vec<u32>,

    model: Matrix4<f32>,
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

            vertices,
            indices,

            model,
        }
    }

    pub fn with_translation(mut self, v: Vector3<f32>) -> Self {
        self.model = self.model * Matrix4::from_translation(v);

        self
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
        shader.use_shader();
        shader.uniform_mat4fv(
            cstr!("model"),
            &self.model
        );

        // draw the mesh
        BindVertexArray(self.buffer_construct.vao);
        DrawElements(
            TRIANGLES, 
            self.indices.len() as i32, 
            UNSIGNED_INT, 
            std::ptr::null()
        );
        BindVertexArray(0);
    }

    pub fn translate(&mut self, v: Vector3<f32>) {
        self.model = self.model * Matrix4::from_translation(v);
    }
}


