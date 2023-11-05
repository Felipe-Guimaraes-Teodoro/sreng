use gl::*;
use std::ffi::c_void;
use std::mem;
use std::ptr;
use gl::types::*;

#[derive(Debug)]
pub struct BufferConstruct {
    pub vbo: u32,
    pub ebo: u32,
    pub vao: u32,
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
            vao
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
}

// default vertex and fragment shaders
pub const DVS: &str = r#"
    #version 330
    layout (location = 0) in vec3 aPos;

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

#[macro_export]
macro_rules! cstr{
    ($s: expr) => {
        std::ffi::CString::new($s)
            .expect("conversion failed at cstr!() macro")
            .as_c_str()
    }
}
