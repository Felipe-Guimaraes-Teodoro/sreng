use crate::math::Math;
use threadpool::ThreadPool;
//thread pool for computing world data 

lazy_static::lazy_static!{
    static ref WORLD_POOL: ThreadPool = 
        ThreadPool::new(std::thread::available_parallelism().unwrap().get());
}

#[derive(Clone, Copy, Debug)]
pub struct Voxel {
    pos: (usize, usize, usize),
    pub adjacent_voxs: [bool; 6],
        /*
         * 0: front,
         * 1: back,
         * 2: bottom,
         * 3: top,
         * 4: left,
         * 5: right
         */

    pub is_air: bool,
}

impl Voxel {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self {
            pos: (x, y, z),
            adjacent_voxs: [false; 6], 
            is_air: false,
        }
    }

    pub fn air(x: usize, y: usize, z: usize) -> Self {
        Self {
            pos: (x, y, z),
            adjacent_voxs: [false; 6],
            is_air: true,
        }
    }

    pub fn gen_vertices(&self) -> Vec<f32> {
        let mut verts: Vec<f32> = vec![];

        for &dx in &[0, 1] {
            for &dy in &[0, 1] {
                for &dz in &[0, 1] {
                    let x = (self.pos.0 + dx) as f32;
                    let y = (self.pos.1 + dy) as f32;
                    let z = (self.pos.2 + dz) as f32;

                    verts.push(x);
                    verts.push(y);
                    verts.push(z);
                }
            }
        }
        verts
    }

    pub fn gen_indices(&self) -> Vec<u32> {
        let mut indices: Vec<u32> = vec![];
        let base_index = (self.pos.0 * SIZE * SIZE + self.pos.1 * SIZE + self.pos.2) as u32 * 8;

        let idx = [ 
            [0, 1, 2, 1, 2, 3], // front
            [4, 5, 6, 5, 6, 7], // back
            [0, 1, 4, 1, 4, 5], // bottom
            [2, 3, 6, 3, 6, 7], // top
            [0, 2, 4, 2, 4, 6], // left
            [1, 3, 5, 3, 5, 7], // right
        ];

        let idx_vec = idx.to_vec();

        // for &face in &[
        //     [0, 1, 2, 1, 2, 3], // front
        //     [4, 5, 6, 5, 6, 7], // back
        //     [0, 1, 4, 1, 4, 5], // bottom
        //     [2, 3, 6, 3, 6, 7], // top
        //     [0, 2, 4, 2, 4, 6], // left
        //     [1, 3, 5, 3, 5, 7], // right
        // ] {
        //     indices.extend(face.iter().map(|&i| base_index + i));
        // }
        for i in 0..idx_vec.len() {
            let face = idx_vec[i];
            if self.adjacent_voxs[i] == false {
                indices.extend(face.iter().map(|&j| base_index + j));
            }
        }

        indices
    }
}

const SIZE: usize = 16;
const SIZE_Y: usize = 64;
#[derive(Clone, Debug)]
pub struct Chunk {
    pub pos: (usize, usize),
    visible: bool,
    renderer_index: usize,
    data: Vec<Vec<Vec<Voxel>>>,
}

impl Chunk {
    pub fn new(x: usize, y: usize) -> Self {
        let mut arr: Vec<Vec<Vec<Voxel>>> = 
            vec![vec![vec![Voxel::air(0,0,0); SIZE]; SIZE_Y]; SIZE];

        for x in 0..SIZE {
            for y in 0..SIZE_Y {
                for z in 0..SIZE {
                    if Math::random(0.0, 1.0) > 0.1 {
                        arr[x][y][z] = Voxel::new(x, y, z);
                        arr[x][y][z].is_air = true;
                    } else {
                        arr[x][y][z] = Voxel::new(x,y,z);
                    }
                }
            }
        }

        Self {
            pos: (x, y),
            visible: true,
            renderer_index: 0,
            data: arr, 
        }
    }

    pub fn empty() -> Self {
        Self {
            pos: (0, 0),
            visible: false,
            renderer_index: 0,
            data: vec![],
        }
    }

    pub fn gen_mesh(self) -> (Vec<f32>, Vec<u32>) {
        let (tx, rx) = std::sync::mpsc::channel();

        WORLD_POOL.execute(move || {
            let mut vertices = vec![];
            let mut indices = vec![];

            for x in 0..SIZE {
                for y in 0..SIZE_Y {
                    for z in 0..SIZE {
                        if !self.data[x][y][z].is_air {
                            let voxel = self.data[x][y][z];
                            vertices.extend(voxel.gen_vertices());
                            indices.extend(voxel.gen_indices());
                        }
                    }
                }
            }

            let data = (vertices, indices);
            tx.send(data).expect("thread pool will be waiting for channel");
        }); 

        rx.recv().unwrap()
    }
}

use cgmath::vec3;
use crate::{renderer::Renderer, renderer::Mesh};

pub fn create_chunk(x: usize, y: usize) -> (Chunk, Mesh) {
    let chunk = &Chunk::new(x, y);
    let chunk_mesh_data = chunk.clone().gen_mesh();
    (chunk.clone(), Mesh::new(chunk_mesh_data.0, chunk_mesh_data.1).with_translation(vec3((chunk.pos.0 * SIZE) as f32, 0.0, (chunk.pos.1 * SIZE) as f32)),
    )
}

pub struct World {
    chunks: Vec<Vec<Chunk>>,
}

use imgui::DrawListMut;
impl World {
    pub fn new() -> Self {
        Self {
            chunks: vec![vec![Chunk::empty(); 2000]; 2000],
        }
    }

    pub fn create_chunk(&mut self, x: usize, y: usize) -> Mesh {
        let (chunk, mesh) = create_chunk(x, y);

        self.chunks[x][y] = chunk;

        mesh
    }

    pub fn get_chunk_mut(&mut self, x: usize, y: usize) -> Option<&mut Chunk> {
        Some(&mut self.chunks[x][y])
    }
    pub fn get_chunk(&mut self, x: usize, y: usize) -> Option<&Chunk> {
        Some(&self.chunks[x][y])
    }

    pub fn update(
        &mut self, 
        x: f32, z: f32, 
        renderer: &mut Renderer, 
        ui: &mut DrawListMut, 
        frame_idx: i32
    ) {
        let rel_x = (x / SIZE as f32) as usize;
        let rel_z = (-z / SIZE as f32) as usize;
        ui.add_text(
            [128.0, 0.0],
            (0.8, 1.0, 0.6),
            format!("RELATIVE CHUNK POSITION: x={:?}, z={:?}", rel_x, rel_z),
        );
        //create a single chunk per frame  
        let frame_chunk_id = (frame_idx % 16 * 16) as usize;

        ui.add_text(
            [512.0, 0.0],
            (0.7, 0.7, 0.7),
            format!("FRAME_CHUNK_ID: {:?}", frame_chunk_id)
        );

        ui.add_text(
            [512.0, 12.0],
            (0.7, 0.7, 0.7),
            format!("CIND: {:?}", frame_chunk_id / 16)
        );

        for i in 0..16 {
            for j in 0..16 {
                let rx = rel_x + i;
                let rz = rel_z + j;

                if !self.get_chunk(rx, rz).unwrap().visible {
                    let (chunk, mesh) = create_chunk(rx, rz);
                    self.chunks[rx][rz] = chunk.clone();
                    renderer.push_mesh(
                        mesh.with_translation(vec3(-((rx) as f32), 0.0, -((rz) as f32)))
                    );
                    self.chunks[rx][rz].renderer_index = renderer.meshes.len() - 1;

                    return;
                }
            }
        }
    } // update 

    pub fn get_mesh_idx_far_away(&mut self, x: f32, z: f32) -> usize {
        let rel_x = (x / SIZE as f32) as usize;
        let rel_z = (-z / SIZE as f32) as usize;

        for i in 0..32 {
            for j in 0..32 {
                let rx = rel_x + i;
                let rz = rel_x + i;

                if (i > 16) && (j > 16) {
                    let c = self.get_chunk(rx,rz).unwrap();
                    if c.visible {
                        return c.renderer_index 
                    }
                }
            }
        }

        return 0;
        
    }

}






