use crate::math::Vec3;
use super::common::*;
use super::FragmentData;

/// SHADER: GIGANTE GASEOSO (Portado de GLSL)
/// Bandas horizontales tipo Júpiter con turbulencia
pub fn shader_gas_giant(fragment: &FragmentData, time: f32) -> (f32, f32, f32) {
    let pos = fragment.position;
    let normal = fragment.normal.normalize();
    let light_dir = Vec3::new(1.0, 1.0, 2.0).normalize();

    // Create horizontal bands based on Y position
    let mut bands = pos.y * 5.0 + time * 0.2;

    // Add swirling effect with noise
    let swirl = noise_3d(Vec3::new(
        pos.x * 2.0,
        pos.y * 8.0 + time * 0.1,
        pos.z * 2.0,
    ));
    bands += swirl * 2.0;

    // More turbulence
    let turbulence = noise_3d(Vec3::new(
        pos.x * 6.0 + time * 0.15,
        pos.y * 15.0,
        pos.z * 6.0,
    ));
    bands += turbulence * 0.8;

    // Color bands (Jupiter-like colors)
    let color1 = Vec3::new(0.9, 0.7, 0.5);   // Light orange
    let color2 = Vec3::new(0.7, 0.5, 0.3);   // Dark orange
    let color3 = Vec3::new(0.95, 0.85, 0.7); // Cream
    let color4 = Vec3::new(0.6, 0.4, 0.25);  // Brown

    let band_pattern = fract(bands);
    let base_color = if band_pattern < 0.25 {
        mix_v3(color1, color2, band_pattern * 4.0)
    } else if band_pattern < 0.5 {
        mix_v3(color2, color3, (band_pattern - 0.25) * 4.0)
    } else if band_pattern < 0.75 {
        mix_v3(color3, color4, (band_pattern - 0.5) * 4.0)
    } else {
        mix_v3(color4, color1, (band_pattern - 0.75) * 4.0)
    };

    // Add atmospheric glow
    let view_dir = Vec3::new(0.0, 0.0, 1.0);
    let atmosphere = (1.0 - (normal.x * view_dir.x + normal.y * view_dir.y + normal.z * view_dir.z).abs())
        .powf(2.0);
    
    let mut final_color = Vec3::new(
        base_color.x + 0.1 * atmosphere,
        base_color.y + 0.05 * atmosphere,
        base_color.z,
    );

    // Lighting
    let diffuse = (normal.x * light_dir.x + normal.y * light_dir.y + normal.z * light_dir.z).max(0.0);
    let ambient = 0.4;

    final_color.x *= ambient + diffuse * 0.6;
    final_color.y *= ambient + diffuse * 0.6;
    final_color.z *= ambient + diffuse * 0.6;

    clamp_color(final_color.x, final_color.y, final_color.z)
}