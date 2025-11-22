use crate::math::Vec3;
use super::common::*;
use super::FragmentData;

/// SHADER: PLANETA ALIEN HOLOGRÁFICO (Extra 3)
/// Púrpura, rosa y aqua con efecto holográfico
pub fn shader_alien(fragment: &FragmentData, time: f32) -> (f32, f32, f32) {
    let pos = fragment.position;
    let normal = fragment.normal.normalize();

    // Base dark purple
    let dark_base = Vec3::new(0.08, 0.02, 0.15);

    // Holographic pulses (multiple frequencies for shimmer effect)
    let pulse_fast = (time * 3.0).sin() * 0.5 + 0.5;
    let pulse_slow = (time * 1.5).sin() * 0.5 + 0.5;
    let pulse_shimmer = (time * 5.0 + pos.x * 10.0).sin() * 0.5 + 0.5;

    // Holographic stripe pattern (like interference patterns)
    let stripe_pattern = ((pos.y * 15.0 + time * 2.0).sin() * (pos.x * 10.0).cos()).abs();
    let stripes = smoothstep(0.3, 0.7, stripe_pattern);

    // Flowing energy veins
    let vein_sample = Vec3::new(pos.x * 6.0 + time * 0.4, pos.y * 6.0, pos.z * 6.0);
    let veins = fbm_3d(vein_sample, 3);
    let vein_intensity = smoothstep(0.55, 0.75, veins);

    // Iridescent spots (color-shifting)
    let spots_sample = Vec3::new(pos.x * 12.0 + time * 0.3, pos.y * 12.0, pos.z * 12.0);
    let spots = noise_3d(spots_sample);
    let spot_intensity = smoothstep(0.65, 0.80, spots);

    // Holographic colors - Purple, Pink, Aqua
    let vivid_purple = Vec3::new(0.7, 0.1, 0.9);    // Vivid purple
    let hot_pink = Vec3::new(1.0, 0.2, 0.7);        // Hot pink
    let electric_aqua = Vec3::new(0.1, 0.9, 0.9);   // Electric aqua
    let neon_magenta = Vec3::new(0.9, 0.0, 0.8);    // Neon magenta

    // Color cycling based on position and time (holographic effect)
    let color_cycle = (pos.x * 5.0 + pos.y * 3.0 + time * 1.0).sin() * 0.5 + 0.5;
    let holographic_color = if color_cycle < 0.33 {
        mix_v3(vivid_purple, hot_pink, color_cycle * 3.0)
    } else if color_cycle < 0.66 {
        mix_v3(hot_pink, electric_aqua, (color_cycle - 0.33) * 3.0)
    } else {
        mix_v3(electric_aqua, vivid_purple, (color_cycle - 0.66) * 3.0)
    };

    // Combine all layers
    let mut final_color = dark_base;

    // Add holographic stripes
    final_color = mix_v3(final_color, holographic_color, stripes * pulse_shimmer * 0.6);

    // Add energy veins with pink/magenta
    final_color = mix_v3(final_color, neon_magenta, vein_intensity * pulse_fast * 0.5);

    // Add iridescent spots with aqua
    final_color = mix_v3(final_color, electric_aqua, spot_intensity * pulse_slow * 0.4);

    // Holographic rim glow (rainbow-like edge)
    let view_dir = Vec3::new(-pos.x, -pos.y, -pos.z).normalize();
    let fresnel = (1.0 - (normal.x * view_dir.x + normal.y * view_dir.y + normal.z * view_dir.z).max(0.0))
        .powf(2.5);

    // Rim color shifts between purple, pink, aqua
    let rim_color = mix_v3(
        mix_v3(vivid_purple, hot_pink, pulse_fast),
        electric_aqua,
        pulse_slow,
    );

    final_color.x += rim_color.x * fresnel * 0.6;
    final_color.y += rim_color.y * fresnel * 0.6;
    final_color.z += rim_color.z * fresnel * 0.6;

    // Overall holographic shimmer
    let shimmer_boost = pulse_shimmer * 0.3;
    final_color.x += shimmer_boost;
    final_color.y += shimmer_boost * 0.8;
    final_color.z += shimmer_boost;

    // Minimal lighting (self-illuminating hologram)
    let ambient = 0.75;
    final_color.x *= ambient;
    final_color.y *= ambient;
    final_color.z *= ambient;

    clamp_color(final_color.x, final_color.y, final_color.z)
}