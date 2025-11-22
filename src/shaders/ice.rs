use crate::math::Vec3;
use super::common::*;
use super::FragmentData;

/// SHADER: PLANETA DE HIELO (Extra 2)
/// Hielo con grietas, nieve, cristales y auroras boreales
pub fn shader_ice(fragment: &FragmentData, time: f32) -> (f32, f32, f32) {
    let pos = fragment.position;
    let normal = fragment.normal.normalize();

    // CAPA 1: Base de hielo con variaciones
    let ice_variation = fbm(pos.x * 3.0, pos.z * 3.0, 3);
    let pure_ice = Vec3::new(0.85, 0.92, 1.0);
    let glacier_ice = Vec3::new(0.70, 0.82, 0.95);
    let ice_base = Vec3::new(
        pure_ice.x + (glacier_ice.x - pure_ice.x) * ice_variation,
        pure_ice.y + (glacier_ice.y - pure_ice.y) * ice_variation,
        pure_ice.z + (glacier_ice.z - pure_ice.z) * ice_variation,
    );

    // CAPA 2: Grietas profundas en hielo
    let crack_large = fbm(pos.x * 8.0, pos.z * 8.0, 4);
    let crack_small = fbm(pos.x * 18.0, pos.z * 18.0, 2);
    let crack_pattern = smoothstep(0.35, 0.45, crack_large) * 0.7 
        + smoothstep(0.38, 0.42, crack_small) * 0.3;
    let deep_ice = Vec3::new(0.25, 0.45, 0.65);

    // CAPA 3: Capas de nieve brillante
    let snow_pattern = fbm(pos.x * 5.0, pos.y * 5.0, 3);
    let snow_coverage = smoothstep(0.45, 0.65, snow_pattern);
    let fresh_snow = Vec3::new(0.98, 0.99, 1.0);

    // CAPA 4: Cristales de hielo (sparkle)
    let crystals = fbm(pos.x * 10.0 + time * 0.08, pos.z * 10.0, 2);
    let sparkle = smoothstep(0.75, 0.88, crystals) * ((time * 2.5).sin() * 0.5 + 0.5) * 0.5;

    // CAPA 5: Auroras boreales
    let aurora_flow1 = ((pos.x * 3.0 + time * 0.5).sin() * (pos.z * 2.0 + time * 0.3).cos()).abs();
    let aurora_flow2 = ((pos.x * 4.0 - time * 0.4).cos() * (pos.z * 3.0).sin()).abs();

    // Intensidad basada en latitud (más fuerte en polos)
    let polar_intensity = ((pos.y.abs() - 0.3).max(0.0) / 0.7).powf(1.5);
    let aurora_intensity = (aurora_flow1 * 0.6 + aurora_flow2 * 0.4) * polar_intensity;

    // Colores de aurora que cambian con el tiempo
    let time_shift = time * 0.2;
    let aurora_green = Vec3::new(0.1, 0.9, 0.5);
    let aurora_cyan = Vec3::new(0.2, 0.7, 0.9);
    let aurora_purple = Vec3::new(0.6, 0.3, 0.9);

    let aurora_mix = time_shift.sin() * 0.5 + 0.5;
    let aurora_color = if aurora_mix < 0.33 {
        aurora_green
    } else if aurora_mix < 0.66 {
        aurora_cyan
    } else {
        aurora_purple
    };

    // CAPA 6: Atmósfera helada (rim glow)
    let view_dir = Vec3::new(-pos.x, -pos.y, -pos.z).normalize();
    let rim = (1.0 - (normal.x * view_dir.x + normal.y * view_dir.y + normal.z * view_dir.z).max(0.0))
        .powf(2.0) * 0.3;

    // Combinar capas
    let mut final_color = Vec3::new(
        ice_base.x * (1.0 - crack_pattern) + deep_ice.x * crack_pattern,
        ice_base.y * (1.0 - crack_pattern) + deep_ice.y * crack_pattern,
        ice_base.z * (1.0 - crack_pattern) + deep_ice.z * crack_pattern,
    );

    // Blend con nieve
    final_color.x = final_color.x * (1.0 - snow_coverage) + fresh_snow.x * snow_coverage;
    final_color.y = final_color.y * (1.0 - snow_coverage) + fresh_snow.y * snow_coverage;
    final_color.z = final_color.z * (1.0 - snow_coverage) + fresh_snow.z * snow_coverage;

    // Iluminación especular fuerte (hielo muy reflectivo)
    let light_dir = Vec3::new(1.0, 1.0, 2.0).normalize();
    let diffuse = (normal.x * light_dir.x + normal.y * light_dir.y + normal.z * light_dir.z).max(0.0);
    let specular = diffuse.powf(4.0) * 0.5;
    let ambient = 0.5; // Base ambient lighting
    
    // Apply diffuse lighting to base color
    final_color.x *= (ambient + diffuse * 0.8);
    final_color.y *= (ambient + diffuse * 0.8);
    final_color.z *= (ambient + diffuse * 0.8);
    
    final_color.x += specular;
    final_color.y += specular;
    final_color.z += specular;

    // Añadir cristales brillantes
    final_color.x += sparkle * 0.4;
    final_color.y += sparkle * 0.4;
    final_color.z += sparkle * 0.4;

    // Añadir auroras boreales
    final_color.x += aurora_color.x * aurora_intensity * 0.4;
    final_color.y += aurora_color.y * aurora_intensity * 0.4;
    final_color.z += aurora_color.z * aurora_intensity * 0.4;

    // Añadir atmósfera helada
    let atmo_ice = Vec3::new(0.7, 0.85, 1.0);
    final_color.x += atmo_ice.x * rim;
    final_color.y += atmo_ice.y * rim;
    final_color.z += atmo_ice.z * rim;

    clamp_color(final_color.x, final_color.y, final_color.z)
}