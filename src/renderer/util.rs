use gl::*;
use std::ffi::c_void;
use std::mem;
use std::ptr;
use gl::types::*;

use cgmath::*;

#[repr(C)]
pub struct UniformData {
    pub model: Matrix4<f32>,
}

impl UniformData {
    pub fn new_from_transl(v: Vector3<f32>) -> Self {
        Self {
            model: Matrix4::from_translation(v)
        }
    }

    pub fn new_from_mat4(mat4: Matrix4<f32>) -> Self {
        Self {
            model: mat4,
        }
    }
}

fn create_ubo(data: &Vec<UniformData>) -> GLuint {
    let mut ubo_id: GLuint = 0;

    unsafe {
        // Generate UBO ID
        GenBuffers(1, &mut ubo_id);

        // Bind the UBO
        BindBuffer(gl::UNIFORM_BUFFER, ubo_id);

        // Allocate storage for the UBO
        BufferData(
            gl::UNIFORM_BUFFER,
            (data.len() * std::mem::size_of::<UniformData>()) as isize,
            data.as_ptr() as *const GLvoid,
            gl::DYNAMIC_DRAW,
        );

        // // Set the UBO data
        // BufferSubData(
        //     gl::UNIFORM_BUFFER,
        //     0,
        //     std::mem::size_of::<UniformData>() as isize,
        //     data as *const _ as *const GLvoid,
        // );

        // Unbind the UBO
        BindBuffer(gl::UNIFORM_BUFFER, 0);
    }

    ubo_id
}

#[derive(Debug, Clone, Copy)]
pub struct BufferConstruct {
    pub vbo: u32,
    pub ebo: u32,
    pub vao: u32,
    pub ubo: Option<u32>,
}

impl BufferConstruct {
    pub unsafe fn new(vertices: &Vec<f32>) -> (u32, u32) {
        let (mut vbo, mut vao) = (0, 0);
        let vertex_ammount = (vertices.len() / 3) as i32;
        
        GenVertexArrays(1, &mut vao);
        GenBuffers(1, &mut vbo);

        // bind vertex array so we can do the vbo
        BindVertexArray(vao);

        BindBuffer(ARRAY_BUFFER, vbo);
        BufferData(
            ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            STATIC_DRAW
        );

        VertexAttribPointer(
            0, 
            3, 
            FLOAT, 
            FALSE, 
            3 * mem::size_of::<GLfloat>() as GLsizei, 
            ptr::null()
        );
        EnableVertexAttribArray(0);

        BindBuffer(ARRAY_BUFFER, 0);

        //unbind everything
        BindVertexArray(0);

        (vbo, vao)
    }

    pub unsafe fn new_indexed(
        vertices: &Vec<f32>, 
        indices: &Vec<u32>
    ) -> Self {
        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
        let vertex_ammount = (vertices.len() / 3) as usize;
        
        GenVertexArrays(1, &mut vao);
        GenBuffers(1, &mut vbo);
        GenBuffers(1, &mut ebo);

        // bind vertex array so we can do the vbo
        BindVertexArray(vao);

        BindBuffer(ARRAY_BUFFER, vbo);
        BufferData(
            ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const c_void,
            DYNAMIC_DRAW
        );

        BindBuffer(ELEMENT_ARRAY_BUFFER, ebo);
        BufferData(
            ELEMENT_ARRAY_BUFFER,
            (indices.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
            indices.as_ptr() as *const c_void,
            DYNAMIC_DRAW 
        );
        
        VertexAttribPointer(
            0, 
            3, 
            FLOAT, 
            FALSE, 
            (3 * mem::size_of::<GLfloat>()) as GLsizei, 
            ptr::null()
        );
        EnableVertexAttribArray(0);

        //unbind everything but the ebo
        BindBuffer(ARRAY_BUFFER, 0);
        BindVertexArray(0);

        Self {
            vbo,
            ebo, 
            vao,
            ubo: None,
        }
    }

    pub unsafe fn update_vertices(&mut self, vertices: &Vec<f32>) {
        BindVertexArray(self.vao);
        BindBuffer(ARRAY_BUFFER, self.vbo);

        BufferSubData(
            ARRAY_BUFFER, 
            0, 
            (vertices.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
            vertices.as_ptr() as *const c_void,
        );

        BindVertexArray(0);
        BindBuffer(ARRAY_BUFFER, 0);
    }

    pub unsafe fn update_ubo(&mut self, data: &Vec<UniformData>) {
        BindVertexArray(self.vao);
        BindBuffer(UNIFORM_BUFFER, self.ubo.unwrap());

        BufferSubData(
            UNIFORM_BUFFER,
            0,
            (data.len() * std::mem::size_of::<UniformData>()) as GLsizeiptr,
            data.as_ptr() as *const c_void,
        );

        BindVertexArray(0);
        BindBuffer(UNIFORM_BUFFER, 0);
    }


    pub unsafe fn new_instanced(
        vertices: &Vec<f32>, 
        indices: &Vec<u32>,
        instance_data: &Vec<UniformData>,
    ) -> Self {
        let ubo = create_ubo(instance_data);
        BindBufferBase(UNIFORM_BUFFER, 0, ubo);

        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
        let vertex_ammount = (vertices.len() / 3) as usize;
        
        GenVertexArrays(1, &mut vao);
        GenBuffers(1, &mut vbo);
        GenBuffers(1, &mut ebo);

        // bind vertex array so we can do the vbo
        BindVertexArray(vao);

        BindBuffer(ARRAY_BUFFER, vbo);
        BufferData(
            ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            vertices.as_ptr() as *const c_void,
            DYNAMIC_DRAW
        );

        BindBuffer(ELEMENT_ARRAY_BUFFER, ebo);
        BufferData(
            ELEMENT_ARRAY_BUFFER,
            (indices.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
            indices.as_ptr() as *const c_void,
            DYNAMIC_DRAW 
        );
        
        VertexAttribPointer(
            0, 
            3, 
            FLOAT, 
            FALSE, 
            (3 * mem::size_of::<GLfloat>()) as GLsizei, 
            ptr::null()
        );
        EnableVertexAttribArray(0);

        //unbind everything but the ebo
        BindBuffer(ARRAY_BUFFER, 0);
        BindVertexArray(0);




        Self {
            vbo,
            ebo, 
            vao,
            ubo: Some(ubo),
        }
    }
}

// default vertex and fragment shaders
pub const DVS: &str = r#"
    #version 330
    layout (location = 0) in vec3 aPos;
    // layout (location = 3) in mat4 instanceMatrix;

    uniform mat4 proj; 
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        gl_Position = proj * view * model * vec4(aPos, 1.0);
    }
"#;

pub const DFS: &str = r#"
    #version 330

    out vec4 frag_color;

    void main() {
        frag_color = vec4(0.67, 0.78, 0.81, 1.0);
    }
"#;

pub const MANDELBROT_FS: &str = r#"
        #version 330

        out vec4 frag_color;

        uniform mat4 proj; 
        uniform mat4 view;
        uniform mat4 model;

        vec2 complex_mul(vec2 a, vec2 b) { 
            return vec2(a.x*b.x - a.y*b.y, a.x*b.y + a.y*b.x); 
        }

        void main() {
            vec2 coord = (2.*gl_FragCoord.xy - vec2(1000.0, 1000.0))/1000.0;
            vec2 z = vec2(0,0);
            vec2 c = coord - vec2(0.5, 0);
            bool in_set = true;

            for (int i = 0; i < 256; i++) {
                z =  complex_mul(z, z) + c;
                if (length(z) > 2.) {
                    in_set = false;
                    break;
                }
            }

            if (in_set) {
                frag_color = vec4(1.0 * sin(gl_FragCoord.y / 10.0), 0.0, 0.0, 1.0);
            } else {
                frag_color = vec4(0.0, 0.0, 1.0, 1.0);
            }
        }
"#;

pub const VS2: &str = r#"
    #version 330
    layout (location = 0) in vec3 aPos;

    uniform mat4 proj; 
    uniform mat4 view;
    uniform mat4 model;
    
    out vec3 vert_pos;

    void main() {
        gl_Position = proj * view * model * vec4(aPos, 1.0);
        vert_pos = aPos;
    }
"#;

pub const FS2: &str = r#"
    #version 330

    in vec3 vert_pos;
    out vec4 frag_color;

    void main() {
        frag_color = vec4(vert_pos, 1.0);
    }
"#;

pub const VFS: &str = r#"
   #version 330
    layout (location = 0) in vec3 aPos;

    uniform mat4 proj; 
    uniform mat4 view;
    uniform mat4 model;
    
    out vec3 vert_pos;

    void main() {
        gl_Position = vec4(aPos, 1.0);
        vert_pos = aPos;
    }

"#;

pub const FS3: &str = r#"
    #version 330

    in vec3 vert_pos;
    out vec4 frag_color;

    uniform mat4 proj; 
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        vec2 frag_coord = vec2(vert_pos.xy);
        for (float i = 0.0; i < 32.0; i+=1.0) {
            frag_coord = abs(frag_coord);
            frag_coord -= 0.5;
            frag_coord *= 1.1;
            frag_coord *= mat2(
                cos(0.2), -sin(0.2),
                sin(0.2), cos(0.2)
            );
        }
        frag_color = vec4(vec3(length(frag_coord)), 1.0);
    }
"#;

pub const DVS_I: &str = r#"
    #version 330
    layout (location = 0) in vec3 aPos;

    layout (std140) uniform InstanceData {
        mat4 model[1024];
    };

    uniform mat4 proj; 
    uniform mat4 view;

    out vec3 vert_pos;
    out float instance_id;
    
    // uniform mat4 models;

    void main() {
        gl_Position = proj * view * model[gl_InstanceID] * vec4(aPos, 1.0);
        vert_pos = aPos;
        instance_id = gl_InstanceID;
    }
"#;

pub const DFS_I: &str = r#"
    #version 330

    out vec4 frag_color;

    in vec3 vert_pos;
    in float instance_id;

    void main() {
    frag_color = vec4(1.0);
    }
"#;

#[macro_export]
macro_rules! cstr{
    ($s: expr) => {
        std::ffi::CString::new($s)
            .expect("conversion failed at cstr!() macro")
            .as_c_str()
    }
}
