use crate::celestial_body::*;
use crate::shaders::ShaderType;
use rand::Rng;

pub struct SolarSystem {
    pub bodies: Vec<CelestialBody>,
    pub orbit_lines_enabled: bool,
}

impl SolarSystem {
    pub fn new() -> Self {
        let mut system = SolarSystem {
            bodies: Vec::new(),
            orbit_lines_enabled: true,
        };
        
        system.initialize_solar_system();
        system
    }
    
    fn initialize_solar_system(&mut self) {
        let mut rng = rand::rng();

        // ⭐ Sol central
        let sun = CelestialBody::new_star(
            "Sol",
            1.5,
            ShaderType::SolarHeart  // Modificado con tonos azules
        );
        self.bodies.push(sun);

        // 🪐 Mercurio
        let mut mercury = CelestialBody::new_planet(
            "Mercurio",
            0.3,
            3.0,   // Radio de órbita
            0.8,   // Velocidad orbital (más rápido = más cerca)
            1.2,   // Velocidad de rotación
            ShaderType::Rocky
        );
        mercury.orbit_angle = rng.random_range(0.0..360.0);
        self.bodies.push(mercury);

        // 🪐 Venus
        let mut venus = CelestialBody::new_planet(
            "Venus",
            0.5,
            5.0,
            0.6,
            0.9,
            ShaderType::GasGiant
        );
        venus.orbit_angle = rng.random_range(0.0..360.0);
        self.bodies.push(venus);

        // 🌍 Tierra (con luna)
        let mut earth = CelestialBody::new_planet(
            "Tierra",
            0.6,
            8.0,
            0.4,
            1.0,
            ShaderType::Ice  // Modificado para tonos azul agua
        );
        earth.orbit_angle = rng.random_range(0.0..360.0);

        // 🌙 Luna de la Tierra
        let mut moon = CelestialBody::new_planet(
            "Luna",
            0.15,
            1.0,   // Orbita a 1.0 unidades de la Tierra
            2.0,   // Más rápida (órbita mensual vs anual)
            0.5,
            ShaderType::Moon
        );
        moon.orbit_angle = rng.random_range(0.0..360.0);
        earth.add_moon(moon);
        self.bodies.push(earth);

        // 🪐 Marte (con luna)
        let mut mars = CelestialBody::new_planet(
            "Marte",
            0.4,
            11.0,
            0.3,
            1.1,
            ShaderType::Rocky
        );
        mars.orbit_angle = rng.random_range(0.0..360.0);
        
        // 🌙 Fobos - Luna de Marte
        let mut phobos = CelestialBody::new_planet(
            "Fobos",
            0.12,
            0.8,   // Orbita a 0.8 unidades de Marte
            2.5,   // Rápida
            1.0,
            ShaderType::Moon
        );
        phobos.orbit_angle = rng.random_range(0.0..360.0);
        mars.add_moon(phobos);
        self.bodies.push(mars);

        // 🪐 Júpiter (con anillos)
        let mut jupiter = CelestialBody::new_planet(
            "Júpiter",
            1.2,
            16.0,
            0.15,
            0.7,
            ShaderType::GasGiant
        );
        jupiter.orbit_angle = rng.random_range(0.0..360.0);
        jupiter.has_rings = true;
        jupiter.ring_inner_radius = 1.5;
        jupiter.ring_outer_radius = 2.5;
        self.bodies.push(jupiter);
        
        // 🪐 Planeta Alien (con 2 lunas)
        let mut alien_planet = CelestialBody::new_planet(
            "Xenos",
            0.7,
            20.0,
            0.12,
            0.8,
            ShaderType::Alien
        );
        alien_planet.orbit_angle = rng.random_range(0.0..360.0);
        
        // 🌙 Primera luna de Xenos
        let mut moon1 = CelestialBody::new_planet(
            "Xenos-A",
            0.18,
            1.2,
            2.0,
            1.2,
            ShaderType::Ice
        );
        moon1.orbit_angle = rng.random_range(0.0..360.0);
        alien_planet.add_moon(moon1);
        
        // 🌙 Segunda luna de Xenos
        let mut moon2 = CelestialBody::new_planet(
            "Xenos-B",
            0.15,
            1.8,
            1.5,
            0.9,
            ShaderType::Lava
        );
        moon2.orbit_angle = rng.random_range(0.0..360.0);
        alien_planet.add_moon(moon2);
        self.bodies.push(alien_planet);
    }
    
    pub fn update(&mut self, delta_time: f32) {
        for body in &mut self.bodies {
            body.update(delta_time);
        }
    }
    
    pub fn get_body_by_name(&self, name: &str) -> Option<&CelestialBody> {
        self.bodies.iter().find(|b| b.name == name)
    }
}