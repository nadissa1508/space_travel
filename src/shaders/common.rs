use crate::math::Vec3;

// === FUNCIONES HELPER (Portadas de GLSL) ===

/// Smoothstep - transición suave entre 0 y 1
#[inline]
pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    if edge0 >= edge1 {
        return x.clamp(0.0, 1.0);
    }
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Mix - interpolación lineal (GLSL style)
#[inline]
pub fn mix(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Mix para Vec3
#[inline]
pub fn mix_v3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    Vec3::new(
        a.x + (b.x - a.x) * t,
        a.y + (b.y - a.y) * t,
        a.z + (b.z - a.z) * t,
    )
}

/// Fract - parte fraccionaria
#[inline]
pub fn fract(x: f32) -> f32 {
    x - x.floor()
}

/// Fract para Vec3
#[inline]
pub fn fract_v3(v: Vec3) -> Vec3 {
    Vec3::new(fract(v.x), fract(v.y), fract(v.z))
}

/// Hash function (de GLSL shader para mejor calidad de ruido)
pub fn hash_v3(p: Vec3) -> f32 {
    let mut p = fract_v3(Vec3::new(
        p.x * 0.3183099 + 0.1,
        p.y * 0.3183099 + 0.1,
        p.z * 0.3183099 + 0.1,
    ));
    p.x *= 17.0;
    p.y *= 17.0;
    p.z *= 17.0;
    fract(p.x * p.y * p.z * (p.x + p.y + p.z))
}

/// Ruido 3D mejorado usando hash (de GLSL shader)
pub fn noise_3d(x: Vec3) -> f32 {
    let i = Vec3::new(x.x.floor(), x.y.floor(), x.z.floor());
    let f = fract_v3(x);
    let f = Vec3::new(
        f.x * f.x * (3.0 - 2.0 * f.x),
        f.y * f.y * (3.0 - 2.0 * f.y),
        f.z * f.z * (3.0 - 2.0 * f.z),
    );

    mix(
        mix(
            mix(
                hash_v3(Vec3::new(i.x, i.y, i.z)),
                hash_v3(Vec3::new(i.x + 1.0, i.y, i.z)),
                f.x,
            ),
            mix(
                hash_v3(Vec3::new(i.x, i.y + 1.0, i.z)),
                hash_v3(Vec3::new(i.x + 1.0, i.y + 1.0, i.z)),
                f.x,
            ),
            f.y,
        ),
        mix(
            mix(
                hash_v3(Vec3::new(i.x, i.y, i.z + 1.0)),
                hash_v3(Vec3::new(i.x + 1.0, i.y, i.z + 1.0)),
                f.x,
            ),
            mix(
                hash_v3(Vec3::new(i.x, i.y + 1.0, i.z + 1.0)),
                hash_v3(Vec3::new(i.x + 1.0, i.y + 1.0, i.z + 1.0)),
                f.x,
            ),
            f.y,
        ),
        f.z,
    )
}

/// FBM usando el ruido mejorado (de GLSL shader)
pub fn fbm_3d(p: Vec3, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    let mut frequency = 1.0;

    for _ in 0..octaves {
        value += amplitude * noise_3d(Vec3::new(
            p.x * frequency,
            p.y * frequency,
            p.z * frequency,
        ));
        frequency *= 2.0;
        amplitude *= 0.5;
    }

    value
}

/// FBM legacy (2D, usado por Lava/Ice)
pub fn fbm(x: f32, y: f32, octaves: i32) -> f32 {
    fbm_3d(Vec3::new(x, y, 0.0), octaves)
}

/// Clamp de color a rango válido y convierte a tupla
#[inline]
pub fn clamp_color(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    (r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0))
}

/// Vec3 a tupla de color
#[inline]
pub fn v3_to_color(v: Vec3) -> (f32, f32, f32) {
    clamp_color(v.x, v.y, v.z)
}