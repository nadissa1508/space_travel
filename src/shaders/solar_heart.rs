use crate::math::Vec3;
use super::common::*;
use super::FragmentData;

// Heart-shaped Signed Distance Function (SDF)
fn heart_sdf(px: f32, py: f32) -> f32 {
    let x = px;
    let y = py;
    let a = x * x + y * y - 1.0;
    a * a * a - x * x * y * y * y
}

// Generate heart pattern intensity
fn heart_pattern(px: f32, py: f32, thickness: f32) -> f32 {
    let d = heart_sdf(px, py);
    let intensity = 1.0 - (d.abs() / thickness).clamp(0.0, 1.0);
    smoothstep(0.0, 1.0, intensity)
}

/// SHADER: SOLAR HEART - Estrella con patrón de corazón
pub fn shader_solar_heart(fragment: &FragmentData, time: f32) -> (f32, f32, f32) {
    let pos = fragment.position;
    let normal = fragment.normal.normalize();

    // === GLOBAL PULSATION (Breathing Effect) ===
    let pulse_slow = (time * 1.0).sin() * 0.5 + 0.5;
    let pulse_fast = (time * 3.0).sin() * 0.5 + 0.5;
    let pulse_heart = (time * 0.7).sin() * 0.5 + 0.5;
    let pulse_combined = pulse_slow * 0.6 + pulse_fast * 0.4;

    // === CAPA 1: BASE TURBULENCE ===
    let turbulence_scale = 10.0;
    let turbulence = fbm_3d(
        Vec3::new(
            pos.x * turbulence_scale + time * 0.2,
            pos.y * turbulence_scale - time * 0.15,
            pos.z * turbulence_scale + time * 0.18,
        ),
        4,
    );

    // === CAPA 2: HIGH-ENERGY REGIONS ===
    let energy_scale = 15.0;
    let energy_zones = fbm_3d(
        Vec3::new(
            pos.x * energy_scale + time * 0.35,
            pos.y * energy_scale,
            pos.z * energy_scale - time * 0.3,
        ),
        5,
    );
    let energy_intensity = smoothstep(0.55, 0.75, energy_zones);

    // === CAPA 3: SOLAR SPARKS ===
    let spark_pos = Vec3::new(
        pos.x * 20.0 + time * 0.5,
        pos.y * 20.0 + time * 0.4,
        pos.z * 20.0 + time * 0.45,
    );
    let spark_value = hash_v3(spark_pos);
    let spark_threshold = 0.98;
    let spark_raw = if spark_value > spark_threshold {
        (spark_value - spark_threshold) / (1.0 - spark_threshold)
    } else {
        0.0
    };
    let spark_intensity = smoothstep(0.0, 1.0, spark_raw) * pulse_fast * 0.6;

    // === CAPA 4: HEART PATTERN EXPANSION ===
    let radius_from_center = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
    let heart_scale = 2.5;
    let heart_2d_x = pos.x * heart_scale;
    let heart_2d_y = pos.y * heart_scale;

    let wave_speed = 0.5;
    let wave_frequency = 2.0;

    let wave1_phase = (time * wave_speed) % wave_frequency;
    let wave2_phase = (time * wave_speed + 0.66) % wave_frequency;
    let wave3_phase = (time * wave_speed + 1.33) % wave_frequency;

    let heart1_scale = 0.5 + wave1_phase * 1.5;
    let heart2_scale = 0.5 + wave2_phase * 1.5;
    let heart3_scale = 0.5 + wave3_phase * 1.5;

    let heart1 = heart_pattern(
        heart_2d_x / heart1_scale,
        heart_2d_y / heart1_scale,
        0.8,
    ) * (1.0 - wave1_phase / wave_frequency);

    let heart2 = heart_pattern(
        heart_2d_x / heart2_scale,
        heart_2d_y / heart2_scale,
        0.8,
    ) * (1.0 - wave2_phase / wave_frequency);

    let heart3 = heart_pattern(
        heart_2d_x / heart3_scale,
        heart_2d_y / heart3_scale,
        0.8,
    ) * (1.0 - wave3_phase / wave_frequency);

    let heart_intensity = (heart1 + heart2 + heart3).clamp(0.0, 1.0) * pulse_heart;

    // === CAPA 5: YELLOW SOLAR FLARES ===
    let flare_scale = 8.0;
    let flare_noise = fbm_3d(
        Vec3::new(
            pos.x * flare_scale + time * 0.4,
            pos.y * flare_scale - time * 0.3,
            pos.z * flare_scale + time * 0.35,
        ),
        4,
    );

    let flare_detail = noise_3d(Vec3::new(
        pos.x * 15.0 + time * 0.6,
        pos.y * 15.0,
        pos.z * 15.0 - time * 0.5,
    ));

    let normal_view_dot = (normal.x * (-pos.x) + normal.y * (-pos.y) + normal.z * (-pos.z))
        / ((pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt() + 0.001);
    let is_at_edge = 1.0 - normal_view_dot.abs();

    let flare_threshold = 0.55;
    let flare_mask = smoothstep(flare_threshold, flare_threshold + 0.15, flare_noise);
    let flare_intensity = (is_at_edge.powf(2.5) * flare_mask * (0.7 + flare_detail * 0.3) * pulse_fast)
        .clamp(0.0, 1.0);

    let flare_bright_yellow = Vec3::new(1.0, 0.95, 0.3);
    let flare_orange = Vec3::new(1.0, 0.7, 0.2);
    let flare_color = mix_v3(flare_orange, flare_bright_yellow, flare_detail);

    // === RADIAL GRADIENT ===
    let center_distance = radius_from_center;

    let core_white = Vec3::new(1.0, 0.95, 0.98);
    let core_yellow = Vec3::new(1.0, 0.85, 0.5);
    let middle_orange = Vec3::new(1.0, 0.45, 0.35);
    let middle_red = Vec3::new(0.95, 0.25, 0.35);
    let outer_pink = Vec3::new(1.0, 0.2, 0.6);
    let deep_pink = Vec3::new(0.95, 0.15, 0.5);

    let base_color = if center_distance < 0.3 {
        mix_v3(core_yellow, core_white, (center_distance / 0.3) * pulse_combined)
    } else if center_distance < 0.5 {
        let t = (center_distance - 0.3) / 0.2;
        mix_v3(core_yellow, middle_orange, t)
    } else if center_distance < 0.8 {
        let t = (center_distance - 0.5) / 0.3;
        mix_v3(middle_orange, middle_red, t)
    } else if center_distance < 1.0 {
        let t = (center_distance - 0.8) / 0.2;
        mix_v3(middle_red, outer_pink, t)
    } else {
        let t = ((center_distance - 1.0) / 0.2).clamp(0.0, 1.0);
        mix_v3(outer_pink, deep_pink, t * turbulence)
    };

    // === COMBINE ALL LAYERS ===
    let mut final_color = Vec3::new(
        base_color.x * (0.8 + turbulence * 0.2),
        base_color.y * (0.8 + turbulence * 0.2),
        base_color.z * (0.8 + turbulence * 0.2),
    );

    // Heart pattern
    let heart_boost = heart_intensity * 0.8;
    final_color.x += heart_boost * outer_pink.x;
    final_color.y += heart_boost * outer_pink.y;
    final_color.z += heart_boost * outer_pink.z;

    // High-energy zones
    final_color.x += energy_intensity * 0.3 * core_yellow.x;
    final_color.y += energy_intensity * 0.3 * core_yellow.y;
    final_color.z += energy_intensity * 0.2;

    // Solar sparks
    if spark_intensity > 0.0 {
        let spark_color = Vec3::new(1.0, 0.85, 0.95);
        final_color.x += spark_intensity * spark_color.x * 0.5;
        final_color.y += spark_intensity * spark_color.y * 0.5;
        final_color.z += spark_intensity * spark_color.z * 0.5;
    }

    // === LIMB EFFECTS ===
    let view_dir = Vec3::new(-pos.x, -pos.y, -pos.z).normalize();
    let edge_factor = 1.0 - (normal.x * view_dir.x + normal.y * view_dir.y + normal.z * view_dir.z).max(0.0);
    let edge_glow = edge_factor.powf(1.8) * 0.85;

    let edge_color = mix_v3(outer_pink, deep_pink, edge_factor * 0.5);
    final_color.x += edge_glow * edge_color.x;
    final_color.y += edge_glow * edge_color.y;
    final_color.z += edge_glow * edge_color.z;

    // === YELLOW SOLAR FLARES ===
    if flare_intensity > 0.1 {
        final_color.x += flare_intensity * flare_color.x * 1.2;
        final_color.y += flare_intensity * flare_color.y * 1.2;
        final_color.z += flare_intensity * flare_color.z * 1.2;
    }

    // === GLOBAL PULSATION ===
    final_color.x *= 0.7 + pulse_combined * 0.3;
    final_color.y *= 0.7 + pulse_combined * 0.3;
    final_color.z *= 0.7 + pulse_combined * 0.3;

    clamp_color(final_color.x, final_color.y, final_color.z)
}