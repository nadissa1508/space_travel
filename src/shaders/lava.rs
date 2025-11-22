use crate::math::Vec3;
use super::common::*;
use super::FragmentData;

/// SHADER: PLANETA DE LAVA (Extra 1)
/// Naranja y rojo con sombras negras
pub fn shader_lava(fragment: &FragmentData, time: f32) -> (f32, f32, f32) {
    let pos = fragment.position;
    let normal = fragment.normal.normalize();
    let light_dir = Vec3::new(1.0, 1.0, 2.0).normalize();

    // Lava flow pattern with animation
    let flow1 = fbm(pos.x * 2.0 + time * 0.3, pos.z * 2.0 + time * 0.25, 3);
    let flow2 = fbm(pos.x * 3.5 - time * 0.15, pos.y * 3.5, 2);
    let lava_flow = (flow1 * 0.7 + flow2 * 0.3).clamp(0.0, 1.0);

    // Define colors with BLACK shadows for contrast
    let black_crust = Vec3::new(0.0, 0.0, 0.0);   // Pure black shadows
    let dark_crust = Vec3::new(0.1, 0.05, 0.0);   // Very dark crust
    let dark_red = Vec3::new(0.8, 0.1, 0.0);      // Deep red
    let bright_red = Vec3::new(1.0, 0.2, 0.0);    // Bright red
    let hot_orange = Vec3::new(1.0, 0.6, 0.0);    // Hot orange
    let yellow_hot = Vec3::new(1.0, 0.9, 0.2);    // Yellow-hot

    // Lava rivers vs solid crust - sharp threshold
    let is_lava = smoothstep(0.35, 0.50, lava_flow);

    // Crust color (dark with shadows)
    let crust_color = mix_v3(black_crust, dark_crust, lava_flow * 2.0);

    // Lava color gradient (bright flowing lava)
    let lava_color = if lava_flow < 0.5 {
        dark_red
    } else if lava_flow < 0.65 {
        mix_v3(dark_red, bright_red, (lava_flow - 0.5) / 0.15)
    } else if lava_flow < 0.8 {
        mix_v3(bright_red, hot_orange, (lava_flow - 0.65) / 0.15)
    } else {
        mix_v3(hot_orange, yellow_hot, (lava_flow - 0.8) / 0.2)
    };

    // Mix crust and lava with sharp contrast
    let base_color = mix_v3(crust_color, lava_color, is_lava);

    // Add flowing cracks
    let cracks = fbm(pos.x * 6.0, pos.z * 6.0, 2);
    let crack_intensity = smoothstep(0.55, 0.70, cracks);
    let crack_color = Vec3::new(1.0, 0.5, 0.0); // Orange cracks
    let color_with_cracks = mix_v3(base_color, crack_color, crack_intensity * is_lava * 0.5);

    // Heat pulse (breathing effect)
    let pulse = (time * 1.5).sin() * 0.5 + 0.5;
    let heat_boost = pulse * is_lava * 0.15;

    // Lighting for shadows on dark crust
    let diffuse = (normal.x * light_dir.x + normal.y * light_dir.y + normal.z * light_dir.z).max(0.0);
    let shadow_factor = 0.1 + diffuse * 0.3;

    // Apply shadows only to crust (not lava - it emits light)
    let crust_with_shadow = Vec3::new(
        color_with_cracks.x * (is_lava + (1.0 - is_lava) * shadow_factor),
        color_with_cracks.y * (is_lava + (1.0 - is_lava) * shadow_factor),
        color_with_cracks.z * (is_lava + (1.0 - is_lava) * shadow_factor),
    );

    // Add emission glow
    let final_color = Vec3::new(
        (crust_with_shadow.x + heat_boost).clamp(0.0, 1.0),
        (crust_with_shadow.y + heat_boost * 0.5).clamp(0.0, 1.0),
        (crust_with_shadow.z * 0.3).clamp(0.0, 1.0),
    );

    (final_color.x, final_color.y, final_color.z)
}