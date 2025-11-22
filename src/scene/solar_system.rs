use crate::math::Vec3;
use crate::renderer::generate_orbit_points;
use crate::shaders::ShaderType;
use super::celestial_body::CelestialBody;

/// Sistema solar completo
pub struct SolarSystem {
    pub bodies: Vec<CelestialBody>,
    pub orbit_points: Vec<Vec<Vec3>>, // Puntos de órbita para cada cuerpo
}

impl SolarSystem {
    /// Crea un sistema solar personalizado con 6 cuerpos
    pub fn new() -> Self {
        let mut bodies = Vec::new();

        // Sol (estrella central) - Amarillo brillante
        bodies.push(CelestialBody::new(
            "Sol",
            2.5,                    // Radio grande
            (1.0, 0.9, 0.3),        // Amarillo dorado
            0.0,                    // Sin órbita (centro)
            0.0,                    // Sin velocidad orbital
            0.1,                    // Rotación lenta
            true,                   // Emite luz
            ShaderType::SolarHeart, // Shader solar_heart
        ));

        // Planeta 1: Ignis - Planeta volcánico de lava
        bodies.push(CelestialBody::new(
            "Ignis",
            0.4,
            (0.7, 0.4, 0.3),        // Rojizo
            5.0,                    // Órbita cercana
            0.8,                    // Rápido
            1.5,
            false,
            ShaderType::Lava,       // Shader de lava
        ));

        // Planeta 2: Terra - Planeta rocoso
        bodies.push(CelestialBody::new(
            "Terra",
            0.8,
            (0.2, 0.5, 0.8),        // Azulado
            8.0,
            0.5,
            1.0,
            false,
            ShaderType::Rocky,      // Shader rocoso
        ));

        // Planeta 3: Xenon - Planeta alien holográfico
        bodies.push(CelestialBody::new(
            "Xenon",
            0.6,
            (0.5, 0.1, 0.6),        // Púrpura base
            12.0,
            0.35,
            0.9,
            false,
            ShaderType::Alien,      // Shader alien holográfico
        ));

        // Planeta 4: Magnus - Gigante gaseoso
        bodies.push(CelestialBody::new(
            "Magnus",
            1.5,
            (0.8, 0.7, 0.5),        // Naranja/marrón
            18.0,
            0.2,
            2.0,                    // Rotación rápida como Júpiter
            false,
            ShaderType::GasGiant,   // Shader de gigante gaseoso
        ));

        // Planeta 5: Glacius - Planeta de hielo
        bodies.push(CelestialBody::new(
            "Glacius",
            1.0,
            (0.2, 0.3, 0.7),        // Azul oscuro
            25.0,
            0.12,
            0.8,
            false,
            ShaderType::Ice,        // Shader de hielo
        ));

        // Generar puntos de órbita para cada cuerpo
        let mut orbit_points = Vec::new();
        for body in &bodies {
            if body.orbit_radius > 0.0 {
                orbit_points.push(generate_orbit_points(body.orbit_radius, 64));
            } else {
                orbit_points.push(Vec::new());
            }
        }

        Self { bodies, orbit_points }
    }

    /// Actualiza todos los cuerpos
    pub fn update(&mut self, delta_time: f32) {
        for body in &mut self.bodies {
            body.update(delta_time);
        }
    }

    /// Obtiene un cuerpo por índice
    pub fn get_body(&self, index: usize) -> Option<&CelestialBody> {
        self.bodies.get(index)
    }

    /// Obtiene la posición de un cuerpo
    pub fn get_body_position(&self, index: usize) -> Vec3 {
        self.bodies.get(index)
            .map(|b| b.get_position())
            .unwrap_or(Vec3::zero())
    }

    /// Número de cuerpos
    pub fn body_count(&self) -> usize {
        self.bodies.len()
    }
}

impl Default for SolarSystem {
    fn default() -> Self {
        Self::new()
    }
}