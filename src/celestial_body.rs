use nalgebra::Vector3;
use raylib::prelude::Matrix;
use crate::shaders::ShaderType;
use crate::vertex::Vertex;
use crate::matrix::create_model_matrix;

#[derive(Clone)]
pub struct CelestialBody {
    pub name: String,
    pub body_type: BodyType,
    
    // Propiedades físicas
    pub scale: f32,
    pub rotation_speed: f32,      // Velocidad de rotación sobre su eje
    pub rotation_angle: f32,       // Ángulo actual de rotación
    
    // Propiedades orbitales
    pub orbit_radius: f32,         // Distancia del centro (0 para el sol)
    pub orbit_speed: f32,          // Velocidad de traslación
    pub orbit_angle: f32,          // Posición actual en la órbita (0-360°)
    pub orbit_center: Vector3<f32>,     // Centro de la órbita (para lunas)
    
    // Rendering
    pub shader_type: ShaderType,
    pub mesh: Vec<Vertex>,
    
    // Hijos (para sistema de lunas)
    pub satellites: Vec<CelestialBody>,
    
    // Rings
    pub has_rings: bool,
    pub ring_inner_radius: f32,
    pub ring_outer_radius: f32,
}

#[derive(Clone, Copy)]
pub enum BodyType {
    Star,
    Planet,
    Moon,
}

impl CelestialBody {
    pub fn new_star(name: &str, scale: f32, shader: ShaderType) -> Self {
        CelestialBody {
            name: name.to_string(),
            body_type: BodyType::Star,
            scale,
            rotation_speed: 0.1,
            rotation_angle: 0.0,
            orbit_radius: 0.0,  // El sol no orbita
            orbit_speed: 0.0,
            orbit_angle: 0.0,
            orbit_center: Vector3::zeros(),
            shader_type: shader,
            mesh: Vec::new(),
            satellites: Vec::new(),
            has_rings: false,
            ring_inner_radius: 0.0,
            ring_outer_radius: 0.0,
        }
    }
    
    pub fn new_planet(
        name: &str,
        scale: f32,
        orbit_radius: f32,
        orbit_speed: f32,
        rotation_speed: f32,
        shader: ShaderType
    ) -> Self {
        CelestialBody {
            name: name.to_string(),
            body_type: BodyType::Planet,
            scale,
            rotation_speed,
            rotation_angle: 0.0,
            orbit_radius,
            orbit_speed,
            orbit_angle: 0.0,  // Posición inicial aleatoria opcional
            orbit_center: Vector3::zeros(),
            shader_type: shader,
            mesh: Vec::new(),
            satellites: Vec::new(),
            has_rings: false,
            ring_inner_radius: 0.0,
            ring_outer_radius: 0.0,
        }
    }
    
    pub fn add_moon(&mut self, moon: CelestialBody) {
        self.satellites.push(moon);
    }
    
    // Actualizar posiciones
    pub fn update(&mut self, delta_time: f32) {
        // Rotación sobre su eje
        self.rotation_angle += self.rotation_speed * delta_time;
        if self.rotation_angle > 360.0 {
            self.rotation_angle -= 360.0;
        }
        
        // Traslación orbital
        self.orbit_angle += self.orbit_speed * delta_time;
        if self.orbit_angle > 360.0 {
            self.orbit_angle -= 360.0;
        }
        
        // Actualizar lunas (orbitan alrededor de este cuerpo)
        let current_position = self.get_world_position();
        for satellite in &mut self.satellites {
            satellite.orbit_center = current_position;
            satellite.update(delta_time);
        }
    }
    
    // Obtener posición en el espacio
    pub fn get_world_position(&self) -> Vector3<f32> {
        let angle_rad = self.orbit_angle.to_radians();
        Vector3::new(
            self.orbit_center.x + self.orbit_radius * angle_rad.cos(),
            self.orbit_center.y,  // Plano eclíptico (Y fijo)
            self.orbit_center.z + self.orbit_radius * angle_rad.sin(),
        )
    }
    
    // Obtener matriz de transformación
    pub fn get_model_matrix(&self) -> Matrix {
        use raylib::math::Vector3 as RVec3;
        let position = self.get_world_position();
        let rotation = Vector3::new(0.0, self.rotation_angle.to_radians(), 0.0);
        
        // Convertir nalgebra Vector3 a raylib Vector3
        let pos_raylib = RVec3::new(position.x, position.y, position.z);
        let rot_raylib = RVec3::new(rotation.x, rotation.y, rotation.z);
        
        create_model_matrix(pos_raylib, self.scale, rot_raylib)
    }
}