use crate::renderer::Mesh;

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

}
