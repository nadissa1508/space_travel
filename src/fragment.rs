// fragment.rs
#![allow(dead_code)]

use raylib::math::{Vector2, Vector3};

pub struct Fragment {
    pub position: Vector2,       // Screen-space position
    pub color: Vector3,           // Interpolated color
    pub depth: f32,               // Interpolated depth
    pub world_position: Vector3,  // Interpolated world-space position
    pub normal: Vector3,          // NUEVO: normal interpolada
}

impl Fragment {
    pub fn new(x: f32, y: f32, color: Vector3, depth: f32) -> Self {
        Fragment {
            position: Vector2::new(x, y),
            color,
            depth,
            world_position: Vector3::zero(),
            normal: Vector3::zero(),  // AÑADIR
        }
    }

    pub fn new_with_world_pos(x: f32, y: f32, color: Vector3, depth: f32, world_pos: Vector3) -> Self {
        Fragment {
            position: Vector2::new(x, y),
            color,
            depth,
            world_position: world_pos,
            normal: Vector3::zero(),  // AÑADIR
        }
    }

    // NUEVO: Constructor completo con normal
    pub fn new_complete(x: f32, y: f32, color: Vector3, depth: f32, world_pos: Vector3, normal: Vector3) -> Self {
        Fragment {
            position: Vector2::new(x, y),
            color,
            depth,
            world_position: world_pos,
            normal,
        }
    }
}