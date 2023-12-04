use crate::renderer::Mesh;
use cgmath::*;

pub struct Shapes { }

impl Shapes {
    pub fn grid(num: i32) -> Mesh {
        let rects_per_row = f32::sqrt(num as f32) as i32;
        let rects_per_col = (num - 1) / rects_per_row + 1;
        let spacing = 1.0;

        let mut vertices = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
    
        for i in 0..num {
            let x = ((i % rects_per_row - rects_per_row) as f32 / 2.0 + 0.5) * spacing;
            let y = ((i / rects_per_row - rects_per_col) as f32 / 2.0 + 0.5) * spacing;

            let spacing = spacing / 4.0;
            let quad_vertices = vec![
                spacing + x, -spacing + y, 0.0,
                spacing + x, spacing + y, 0.0,
                -spacing + x, spacing + y, 0.0,
                -spacing + x, -spacing + y, 0.0,
            ];

            let i = i as u32;
            let quad_indices: Vec<u32> = 
                vec![0 + 4*i, 1 + 4*i, 2 + 4*i, 0 + 4*i, 2 + 4*i, 3 + 4*i]; 

            vertices.extend_from_slice(&quad_vertices);
            indices.extend_from_slice(&quad_indices);
        }

        Mesh::new(vertices, indices)
    }

    pub fn fullscreen_quad() -> (Vec<f32>, Vec<u32>) {
        (
            vec![
                -1.0, -1.0, 0.0,
                -1.0, 1.0, 0.0,
                1.0, -1.0, 0.0,
                1.0, 1.0, 0.0,
            ],

            vec![0, 1, 2, 2, 1, 3]
        )
    }

    pub fn cube_raw(p: Vector3<f32>, i: u32) -> (Vec<f32>, Vec<u32>) {
        (vec![
            -1.0 + p.x, -1.0 + p.y,  1.0 + p.z, //0
             1.0 + p.x, -1.0 + p.y,  1.0 + p.z, //1
            -1.0 + p.x,  1.0 + p.y,  1.0 + p.z, //2
             1.0 + p.x,  1.0 + p.y,  1.0 + p.z, //3
            -1.0 + p.x, -1.0 + p.y, -1.0 + p.z, //4
             1.0 + p.x, -1.0 + p.y, -1.0 + p.z, //5
            -1.0 + p.x,  1.0 + p.y, -1.0 + p.z, //6
             1.0 + p.x,  1.0 + p.y, -1.0 + p.z  //7
        ],
        
        vec![
            2 + 8 * i, 6 + 8 * i, 7 + 8 * i, // top
            2 + 8 * i, 3 + 8 * i, 7 + 8 * i,
            0 + 8 * i, 4 + 8 * i, 5 + 8 * i, // bottom
            0 + 8 * i, 1 + 8 * i, 5 + 8 * i,
            0 + 8 * i, 2 + 8 * i, 6 + 8 * i, // left
            0 + 8 * i, 4 + 8 * i, 6 + 8 * i,
            1 + 8 * i, 3 + 8 * i, 7 + 8 * i, // right
            1 + 8 * i, 5 + 8 * i, 7 + 8 * i,
            0 + 8 * i, 2 + 8 * i, 3 + 8 * i, // front
            0 + 8 * i, 1 + 8 * i, 3 + 8 * i,
            4 + 8 * i, 6 + 8 * i, 7 + 8 * i, // back
            4 + 8 * i, 5 + 8 * i, 7 + 8 * i
        ],)
    }

    pub fn floor(x: f32, y: f32) -> Mesh {
        let mut mesh = Mesh::rect(x, y);
        mesh.model = Matrix4::from_angle_x(Deg(90.0));

        mesh
    }

    pub fn rect_raw(tl: [f32; 2], br: [f32; 2]) -> (Vec<f32>, Vec<u32>) {
        let verts: Vec<f32> = vec![
            tl[0], tl[1], 0.0,
            br[0], tl[1], 0.0,
            br[0], br[1], 0.0,
            tl[0], br[1], 0.0,
        ];
        let inds: Vec<u32> = vec![
            0, 1, 2,
            0, 2, 3
        ];

        (verts, inds)
    }

    pub fn cube(v: Vector3<f32>) -> Mesh {
        let cube_raw = Self::cube_raw(v, 0);

        Mesh::new(cube_raw.0, cube_raw.1)
    }
}
