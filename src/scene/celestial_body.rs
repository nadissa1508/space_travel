use crate::math::{Vec3, Mat4};
use crate::renderer::{Vertex, generate_sphere};

use crate::shaders::ShaderType;

/// Representa un cuerpo celeste (sol, planeta, luna)
pub struct CelestialBody {
    pub name: String,
    pub radius: f32,
    pub color: (f32, f32, f32),
    pub orbit_radius: f32,      // Distancia al centro de órbita
    pub orbit_speed: f32,       // Velocidad orbital (radianes/segundo)
    pub rotation_speed: f32,    // Velocidad de rotación propia
    pub orbit_angle: f32,       // Ángulo actual en la órbita
    pub rotation_angle: f32,    // Ángulo de rotación propia
    pub is_emissive: bool,      // True para el sol (brilla por sí mismo)
    pub shader_type: ShaderType, // Tipo de shader para este cuerpo
    pub mesh: Vec<[Vertex; 3]>, // Triángulos de la esfera
}

impl CelestialBody {
    pub fn new(
        name: &str,
        radius: f32,
        color: (f32, f32, f32),
        orbit_radius: f32,
        orbit_speed: f32,
        rotation_speed: f32,
        is_emissive: bool,
        shader_type: ShaderType,
    ) -> Self {
        // Más segmentos para planetas grandes, menos para pequeños
        let detail = if radius > 1.5 { 24 } else { 16 };
        let mesh = generate_sphere(radius, detail, detail, color);

        Self {
            name: name.to_string(),
            radius,
            color,
            orbit_radius,
            orbit_speed,
            rotation_speed,
            orbit_angle: 0.0,
            rotation_angle: 0.0,
            is_emissive,
            shader_type,
            mesh,
        }
    }

    /// Actualiza la posición orbital y rotación
    pub fn update(&mut self, delta_time: f32) {
        self.orbit_angle += self.orbit_speed * delta_time;
        self.rotation_angle += self.rotation_speed * delta_time;

        // Mantener ángulos en rango [0, TAU) usando módulo
        // Esto previene overflow cuando delta_time es muy grande
        self.orbit_angle = self.orbit_angle % std::f32::consts::TAU;
        self.rotation_angle = self.rotation_angle % std::f32::consts::TAU;

        // Manejar ángulos negativos (aunque no debería ocurrir en este caso)
        if self.orbit_angle < 0.0 {
            self.orbit_angle += std::f32::consts::TAU;
        }
        if self.rotation_angle < 0.0 {
            self.rotation_angle += std::f32::consts::TAU;
        }
    }

    /// Obtiene la posición actual en el espacio
    pub fn get_position(&self) -> Vec3 {
        if self.orbit_radius == 0.0 {
            Vec3::zero() // El sol está en el centro
        } else {
            Vec3::new(
                self.orbit_radius * self.orbit_angle.cos(),
                0.0, // En el plano eclíptico
                self.orbit_radius * self.orbit_angle.sin(),
            )
        }
    }

    /// Obtiene la matriz de modelo (traslación + rotación)
    pub fn get_model_matrix(&self) -> Mat4 {
        let position = self.get_position();
        let translation = Mat4::translation(position.x, position.y, position.z);
        let rotation = Mat4::rotation_y(self.rotation_angle);
        translation.multiply(&rotation)
    }
}