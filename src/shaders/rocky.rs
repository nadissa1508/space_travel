use crate::math::Vec3;
use super::common::*;
use super::FragmentData;

/// SHADER: PLANETA ROCOSO (Portado de GLSL)
/// Paleta de azules claros con terreno y cráteres
pub fn shader_rocky(fragment: &FragmentData, _time: f32) -> (f32, f32, f32) {
    let pos = fragment.position;
    let normal = fragment.normal.normalize();
    let light_dir = Vec3::new(1.0, 1.0, 2.0).normalize();

    // Calculate noise-based terrain
    let terrain = fbm_3d(Vec3::new(pos.x * 3.0, pos.y * 3.0, pos.z * 3.0), 5);
    let craters = fbm_3d(Vec3::new(pos.x * 8.0, pos.y * 8.0, pos.z * 8.0), 5);

    // Color variation based on terrain height (light blue palette)
    let color1 = Vec3::new(0.4, 0.6, 0.8);   // Light blue
    let color2 = Vec3::new(0.6, 0.75, 0.9);  // Lighter blue
    let color3 = Vec3::new(0.5, 0.65, 0.85); // Medium light blue

    let mut base_color = mix_v3(color1, color2, terrain);
    base_color = mix_v3(base_color, color3, craters * 0.5);

    // Add rocky variation
    let rock_detail = noise_3d(Vec3::new(pos.x * 20.0, pos.y * 20.0, pos.z * 20.0));
    base_color.x += rock_detail * 0.1;
    base_color.y += rock_detail * 0.1;
    base_color.z += rock_detail * 0.1;

    // Lighting (diffuse + ambient) - increased for visibility
    let diffuse = (normal.x * light_dir.x + normal.y * light_dir.y + normal.z * light_dir.z).max(0.0);
    let ambient = 0.5; // Increased ambient light

    let final_color = Vec3::new(
        base_color.x * (ambient + diffuse * 1.0), // Increased diffuse contribution
        base_color.y * (ambient + diffuse * 1.0),
        base_color.z * (ambient + diffuse * 1.0),
    );

    clamp_color(final_color.x, final_color.y, final_color.z)
}