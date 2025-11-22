pub mod common;
pub mod solar_heart;
pub mod rocky;
pub mod gas_giant;
pub mod ice;
pub mod lava;
pub mod alien;

pub use common::*;
pub use solar_heart::shader_solar_heart;
pub use rocky::shader_rocky;
pub use gas_giant::shader_gas_giant;
pub use ice::shader_ice;
pub use lava::shader_lava;
pub use alien::shader_alien;

use crate::math::Vec3;

/// Tipos de shader disponibles
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShaderType {
    SolarHeart, // Estrella con patrón de corazón
    Rocky,      // Planeta rocoso
    GasGiant,   // Gigante gaseoso tipo Júpiter
    Lava,       // Planeta volcánico
    Ice,        // Planeta helado
    Alien,      // Planeta alien holográfico
}

/// Datos del fragmento para el shader
#[derive(Debug, Clone, Copy)]
pub struct FragmentData {
    pub position: Vec3,      // Posición en espacio local
    pub normal: Vec3,        // Normal del fragmento
    pub world_pos: Vec3,     // Posición en espacio mundo
}

/// Aplica el shader correspondiente
pub fn apply_shader(
    shader_type: ShaderType,
    fragment: &FragmentData,
    time: f32,
    _light_dir: &Vec3,
) -> (f32, f32, f32) {
    match shader_type {
        ShaderType::SolarHeart => shader_solar_heart(fragment, time),
        ShaderType::Rocky => shader_rocky(fragment, time),
        ShaderType::GasGiant => shader_gas_giant(fragment, time),
        ShaderType::Lava => shader_lava(fragment, time),
        ShaderType::Ice => shader_ice(fragment, time),
        ShaderType::Alien => shader_alien(fragment, time),
    }
}