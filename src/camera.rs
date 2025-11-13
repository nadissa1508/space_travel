// camera.rs - Defines a simple orbit camera for 3D navigation
#![allow(dead_code)]

use crate::matrix::create_view_matrix;
use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Camera {
    // Camera position/orientation
    pub eye: Vector3,    // Camera position
    pub target: Vector3, // Point the camera is looking at
    pub up: Vector3,     // Up vector

    // Orbit camera parameters
    pub yaw: f32,      // Rotation around Y axis (left/right)
    pub pitch: f32,    // Rotation around X axis (up/down)
    pub distance: f32, // Distance from target

    // Movement speed
    pub rotation_speed: f32,
    pub zoom_speed: f32,
    pub pan_speed: f32,

    //  warping
    pub warp_target: Option<Vector3>,
    pub warp_progress: f32,
    pub warp_duration: f32,
    pub warp_start_pos: Vector3,
}

impl Camera {
    pub fn new(eye: Vector3, target: Vector3, up: Vector3) -> Self {
        // Calculate initial yaw and pitch from eye and target
        let direction = Vector3::new(eye.x - target.x, eye.y - target.y, eye.z - target.z);

        let distance =
            (direction.x * direction.x + direction.y * direction.y + direction.z * direction.z)
                .sqrt();
        let pitch = (direction.y / distance).asin();
        let yaw = direction.z.atan2(direction.x);

        Camera {
            eye,
            target,
            up,
            yaw,
            pitch,
            distance,
            rotation_speed: 0.05,
            zoom_speed: 0.5,
            pan_speed: 0.1,
            warp_target: None,
            warp_progress: 0.0,
            warp_duration: 1.0,
            warp_start_pos: eye,
        }
    }

    /// Update camera eye position based on yaw, pitch, and distance
    fn update_eye_position(&mut self) {
        // Clamp pitch to avoid gimbal lock
        self.pitch = self.pitch.clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        // Calculate camera position using spherical coordinates
        // x = distance * cos(pitch) * cos(yaw)
        // y = distance * sin(pitch)
        // z = distance * cos(pitch) * sin(yaw)
        self.eye.x = self.target.x + self.distance * self.pitch.cos() * self.yaw.cos();
        self.eye.y = self.target.y + self.distance * self.pitch.sin();
        self.eye.z = self.target.z + self.distance * self.pitch.cos() * self.yaw.sin();
    }

    /// Get the view matrix for this camera
    pub fn get_view_matrix(&self) -> Matrix {
        create_view_matrix(self.eye, self.target, self.up)
    }

    /// Process keyboard input to control the camera
    pub fn process_input(&mut self, window: &RaylibHandle) {
        // Rotation controls (yaw)
        if window.is_key_down(KeyboardKey::KEY_A) {
            self.yaw += self.rotation_speed;
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_D) {
            self.yaw -= self.rotation_speed;
            self.update_eye_position();
        }

        // Rotation controls (pitch)
        if window.is_key_down(KeyboardKey::KEY_W) {
            self.pitch += self.rotation_speed;
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            self.pitch -= self.rotation_speed;
            self.update_eye_position();
        }

        // Zoom controls (distance from target) - arrow keys
        if window.is_key_down(KeyboardKey::KEY_UP) {
            self.distance -= self.zoom_speed;
            if self.distance < 0.5 {
                self.distance = 0.5; // Prevent camera from going too close
            }
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            self.distance += self.zoom_speed;
            self.update_eye_position();
        }

        // Pan controls (move target/center point)
        // Calculate right and forward vectors for panning
        let forward = Vector3::new(
            self.target.x - self.eye.x,
            0.0, // Keep on horizontal plane
            self.target.z - self.eye.z,
        );
        let forward_len = (forward.x * forward.x + forward.z * forward.z).sqrt();
        let forward_normalized = if forward_len > 0.0 {
            Vector3::new(forward.x / forward_len, 0.0, forward.z / forward_len)
        } else {
            Vector3::new(0.0, 0.0, 1.0)
        };

        let _right = Vector3::new(forward_normalized.z, 0.0, -forward_normalized.x);

        // NUEVO: Q/E para subir/bajar en Y
        if window.is_key_down(KeyboardKey::KEY_Q) {
            self.eye.y += 0.1; // Subir
            self.target.y += 0.1;
        }
        if window.is_key_down(KeyboardKey::KEY_E) {
            self.eye.y -= 0.1; // Bajar
            self.target.y -= 0.1;
        }

        // NUEVO: R/F para inclinar pitch
        if window.is_key_down(KeyboardKey::KEY_R) {
            self.pitch += 0.02;
        }
        if window.is_key_down(KeyboardKey::KEY_F) {
            self.pitch -= 0.02;
        }

        // Vertical panning
        if window.is_key_down(KeyboardKey::KEY_R) {
            self.target.y += self.pan_speed;
            self.update_eye_position();
        }
        if window.is_key_down(KeyboardKey::KEY_F) {
            self.target.y -= self.pan_speed;
            self.update_eye_position();
        }

         self.update_eye_position();
    }

    // Iniciar warp a una posición
    pub fn warp_to(&mut self, target_position: Vector3, duration: f32) {
        self.warp_start_pos = self.eye;
        self.warp_target = Some(target_position);
        self.warp_progress = 0.0;
        self.warp_duration = duration;
    }

    // Actualizar animación de warp
    pub fn update_warp(&mut self, delta_time: f32) {
        if let Some(target) = self.warp_target {
            self.warp_progress += delta_time / self.warp_duration;

            if self.warp_progress >= 1.0 {
                // Warp completado
                self.eye = target;
                self.warp_target = None;
                self.warp_progress = 0.0;
            } else {
                // Interpolación suave (ease-in-out)
                let t = Self::smoothstep(0.0, 1.0, self.warp_progress);
                self.eye = Self::lerp_vector3(self.warp_start_pos, target, t);
            }
        }
    }

    // Función de suavizado
    fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }

    // Interpolación lineal de vectores
    fn lerp_vector3(a: Vector3, b: Vector3, t: f32) -> Vector3 {
        Vector3::new(
            a.x + (b.x - a.x) * t,
            a.y + (b.y - a.y) * t,
            a.z + (b.z - a.z) * t,
        )
    }
}
