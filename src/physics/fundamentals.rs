use cgmath::*;
use crate::renderer::*;
use glfw::{Key, Action};

/*
 * switch renderer for mesh in the draw calls of the PPoint struct 
 * draw using instances
 */

pub const bounce: f32 = 0.0;

pub struct PPoint {
    pub pos: Vector3<f32>,
    old_pos: Vector3<f32>, 
    pub velocity: Vector3<f32>,

    radius: f32,
    pub friction: f32,

    pub on_ground: bool,
}

impl PPoint {
    pub fn new(
        pos: Vector3<f32>,
        old_pos: Vector3<f32>, // initial force
        radius: f32,
        friction: f32,
    ) -> Self {
        let velocity = pos - old_pos;

        Self {
            pos,
            old_pos,
            velocity,

            radius,
            friction,
            on_ground: false,
        }
    }

    pub fn update(&mut self) {
        self.velocity = self.pos - self.old_pos;

        self.old_pos = self.pos;

        self.velocity.y -= 0.000001;
        self.constrain(100.0, 100.0, 100.0);

        self.pos += self.velocity; // * renderer.camera.dt;
    }

    pub fn constrain(&mut self, ground: f32, width: f32, depth: f32,) { 
        let ground = (ground * 2.0) - ground; 
        let width = (width * 2.0) - width;
        let depth = (depth * 2.0) - depth;


        // if self.pos.x - self.radius < 0.0 {
        //     self.pos.x = self.radius;
        //     self.old_pos.x = self.pos.x + self.velocity.x * bounce;
        // }

        // if self.pos.x + self.radius > width {
        //     self.pos.x = width - self.radius;
        //     self.old_pos.x = self.pos.x + self.velocity.x * bounce;
        // }
        //
        if self.pos.y - self.radius <= 0.0 {
            self.pos.y = self.radius;
            self.old_pos.y = self.pos.y + self.velocity.y * bounce;

            self.velocity.x *= 0.995; // friction
            self.velocity.z *= 0.995; // friction

            self.on_ground = true;
        } else {
            self.on_ground = false;
            self.velocity.x *= 0.999; // friction
            self.velocity.z *= 0.999; // friction
        }
        self.velocity.x *= 0.99; // friction
        self.velocity.z *= 0.99; // friction

        // if self.pos.y + self.radius > ground {
        //     self.pos.y = ground - self.radius;
        //     self.old_pos.y = self.pos.y + self.velocity.y * bounce;
        // }       

        // if self.pos.z - self.radius < 0.0 {
        //     self.pos.z = self.radius;
        //     self.old_pos.z = self.pos.z + self.velocity.z * bounce;
        // }

        // if self.pos.z + self.radius > depth {
        //     self.pos.z = depth - self.radius;
        //     self.old_pos.z = self.pos.z + self.velocity.z * bounce;
        // }

    }
}


pub struct Controller {
    ppoint: PPoint,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            ppoint: PPoint::new(
                        vec3(1.0, 1.0, 1.0),
                        vec3(1.0, 1.0, 1.0),
                        2.0,
                        1.0
                    ),
        }
    }

    pub fn update(&mut self) {
        self.ppoint.update();
    }

    pub fn input(
        &mut self, 
        window: &mut glfw::Window, 
        cam: &mut Camera,
    ) {
        cam.pos = self.ppoint.pos;
        let mut speed = 0.04;

        if window.get_key(Key::LeftShift) == Action::Press {
            speed *= 5.0;
        }

        let front = vec3(cam.front.x, 0.0, cam.front.z);

        if window.get_key(Key::W) == Action::Press {
            self.ppoint.pos += speed * cam.dt * front; 
        }
        if window.get_key(Key::S) == Action::Press {
            self.ppoint.pos -= speed * cam.dt * front; 
        }
        if window.get_key(Key::Space) == Action::Press {
            if self.ppoint.on_ground {
                // self.ppoint.pos += speed * cam.dt * cam.up;
                self.ppoint.pos.y += 0.006;
            }
        }
        if window.get_key(Key::LeftControl) == Action::Press {
            self.ppoint.pos -= speed * cam.dt * cam.up;
        }
        if window.get_key(Key::A) == Action::Press {
            self.ppoint.pos -= speed * cam.dt * Vector3::cross(cam.front, cam.up); 
        }
        if window.get_key(Key::D) == Action::Press {
            self.ppoint.pos += speed * cam.dt * Vector3::cross(cam.front, cam.up); 
        }
        
    }
}

