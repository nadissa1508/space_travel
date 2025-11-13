// shaders.rs - Sistema de Shaders Planetarios Procedurales
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::Uniforms;
use raylib::prelude::*;

#[derive(Copy, Clone, PartialEq)]
pub enum ShaderType {
    Rocky,      // Planeta rocoso tipo Marte
    GasGiant,   // Gigante gaseoso tipo Júpiter
    Lava,       // Planeta volcánico (extra 1)
    Ice,        // Planeta helado (extra 2)
    Alien,      // Planeta alien bioluminiscente (extra 3)
    SolarHeart, // Estrella con patrón de corazón (Lab 5)
    Rings,      // Anillos planetarios
    Moon,       // Luna
}

// --- Shader helper functions (ported from GLSL) ---
fn normalize_v(v: Vector3) -> Vector3 {
    let len = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if len > 0.0 {
        Vector3::new(v.x / len, v.y / len, v.z / len)
    } else {
        v
    }
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    if edge0 >= edge1 {
        return x.clamp(0.0, 1.0);
    }
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

// GLSL-style mix (linear interpolation)
fn mix(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn mix_v3(a: Vector3, b: Vector3, t: f32) -> Vector3 {
    Vector3::new(
        a.x + (b.x - a.x) * t,
        a.y + (b.y - a.y) * t,
        a.z + (b.z - a.z) * t,
    )
}

// GLSL-style fract
fn fract(x: f32) -> f32 {
    x - x.floor()
}

fn fract_v3(v: Vector3) -> Vector3 {
    Vector3::new(fract(v.x), fract(v.y), fract(v.z))
}

// Hash function from GLSL shader (for better noise quality)
fn hash_v3(p: Vector3) -> f32 {
    let mut p = fract_v3(Vector3::new(
        p.x * 0.3183099 + 0.1,
        p.y * 0.3183099 + 0.1,
        p.z * 0.3183099 + 0.1,
    ));
    p.x *= 17.0;
    p.y *= 17.0;
    p.z *= 17.0;
    fract(p.x * p.y * p.z * (p.x + p.y + p.z))
}

// Improved 3D noise using hash (from GLSL shader)
fn noise_3d(x: Vector3) -> f32 {
    let i = Vector3::new(x.x.floor(), x.y.floor(), x.z.floor());
    let f = fract_v3(x);
    let f = Vector3::new(
        f.x * f.x * (3.0 - 2.0 * f.x),
        f.y * f.y * (3.0 - 2.0 * f.y),
        f.z * f.z * (3.0 - 2.0 * f.z),
    );

    mix(
        mix(
            mix(
                hash_v3(Vector3::new(i.x, i.y, i.z)),
                hash_v3(Vector3::new(i.x + 1.0, i.y, i.z)),
                f.x,
            ),
            mix(
                hash_v3(Vector3::new(i.x, i.y + 1.0, i.z)),
                hash_v3(Vector3::new(i.x + 1.0, i.y + 1.0, i.z)),
                f.x,
            ),
            f.y,
        ),
        mix(
            mix(
                hash_v3(Vector3::new(i.x, i.y, i.z + 1.0)),
                hash_v3(Vector3::new(i.x + 1.0, i.y, i.z + 1.0)),
                f.x,
            ),
            mix(
                hash_v3(Vector3::new(i.x, i.y + 1.0, i.z + 1.0)),
                hash_v3(Vector3::new(i.x + 1.0, i.y + 1.0, i.z + 1.0)),
                f.x,
            ),
            f.y,
        ),
        f.z,
    )
}

// FBM using the improved noise (from GLSL shader)
fn fbm_3d(p: Vector3, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    let mut frequency = 1.0;

    for _ in 0..octaves {
        value += amplitude
            * noise_3d(Vector3::new(
                p.x * frequency,
                p.y * frequency,
                p.z * frequency,
            ));
        frequency *= 2.0;
        amplitude *= 0.5;
    }

    value
}

// Helper to get a stable surface sample position for noise/patterns.
// Adds a tiny offset along per-pixel normal to avoid sampling exactly on the surface.
fn sample_surface_pos(fragment: &Fragment) -> Vector3 {
    // The world_position is already on the surface, and normal is already normalized
    // We can use it directly for noise sampling without additional normalization
    // since we're just using it for procedural coordinates
    fragment.world_position
}

fn multiply_matrix_vector4(matrix: &Matrix, vector: &Vector4) -> Vector4 {
    Vector4::new(
        matrix.m0 * vector.x + matrix.m4 * vector.y + matrix.m8 * vector.z + matrix.m12 * vector.w,
        matrix.m1 * vector.x + matrix.m5 * vector.y + matrix.m9 * vector.z + matrix.m13 * vector.w,
        matrix.m2 * vector.x + matrix.m6 * vector.y + matrix.m10 * vector.z + matrix.m14 * vector.w,
        matrix.m3 * vector.x + matrix.m7 * vector.y + matrix.m11 * vector.z + matrix.m15 * vector.w,
    )
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position_vec4 = Vector4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);

    // Deformación procedimental para anillos
    let mut deformed_pos = position_vec4;

    if uniforms.shader_type == ShaderType::Rings {
        // Convertir esfera en anillo aplanado
        let radius =
            (vertex.position.x * vertex.position.x + vertex.position.z * vertex.position.z).sqrt();
        let ring_thickness = 0.15;

        // Aplanar en Y y expandir en XZ
        deformed_pos.x = vertex.position.x * (1.0 + radius * 0.3);
        deformed_pos.y = vertex.position.y * ring_thickness;
        deformed_pos.z = vertex.position.z * (1.0 + radius * 0.3);
    } else if uniforms.shader_type == ShaderType::SolarHeart {
        // Desplazamiento de vértices para Solar Heart (breathing/pulsation)
        let time = uniforms.time;

        // Pulsación de respiración global
        let pulse = (time * 1.0).sin() * 0.5 + 0.5; // Main breathing
        let micro_pulse = (time * 3.0).sin() * 0.5 + 0.5; // Micro pulsation

        // Ruido para movimiento orgánico de plasma
        let noise_scale = 8.0;
        let noise_sample = Vector3::new(
            vertex.position.x * noise_scale + time * 0.15,
            vertex.position.y * noise_scale + time * 0.12,
            vertex.position.z * noise_scale + time * 0.18,
        );
        let displacement_noise = fbm_3d(noise_sample, 3);

        // Combinar pulsación y ruido para desplazamiento
        let displacement_amount = 0.05 * pulse + 0.02 * micro_pulse + 0.03 * displacement_noise;

        // Desplazar a lo largo de la normal (efecto de respiración + turbulencia)
        deformed_pos.x = vertex.position.x + vertex.normal.x * displacement_amount;
        deformed_pos.y = vertex.position.y + vertex.normal.y * displacement_amount;
        deformed_pos.z = vertex.position.z + vertex.normal.z * displacement_amount;
    }

    let world_position = multiply_matrix_vector4(&uniforms.model_matrix, &deformed_pos);
    let view_position = multiply_matrix_vector4(&uniforms.view_matrix, &world_position);
    let clip_position = multiply_matrix_vector4(&uniforms.projection_matrix, &view_position);

    // Transform normal correctly (approximate using model matrix; use inverse-transpose if non-uniform scale)
    let normal_vec4 = Vector4::new(vertex.normal.x, vertex.normal.y, vertex.normal.z, 0.0);
    let world_normal4 = multiply_matrix_vector4(&uniforms.model_matrix, &normal_vec4);
    let transformed_normal_vec3 = normalize_v(Vector3::new(
        world_normal4.x,
        world_normal4.y,
        world_normal4.z,
    ));

    let ndc = if clip_position.w != 0.0 {
        Vector3::new(
            clip_position.x / clip_position.w,
            clip_position.y / clip_position.w,
            clip_position.z / clip_position.w,
        )
    } else {
        Vector3::new(clip_position.x, clip_position.y, clip_position.z)
    };

    let ndc_vec4 = Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
    let screen_position = multiply_matrix_vector4(&uniforms.viewport_matrix, &ndc_vec4);

    let transformed_position =
        Vector3::new(screen_position.x, screen_position.y, screen_position.z);

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position,
        transformed_normal: transformed_normal_vec3, // use transformed normal
        deformation_factor: vertex.deformation_factor,
    }
}

// Legacy fbm wrapper for backward compatibility (used by Lava/Ice shaders)
fn fbm(x: f32, y: f32, octaves: i32) -> f32 {
    fbm_3d(Vector3::new(x, y, 0.0), octaves)
}

// Heart-shaped Signed Distance Function (SDF)
// Returns distance to heart shape boundary (0 = on boundary, <0 = inside, >0 = outside)
fn heart_sdf(p: Vector2) -> f32 {
    let x = p.x;
    let y = p.y;

    // Heart equation: (x^2 + y^2 - 1)^3 - x^2*y^3 = 0
    // Modified for better shape
    let a = x * x + y * y - 1.0;
    a * a * a - x * x * y * y * y
}

// Generate heart pattern intensity at given 2D position
// Returns 0.0-1.0 where 1.0 is on the heart boundary
fn heart_pattern(p: Vector2, thickness: f32) -> f32 {
    let d = heart_sdf(p);
    // Convert distance to intensity (glow around heart shape)
    let intensity = 1.0 - (d.abs() / thickness).clamp(0.0, 1.0);
    smoothstep(0.0, 1.0, intensity)
}

// ============================================
// SHADER 1: PLANETA ROCOSO (Ported from GLSL)
// ============================================
fn shader_rocky_planet(fragment: &Fragment, _time: f32) -> Vector3 {
    let pos = sample_surface_pos(fragment);
    let normal = normalize_v(fragment.normal);
    let light_dir = normalize_v(Vector3::new(1.0, 1.0, 2.0));

    // Calculate noise-based terrain (exactly like GLSL version)
    let terrain = fbm_3d(Vector3::new(pos.x * 3.0, pos.y * 3.0, pos.z * 3.0), 5);
    let craters = fbm_3d(Vector3::new(pos.x * 8.0, pos.y * 8.0, pos.z * 8.0), 5);

    // Color variation based on terrain height (light blue palette)
    let color1 = Vector3::new(0.4, 0.6, 0.8); // Light blue
    let color2 = Vector3::new(0.6, 0.75, 0.9); // Lighter blue
    let color3 = Vector3::new(0.5, 0.65, 0.85); // Medium light blue

    let mut base_color = mix_v3(color1, color2, terrain);
    base_color = mix_v3(base_color, color3, craters * 0.5);

    // Add rocky variation
    let rock_detail = noise_3d(Vector3::new(pos.x * 20.0, pos.y * 20.0, pos.z * 20.0));
    base_color.x += rock_detail * 0.1;
    base_color.y += rock_detail * 0.1;
    base_color.z += rock_detail * 0.1;

    // Lighting (diffuse + ambient like GLSL)
    let diffuse =
        (normal.x * light_dir.x + normal.y * light_dir.y + normal.z * light_dir.z).max(0.0);
    let ambient = 0.3;

    let final_color = Vector3::new(
        base_color.x * (ambient + diffuse * 0.7),
        base_color.y * (ambient + diffuse * 0.7),
        base_color.z * (ambient + diffuse * 0.7),
    );

    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}

// ============================================
// SHADER 2: GIGANTE GASEOSO (Ported from GLSL)
// ============================================
fn shader_gas_giant(fragment: &Fragment, time: f32) -> Vector3 {
    let pos = sample_surface_pos(fragment);
    let normal = normalize_v(fragment.normal);
    let light_dir = normalize_v(Vector3::new(1.0, 1.0, 2.0));

    // Create horizontal bands based on Y position (like GLSL)
    let mut bands = pos.y * 5.0 + time * 0.2;

    // Add swirling effect with noise
    let swirl = noise_3d(Vector3::new(
        pos.x * 2.0,
        pos.y * 8.0 + time * 0.1,
        pos.z * 2.0,
    ));
    bands += swirl * 2.0;

    // More turbulence
    let turbulence = noise_3d(Vector3::new(
        pos.x * 6.0 + time * 0.15,
        pos.y * 15.0,
        pos.z * 6.0,
    ));
    bands += turbulence * 0.8;

    // Color bands (Jupiter-like colors from GLSL)
    let color1 = Vector3::new(0.9, 0.7, 0.5); // Light orange
    let color2 = Vector3::new(0.7, 0.5, 0.3); // Dark orange
    let color3 = Vector3::new(0.95, 0.85, 0.7); // Cream
    let color4 = Vector3::new(0.6, 0.4, 0.25); // Brown

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
    let view_dir = Vector3::new(0.0, 0.0, 1.0);
    let atmosphere = (1.0
        - (normal.x * view_dir.x + normal.y * view_dir.y + normal.z * view_dir.z).abs())
    .powf(2.0);
    let mut final_color = Vector3::new(
        base_color.x + 0.1 * atmosphere,
        base_color.y + 0.05 * atmosphere,
        base_color.z,
    );

    // Lighting (like GLSL)
    let diffuse =
        (normal.x * light_dir.x + normal.y * light_dir.y + normal.z * light_dir.z).max(0.0);
    let ambient = 0.4;

    final_color.x *= ambient + diffuse * 0.6;
    final_color.y *= ambient + diffuse * 0.6;
    final_color.z *= ambient + diffuse * 0.6;

    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}

// ============================================
// SHADER 3: PLANETA DE LAVA (Extra 1) - ORANGE AND RED with SHADOWS
// ============================================
fn shader_lava_planet(fragment: &Fragment, time: f32) -> Vector3 {
    let pos = sample_surface_pos(fragment);
    let normal = normalize_v(fragment.normal);
    let light_dir = normalize_v(Vector3::new(1.0, 1.0, 2.0));

    // Lava flow pattern with animation
    let flow1 = fbm(pos.x * 2.0 + time * 0.3, pos.z * 2.0 + time * 0.25, 3);
    let flow2 = fbm(pos.x * 3.5 - time * 0.15, pos.y * 3.5, 2);
    let lava_flow = (flow1 * 0.7 + flow2 * 0.3).clamp(0.0, 1.0);

    // Define colors with BLACK shadows for contrast
    let black_crust = Vector3::new(0.0, 0.0, 0.0); // Pure black shadows
    let dark_crust = Vector3::new(0.1, 0.05, 0.0); // Very dark crust
    let dark_red = Vector3::new(0.8, 0.1, 0.0); // Deep red
    let bright_red = Vector3::new(1.0, 0.2, 0.0); // Bright red
    let hot_orange = Vector3::new(1.0, 0.6, 0.0); // Hot orange
    let yellow_hot = Vector3::new(1.0, 0.9, 0.2); // Yellow-hot

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
    let crack_color = Vector3::new(1.0, 0.5, 0.0); // Orange cracks
    let color_with_cracks = mix_v3(base_color, crack_color, crack_intensity * is_lava * 0.5);

    // Heat pulse (breathing effect)
    let pulse = (time * 1.5).sin() * 0.5 + 0.5;
    let heat_boost = pulse * is_lava * 0.15;

    // Lighting for shadows on dark crust
    let diffuse = (normal.x * light_dir.x + normal.y * light_dir.y + normal.z * light_dir.z).max(0.0);
    let shadow_factor = 0.1 + diffuse * 0.3; // Dark shadows

    // Apply shadows only to crust (not lava - it emits light)
    let crust_with_shadow = Vector3::new(
        color_with_cracks.x * (is_lava + (1.0 - is_lava) * shadow_factor),
        color_with_cracks.y * (is_lava + (1.0 - is_lava) * shadow_factor),
        color_with_cracks.z * (is_lava + (1.0 - is_lava) * shadow_factor),
    );

    // Add emission glow
    let final_color = Vector3::new(
        (crust_with_shadow.x + heat_boost).clamp(0.0, 1.0),
        (crust_with_shadow.y + heat_boost * 0.5).clamp(0.0, 1.0),
        (crust_with_shadow.z * 0.3).clamp(0.0, 1.0),
    );

    final_color
}

// ============================================
// SHADER 4: PLANETA DE HIELO (Extra 2)
// ============================================
fn shader_ice_planet(fragment: &Fragment, time: f32) -> Vector3 {
    let pos = sample_surface_pos(fragment);
    let base_lighting = fragment.color;

    // CAPA 1: Base de hielo con variaciones
    let ice_variation = fbm(pos.x * 3.0, pos.z * 3.0, 3);
    let pure_ice = Vector3::new(0.85, 0.92, 1.0);
    let glacier_ice = Vector3::new(0.70, 0.82, 0.95);
    let ice_base = Vector3::new(
        pure_ice.x + (glacier_ice.x - pure_ice.x) * ice_variation,
        pure_ice.y + (glacier_ice.y - pure_ice.y) * ice_variation,
        pure_ice.z + (glacier_ice.z - pure_ice.z) * ice_variation,
    );

    // CAPA 2: Grietas profundas en hielo con suavizado
    let crack_large = fbm(pos.x * 8.0, pos.z * 8.0, 4);
    let crack_small = fbm(pos.x * 18.0, pos.z * 18.0, 2);
    let crack_pattern =
        smoothstep(0.35, 0.45, crack_large) * 0.7 + smoothstep(0.38, 0.42, crack_small) * 0.3;
    let deep_ice = Vector3::new(0.25, 0.45, 0.65);

    // CAPA 3: Capas de nieve brillante
    let snow_pattern = fbm(pos.x * 5.0, pos.y * 5.0, 3);
    let snow_coverage = smoothstep(0.45, 0.65, snow_pattern);
    let fresh_snow = Vector3::new(0.98, 0.99, 1.0);

    // CAPA 4: Cristales de hielo suaves (sparkle)
    let crystals = fbm(pos.x * 10.0 + time * 0.08, pos.z * 10.0, 2);
    let sparkle = smoothstep(0.75, 0.88, crystals) * ((time * 2.5).sin() * 0.5 + 0.5) * 0.5;

    // CAPA 5: Auroras boreales mejoradas con múltiples colores
    let aurora_flow1 = ((pos.x * 3.0 + time * 0.5).sin() * (pos.z * 2.0 + time * 0.3).cos()).abs();
    let aurora_flow2 = ((pos.x * 4.0 - time * 0.4).cos() * (pos.z * 3.0).sin()).abs();

    // Intensidad basada en latitud (más fuerte en polos)
    let polar_intensity = ((pos.y.abs() - 0.3).max(0.0) / 0.7).powf(1.5);
    let aurora_intensity = (aurora_flow1 * 0.6 + aurora_flow2 * 0.4) * polar_intensity;

    // Colores de aurora que cambian con el tiempo
    let time_shift = time * 0.2;
    let aurora_green = Vector3::new(0.1, 0.9, 0.5);
    let aurora_cyan = Vector3::new(0.2, 0.7, 0.9);
    let aurora_purple = Vector3::new(0.6, 0.3, 0.9);

    let aurora_mix = time_shift.sin() * 0.5 + 0.5;
    let aurora_color = if aurora_mix < 0.33 {
        aurora_green
    } else if aurora_mix < 0.66 {
        aurora_cyan
    } else {
        aurora_purple
    };

    // CAPA 6: Atmósfera helada (rim glow)
    let view_dir = normalize_v(Vector3::new(
        -fragment.world_position.x,
        -fragment.world_position.y,
        -fragment.world_position.z,
    ));
    let n = normalize_v(fragment.normal);
    let rim =
        (1.0 - (n.x * view_dir.x + n.y * view_dir.y + n.z * view_dir.z).max(0.0)).powf(2.0) * 0.3;

    // Combinar capas
    let mut final_color = Vector3::new(
        ice_base.x * (1.0 - crack_pattern) + deep_ice.x * crack_pattern,
        ice_base.y * (1.0 - crack_pattern) + deep_ice.y * crack_pattern,
        ice_base.z * (1.0 - crack_pattern) + deep_ice.z * crack_pattern,
    );

    // Blend con nieve
    final_color.x = final_color.x * (1.0 - snow_coverage) + fresh_snow.x * snow_coverage;
    final_color.y = final_color.y * (1.0 - snow_coverage) + fresh_snow.y * snow_coverage;
    final_color.z = final_color.z * (1.0 - snow_coverage) + fresh_snow.z * snow_coverage;

    // Iluminación especular fuerte (hielo muy reflectivo)
    let specular = base_lighting.x.powf(4.0) * 0.5;
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
    let atmo_ice = Vector3::new(0.7, 0.85, 1.0);
    final_color.x += atmo_ice.x * rim;
    final_color.y += atmo_ice.y * rim;
    final_color.z += atmo_ice.z * rim;

    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}

// ============================================
// SHADER 5: PLANETA ALIEN HOLOGRÁFICO - Purple, Pink & Aqua
// ============================================
fn shader_alien_planet(fragment: &Fragment, time: f32) -> Vector3 {
    let pos = sample_surface_pos(fragment);
    let normal = normalize_v(fragment.normal);

    // Base dark purple
    let dark_base = Vector3::new(0.08, 0.02, 0.15);

    // Holographic pulses (multiple frequencies for shimmer effect)
    let pulse_fast = (time * 3.0).sin() * 0.5 + 0.5;
    let pulse_slow = (time * 1.5).sin() * 0.5 + 0.5;
    let pulse_shimmer = (time * 5.0 + pos.x * 10.0).sin() * 0.5 + 0.5;

    // Holographic stripe pattern (like interference patterns)
    let stripe_pattern = ((pos.y * 15.0 + time * 2.0).sin() * (pos.x * 10.0).cos()).abs();
    let stripes = smoothstep(0.3, 0.7, stripe_pattern);

    // Flowing energy veins
    let vein_sample = Vector3::new(pos.x * 6.0 + time * 0.4, pos.y * 6.0, pos.z * 6.0);
    let veins = fbm_3d(vein_sample, 3);
    let vein_intensity = smoothstep(0.55, 0.75, veins);

    // Iridescent spots (color-shifting)
    let spots_sample = Vector3::new(pos.x * 12.0 + time * 0.3, pos.y * 12.0, pos.z * 12.0);
    let spots = noise_3d(spots_sample);
    let spot_intensity = smoothstep(0.65, 0.80, spots);

    // Holographic colors - Purple, Pink, Aqua
    let vivid_purple = Vector3::new(0.7, 0.1, 0.9); // Vivid purple
    let hot_pink = Vector3::new(1.0, 0.2, 0.7); // Hot pink
    let electric_aqua = Vector3::new(0.1, 0.9, 0.9); // Electric aqua
    let neon_magenta = Vector3::new(0.9, 0.0, 0.8); // Neon magenta

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
    final_color = mix_v3(
        final_color,
        holographic_color,
        stripes * pulse_shimmer * 0.6,
    );

    // Add energy veins with pink/magenta
    final_color = mix_v3(final_color, neon_magenta, vein_intensity * pulse_fast * 0.5);

    // Add iridescent spots with aqua
    final_color = mix_v3(
        final_color,
        electric_aqua,
        spot_intensity * pulse_slow * 0.4,
    );

    // Holographic rim glow (rainbow-like edge)
    let view_dir = normalize_v(Vector3::new(
        -fragment.world_position.x,
        -fragment.world_position.y,
        -fragment.world_position.z,
    ));
    let fresnel = (1.0
        - (normal.x * view_dir.x + normal.y * view_dir.y + normal.z * view_dir.z).max(0.0))
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

    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}

// ============================================
// SHADER 6: SOLAR HEART - Heart-Patterned Star (Lab 5 - Creative)
// ============================================
fn shader_solar_heart(fragment: &Fragment, time: f32) -> Vector3 {
    let pos = sample_surface_pos(fragment);
    let normal = normalize_v(fragment.normal);

    // === GLOBAL PULSATION (Breathing Effect) ===
    // Two pulse frequencies for organic feel
    let pulse_slow = (time * 1.0).sin() * 0.5 + 0.5;      // Main breathing (1 Hz)
    let pulse_fast = (time * 3.0).sin() * 0.5 + 0.5;      // Rapid flutter (3 Hz)
    let pulse_heart = (time * 0.7).sin() * 0.5 + 0.5;     // Heart expansion rate
    let pulse_combined = pulse_slow * 0.6 + pulse_fast * 0.4;

    // === CAPA 1: BASE TURBULENCE (Perlin/Simplex Noise) ===
    // Organic plasma movement
    let turbulence_scale = 10.0;
    let turbulence = fbm_3d(
        Vector3::new(
            pos.x * turbulence_scale + time * 0.2,
            pos.y * turbulence_scale - time * 0.15,
            pos.z * turbulence_scale + time * 0.18,
        ),
        4,
    );

    // === CAPA 2: HIGH-ENERGY REGIONS (Cellular/fBm Noise) ===
    // Active zones with higher emission
    let energy_scale = 15.0;
    let energy_zones = fbm_3d(
        Vector3::new(
            pos.x * energy_scale + time * 0.35,
            pos.y * energy_scale,
            pos.z * energy_scale - time * 0.3,
        ),
        5,
    );
    let energy_intensity = smoothstep(0.55, 0.75, energy_zones);

    // === CAPA 3: SOLAR SPARKS (Pseudo-random bright spots) ===
    // Generate random sparks using hash function - REDUCED for smooth surface
    let spark_pos = Vector3::new(
        pos.x * 20.0 + time * 0.5,  // Reduced frequency and speed
        pos.y * 20.0 + time * 0.4,
        pos.z * 20.0 + time * 0.45,
    );
    let spark_value = hash_v3(spark_pos);
    let spark_threshold = 0.98; // Only brightest 2% become sparks (reduced from 5%)
    // Smooth spark transition to avoid "acne" artifacts
    let spark_raw = if spark_value > spark_threshold {
        (spark_value - spark_threshold) / (1.0 - spark_threshold)
    } else {
        0.0
    };
    let spark_intensity = smoothstep(0.0, 1.0, spark_raw) * pulse_fast * 0.6; // Smoothed and reduced

    // === CAPA 4: HEART PATTERN EXPANSION ===
    // Create expanding heart waves from center
    // Map 3D sphere position to 2D for heart SDF
    let radius_from_center = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();

    // Project to tangent plane for heart pattern
    // Use spherical coordinates approximation
    let heart_scale = 2.5;
    let heart_2d = Vector2::new(pos.x * heart_scale, pos.y * heart_scale);

    // Multiple expanding waves (3 waves at different phases)
    let wave_speed = 0.5;
    let wave_frequency = 2.0;

    let wave1_phase = (time * wave_speed) % wave_frequency;
    let wave2_phase = (time * wave_speed + 0.66) % wave_frequency;
    let wave3_phase = (time * wave_speed + 1.33) % wave_frequency;

    // Scale hearts based on wave expansion
    let heart1_scale = 0.5 + wave1_phase * 1.5;
    let heart2_scale = 0.5 + wave2_phase * 1.5;
    let heart3_scale = 0.5 + wave3_phase * 1.5;

    let heart1 = heart_pattern(
        Vector2::new(heart_2d.x / heart1_scale, heart_2d.y / heart1_scale),
        0.8,
    ) * (1.0 - wave1_phase / wave_frequency); // Fade as it expands

    let heart2 = heart_pattern(
        Vector2::new(heart_2d.x / heart2_scale, heart_2d.y / heart2_scale),
        0.8,
    ) * (1.0 - wave2_phase / wave_frequency);

    let heart3 = heart_pattern(
        Vector2::new(heart_2d.x / heart3_scale, heart_2d.y / heart3_scale),
        0.8,
    ) * (1.0 - wave3_phase / wave_frequency);

    let heart_intensity = (heart1 + heart2 + heart3).clamp(0.0, 1.0) * pulse_heart;

    // === CAPA 5: YELLOW SOLAR FLARES (Around the edge) ===
    // Create dynamic solar prominences/flares at the limb
    let flare_scale = 8.0;
    let flare_noise = fbm_3d(
        Vector3::new(
            pos.x * flare_scale + time * 0.4,
            pos.y * flare_scale - time * 0.3,
            pos.z * flare_scale + time * 0.35,
        ),
        4,
    );

    // Additional high-frequency detail for flare texture
    let flare_detail = noise_3d(Vector3::new(
        pos.x * 15.0 + time * 0.6,
        pos.y * 15.0,
        pos.z * 15.0 - time * 0.5,
    ));

    // Edge detection for limb (where flares appear)
    let normal_view_dot = (normal.x * (-pos.x) + normal.y * (-pos.y) + normal.z * (-pos.z))
        / ((pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt() + 0.001);
    let is_at_edge = 1.0 - normal_view_dot.abs();

    // Flare only appears at edges where noise is high
    let flare_threshold = 0.55;
    let flare_mask = smoothstep(flare_threshold, flare_threshold + 0.15, flare_noise);

    // Combine edge detection + noise mask + detail
    let flare_intensity = (is_at_edge.powf(2.5) * flare_mask * (0.7 + flare_detail * 0.3) * pulse_fast)
        .clamp(0.0, 1.0);

    // Yellow-orange colors for solar flares
    let flare_bright_yellow = Vector3::new(1.0, 0.95, 0.3);   // Bright yellow
    let flare_orange = Vector3::new(1.0, 0.7, 0.2);           // Orange
    let flare_color = mix_v3(flare_orange, flare_bright_yellow, flare_detail);

    // === RADIAL GRADIENT (Core → Middle → Outer) ===
    // Calculate distance from center for gradient
    let center_distance = radius_from_center;

    // Color zones - MORE PINK!
    let core_white = Vector3::new(1.0, 0.95, 0.98);       // Bright white with pink tint
    let core_yellow = Vector3::new(1.0, 0.85, 0.5);       // Yellow-pink (warmer)
    let middle_orange = Vector3::new(1.0, 0.45, 0.35);    // Orange-pink (more pink)
    let middle_red = Vector3::new(0.95, 0.25, 0.35);      // Red-pink (increased blue for pink)
    let outer_pink = Vector3::new(1.0, 0.2, 0.6);         // Vibrant hot pink
    let deep_pink = Vector3::new(0.95, 0.15, 0.5);        // Deep magenta-pink edge

    // Multi-zone gradient - pink throughout!
    let base_color = if center_distance < 0.3 {
        // Core: white-pink with pulsation
        mix_v3(core_yellow, core_white, (center_distance / 0.3) * pulse_combined)
    } else if center_distance < 0.5 {
        // Middle: yellow-pink to orange-pink
        let t = (center_distance - 0.3) / 0.2;
        mix_v3(core_yellow, middle_orange, t)
    } else if center_distance < 0.8 {
        // Outer middle: orange-pink to red-pink
        let t = (center_distance - 0.5) / 0.3;
        mix_v3(middle_orange, middle_red, t)
    } else if center_distance < 1.0 {
        // Edge: red-pink to hot pink
        let t = (center_distance - 0.8) / 0.2;
        mix_v3(middle_red, outer_pink, t)
    } else {
        // Very edge: hot pink to deep magenta
        let t = ((center_distance - 1.0) / 0.2).clamp(0.0, 1.0);
        mix_v3(outer_pink, deep_pink, t * turbulence)
    };

    // === COMBINE ALL LAYERS ===
    // Apply turbulence modulation to base color
    let mut final_color = Vector3::new(
        base_color.x * (0.8 + turbulence * 0.2),
        base_color.y * (0.8 + turbulence * 0.2),
        base_color.z * (0.8 + turbulence * 0.2),
    );

    // Add heart pattern (increases brightness and saturation)
    let heart_boost = heart_intensity * 0.8;
    final_color.x += heart_boost * outer_pink.x;
    final_color.y += heart_boost * outer_pink.y;
    final_color.z += heart_boost * outer_pink.z;

    // Add high-energy zones
    final_color.x += energy_intensity * 0.3 * core_yellow.x;
    final_color.y += energy_intensity * 0.3 * core_yellow.y;
    final_color.z += energy_intensity * 0.2;

    // Add solar sparks (pink-white flashes for consistency)
    if spark_intensity > 0.0 {
        let spark_color = Vector3::new(1.0, 0.85, 0.95); // Pink-white tint
        final_color.x += spark_intensity * spark_color.x * 0.5;
        final_color.y += spark_intensity * spark_color.y * 0.5;
        final_color.z += spark_intensity * spark_color.z * 0.5;
    }

    // === LIMB EFFECTS ===
    // Fresnel-like edge glow (INTENSE PINK AURA!)
    let view_dir = normalize_v(Vector3::new(
        -fragment.world_position.x,
        -fragment.world_position.y,
        -fragment.world_position.z,
    ));
    let edge_factor = 1.0 - (normal.x * view_dir.x + normal.y * view_dir.y + normal.z * view_dir.z).max(0.0);
    let edge_glow = edge_factor.powf(1.8) * 0.85; // Increased intensity and softer falloff

    // Mix of hot pink and deep pink for vibrant edge
    let edge_color = mix_v3(outer_pink, deep_pink, edge_factor * 0.5);
    final_color.x += edge_glow * edge_color.x;
    final_color.y += edge_glow * edge_color.y;
    final_color.z += edge_glow * edge_color.z;

    // === YELLOW SOLAR FLARES (Layered on top) ===
    // Add bright yellow flares around the edges
    if flare_intensity > 0.1 {
        // Strong additive blending for bright prominence effect
        final_color.x += flare_intensity * flare_color.x * 1.2;
        final_color.y += flare_intensity * flare_color.y * 1.2;
        final_color.z += flare_intensity * flare_color.z * 1.2;
    }

    // === GLOBAL PULSATION ON EMISSION ===
    final_color.x *= 0.7 + pulse_combined * 0.3;
    final_color.y *= 0.7 + pulse_combined * 0.3;
    final_color.z *= 0.7 + pulse_combined * 0.3;

    // === NO LIGHTING (Emissive star) ===
    Vector3::new(
        final_color.x.clamp(0.0, 1.0),
        final_color.y.clamp(0.0, 1.0),
        final_color.z.clamp(0.0, 1.0),
    )
}

// ============================================
// SHADER ESPECIAL: ANILLOS PLANETARIOS - Saturn Style
// ============================================
fn shader_rings(fragment: &Fragment, _time: f32) -> Vector3 {
    let pos = sample_surface_pos(fragment);

    // Calculate radial distance from center
    let radius = (pos.x * pos.x + pos.z * pos.z).sqrt();

    // Create concentric bands (Saturn-like)
    let band_freq = radius * 20.0;
    let bands = (band_freq.sin() * 0.5 + 0.5).powf(0.5);

    // Add finer detail
    let detail = (radius * 60.0).sin() * 0.5 + 0.5;

    // Base colors for gray rings (light / dark) - not too bright
    let light_color = Vector3::new(0.72, 0.72, 0.72); // Light gray (reduced)
    let dark_color = Vector3::new(0.45, 0.45, 0.45);  // Dark gray

    // Mix based on bands
    let base_color = mix_v3(dark_color, light_color, bands);

    // Reduce additive lighting that caused whites. Use multiplicative dampening.
    let brightness_base = 0.55;                 // lower base brightness
    let brightness_mod = brightness_base + detail * 0.15; // smaller detail influence
    let mut shaded = Vector3::new(
        (base_color.x * brightness_mod).clamp(0.0, 1.0),
        (base_color.y * brightness_mod).clamp(0.0, 1.0),
        (base_color.z * brightness_mod).clamp(0.0, 1.0),
    );

    // Slightly desaturate / pull towards mid-gray to avoid pure-white highlights
    let desat = 0.06;
    shaded = mix_v3(shaded, Vector3::new(0.5, 0.5, 0.5), desat);

    // Cassini Division (dark gap) but never fully black — cap maximum so no white
    let cassini_pos = 0.65;
    let cassini_width = 0.05;
    let dist_to_cassini = (radius - cassini_pos).abs();
    let gap_t = smoothstep(0.0, cassini_width, dist_to_cassini); // 0 inside gap -> 1 outside
    // map gap_t to brightness factor between 0.45 (in gap) and 0.85 (outside) - never 1.0
    let cassini_brightness = mix(0.45, 0.85, gap_t);

    let final_color = Vector3::new(
        (shaded.x * cassini_brightness).clamp(0.0, 1.0),
        (shaded.y * cassini_brightness).clamp(0.0, 1.0),
        (shaded.z * cassini_brightness).clamp(0.0, 1.0),
    );

    final_color
}

// ============================================
// SHADER ESPECIAL: LUNA (Smooth version)
// ============================================
fn shader_moon(fragment: &Fragment, _time: f32) -> Vector3 {
    let pos = sample_surface_pos(fragment);
    let normal = normalize_v(fragment.normal);
    let light_dir = normalize_v(Vector3::new(1.0, 1.0, 2.0));

    // Base gray with smooth variation
    let surface_variation = fbm_3d(Vector3::new(pos.x * 2.0, pos.y * 2.0, pos.z * 2.0), 3);
    let gray_base = 0.45 + surface_variation * 0.12;

    // Smooth craters
    let craters = fbm_3d(Vector3::new(pos.x * 4.0, pos.y * 4.0, pos.z * 4.0), 3);
    let crater_darkness = smoothstep(0.60, 0.75, craters) * 0.25;

    // Maria (dark zones)
    let maria = fbm_3d(Vector3::new(pos.x * 1.5, pos.y * 1.5, pos.z * 1.5), 2);
    let maria_darkness = smoothstep(0.30, 0.50, maria) * 0.2;

    // Combine
    let base_value = (gray_base - crater_darkness - maria_darkness).max(0.15);
    let base_color = Vector3::new(base_value, base_value, base_value);

    // Lighting
    let diffuse =
        (normal.x * light_dir.x + normal.y * light_dir.y + normal.z * light_dir.z).max(0.0);
    let ambient = 0.3;

    let final_color = Vector3::new(
        base_color.x * (ambient + diffuse * 0.7),
        base_color.y * (ambient + diffuse * 0.7),
        base_color.z * (ambient + diffuse * 0.7),
    );

    final_color
}

// ============================================
// FRAGMENT SHADER PRINCIPAL
// ============================================
pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let time = uniforms.time;

    match uniforms.shader_type {
        ShaderType::Rocky => shader_rocky_planet(fragment, time),
        ShaderType::GasGiant => shader_gas_giant(fragment, time),
        ShaderType::Lava => shader_lava_planet(fragment, time),
        ShaderType::Ice => shader_ice_planet(fragment, time),
        ShaderType::Alien => shader_alien_planet(fragment, time),
        ShaderType::SolarHeart => shader_solar_heart(fragment, time),
        ShaderType::Rings => shader_rings(fragment, time),
        ShaderType::Moon => shader_moon(fragment, time),
    }
}
