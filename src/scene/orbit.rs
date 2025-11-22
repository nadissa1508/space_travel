use crate::math::Vec3;

/// Tipos de órbita soportados
#[derive(Debug, Clone, Copy)]
pub enum OrbitType {
    Circular,
    Elliptical { eccentricity: f32 },
}

/// Representa una órbita planetaria
#[derive(Debug, Clone)]
pub struct Orbit {
    pub orbit_type: OrbitType,
    pub radius: f32,          // Radio (semi-eje mayor para elípticas)
    pub speed: f32,           // Velocidad angular (radianes/segundo)
    pub inclination: f32,     // Inclinación respecto al plano eclíptico
    pub current_angle: f32,   // Ángulo actual en la órbita
}

impl Orbit {
    /// Crea una órbita circular simple
    pub fn circular(radius: f32, speed: f32) -> Self {
        Self {
            orbit_type: OrbitType::Circular,
            radius,
            speed,
            inclination: 0.0,
            current_angle: 0.0,
        }
    }

    /// Crea una órbita elíptica
    pub fn elliptical(radius: f32, eccentricity: f32, speed: f32) -> Self {
        Self {
            orbit_type: OrbitType::Elliptical { eccentricity },
            radius,
            speed,
            inclination: 0.0,
            current_angle: 0.0,
        }
    }

    /// Establece la inclinación de la órbita
    pub fn with_inclination(mut self, inclination: f32) -> Self {
        self.inclination = inclination;
        self
    }

    /// Establece el ángulo inicial
    pub fn with_initial_angle(mut self, angle: f32) -> Self {
        self.current_angle = angle;
        self
    }

    /// Actualiza el ángulo de la órbita
    pub fn update(&mut self, delta_time: f32) {
        self.current_angle += self.speed * delta_time;
        
        // Mantener en rango [0, 2π]
        if self.current_angle > std::f32::consts::TAU {
            self.current_angle -= std::f32::consts::TAU;
        }
    }

    /// Obtiene la posición actual en la órbita
    pub fn get_position(&self) -> Vec3 {
        match self.orbit_type {
            OrbitType::Circular => {
                let x = self.radius * self.current_angle.cos();
                let z = self.radius * self.current_angle.sin();
                let y = z * self.inclination.sin(); // Aplicar inclinación
                let z_adjusted = z * self.inclination.cos();
                Vec3::new(x, y, z_adjusted)
            }
            OrbitType::Elliptical { eccentricity } => {
                // r = a(1 - e²) / (1 + e*cos(θ))
                let r = self.radius * (1.0 - eccentricity * eccentricity)
                    / (1.0 + eccentricity * self.current_angle.cos());
                let x = r * self.current_angle.cos();
                let z = r * self.current_angle.sin();
                let y = z * self.inclination.sin();
                let z_adjusted = z * self.inclination.cos();
                Vec3::new(x, y, z_adjusted)
            }
        }
    }

    /// Genera puntos para visualizar la órbita completa
    pub fn generate_orbit_path(&self, segments: usize) -> Vec<Vec3> {
        let mut points = Vec::with_capacity(segments);
        
        for i in 0..segments {
            let angle = std::f32::consts::TAU * (i as f32) / (segments as f32);
            
            let (x, z) = match self.orbit_type {
                OrbitType::Circular => {
                    (self.radius * angle.cos(), self.radius * angle.sin())
                }
                OrbitType::Elliptical { eccentricity } => {
                    let r = self.radius * (1.0 - eccentricity * eccentricity)
                        / (1.0 + eccentricity * angle.cos());
                    (r * angle.cos(), r * angle.sin())
                }
            };
            
            let y = z * self.inclination.sin();
            let z_adjusted = z * self.inclination.cos();
            
            points.push(Vec3::new(x, y, z_adjusted));
        }
        
        points
    }

    /// Calcula la velocidad orbital en un punto dado (para órbitas elípticas)
    pub fn get_orbital_velocity(&self) -> f32 {
        match self.orbit_type {
            OrbitType::Circular => self.speed * self.radius,
            OrbitType::Elliptical { eccentricity } => {
                // Velocidad varía en órbita elíptica (más rápido en perihelio)
                let r = self.radius * (1.0 - eccentricity * eccentricity)
                    / (1.0 + eccentricity * self.current_angle.cos());
                self.speed * self.radius * self.radius / r
            }
        }
    }
}

/// Órbita nula (para el sol que está estático)
impl Default for Orbit {
    fn default() -> Self {
        Self {
            orbit_type: OrbitType::Circular,
            radius: 0.0,
            speed: 0.0,
            inclination: 0.0,
            current_angle: 0.0,
        }
    }
}