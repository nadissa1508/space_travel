use crate::math::{Vec3, Mat4};

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
    
    // Ángulo de rotación alrededor del eje Y (para movimiento en plano eclíptico)
    pub yaw: f32,
    pub distance_from_target: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            position: Vec3::new(0.0, 5.0, 20.0),
            target: Vec3::zero(),
            up: Vec3::new(0.0, 1.0, 0.0),
            fov: std::f32::consts::PI / 4.0, // 45 grados
            aspect,
            near: 0.1,
            far: 1000.0,
            yaw: 0.0,
            distance_from_target: 20.0,
        }
    }

    /// Obtiene la matriz de vista
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at(self.position, self.target, self.up)
    }

    /// Obtiene la matriz de proyección
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective(self.fov, self.aspect, self.near, self.far)
    }

    /// Matriz combinada view * projection
    pub fn view_projection_matrix(&self) -> Mat4 {
        let view = self.view_matrix();
        let proj = self.projection_matrix();
        proj.multiply(&view)
    }

    /// Mueve la cámara hacia adelante/atrás en el plano eclíptico
    pub fn move_forward(&mut self, amount: f32) {
        let direction = (self.target - self.position).normalize();
        // Solo movimiento en XZ (plano eclíptico)
        let movement = Vec3::new(direction.x, 0.0, direction.z).normalize().scale(amount);
        self.position = self.position + movement;
        self.target = self.target + movement;
    }

    /// Mueve la cámara lateralmente
    pub fn move_right(&mut self, amount: f32) {
        let forward = (self.target - self.position).normalize();
        let right = forward.cross(&self.up).normalize();
        let movement = Vec3::new(right.x, 0.0, right.z).normalize().scale(amount);
        self.position = self.position + movement;
        self.target = self.target + movement;
    }

    /// Rota la cámara (cambia hacia dónde mira)
    pub fn rotate(&mut self, delta_yaw: f32) {
        self.yaw += delta_yaw;
        
        // Recalcular posición relativa al target
        let offset = Vec3::new(
            self.distance_from_target * self.yaw.sin(),
            self.position.y - self.target.y,
            self.distance_from_target * self.yaw.cos(),
        );
        self.position = self.target + offset;
    }

    /// Establece el objetivo de la cámara (para seguir planetas)
    pub fn look_at_target(&mut self, target: Vec3) {
        self.target = target;
        // Asegurar distancia mínima
        let safe_distance = self.distance_from_target.max(5.0);
        // Mantener distancia
        let offset = Vec3::new(
            safe_distance * self.yaw.sin(),
            5.0, // Altura sobre el plano
            safe_distance * self.yaw.cos(),
        );
        self.position = target + offset;
    }

    /// Cambia la distancia al objetivo (zoom)
    pub fn set_distance(&mut self, distance: f32) {
        self.distance_from_target = distance.max(5.0).min(100.0);
        self.look_at_target(self.target);
    }
}