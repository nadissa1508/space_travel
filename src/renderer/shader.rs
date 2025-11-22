use crate::math::Vec3;

/// Resultado de un shader de fragmento
#[derive(Debug, Clone, Copy)]
pub struct FragmentOutput {
    pub color: (f32, f32, f32),
    pub discard: bool,
}

impl FragmentOutput {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            color: (r, g, b),
            discard: false,
        }
    }

    pub fn discard() -> Self {
        Self {
            color: (0.0, 0.0, 0.0),
            discard: true,
        }
    }
}

/// Shader de color sólido (sin iluminación)
pub fn flat_shader(base_color: (f32, f32, f32)) -> FragmentOutput {
    FragmentOutput::new(base_color.0, base_color.1, base_color.2)
}

/// Shader difuso simple (Lambertian)
pub fn diffuse_shader(
    base_color: (f32, f32, f32),
    normal: Vec3,
    light_dir: Vec3,
    ambient: f32,
) -> FragmentOutput {
    let n = normal.normalize();
    let l = light_dir.normalize();
    
    // Componente difusa
    let diffuse = n.dot(&l).max(0.0);
    
    // Combinar ambiente + difusa
    let intensity = ambient + (1.0 - ambient) * diffuse;
    
    FragmentOutput::new(
        (base_color.0 * intensity).min(1.0),
        (base_color.1 * intensity).min(1.0),
        (base_color.2 * intensity).min(1.0),
    )
}

/// Shader emisivo (para el sol)
pub fn emissive_shader(
    base_color: (f32, f32, f32),
    intensity: f32,
) -> FragmentOutput {
    FragmentOutput::new(
        (base_color.0 * intensity).min(1.0),
        (base_color.1 * intensity).min(1.0),
        (base_color.2 * intensity).min(1.0),
    )
}

/// Shader con efecto de borde (glow) para el sol
pub fn sun_glow_shader(
    base_color: (f32, f32, f32),
    normal: Vec3,
    view_dir: Vec3,
) -> FragmentOutput {
    let n = normal.normalize();
    let v = view_dir.normalize();
    
    // Fresnel-like effect (más brillante en los bordes)
    let edge_factor = 1.0 - n.dot(&v).abs();
    let glow = 1.0 + edge_factor * 0.5;
    
    FragmentOutput::new(
        (base_color.0 * glow).min(1.0),
        (base_color.1 * glow).min(1.0),
        (base_color.2 * glow).min(1.0),
    )
}

/// Shader con specular highlight (Blinn-Phong simplificado)
pub fn specular_shader(
    base_color: (f32, f32, f32),
    normal: Vec3,
    light_dir: Vec3,
    view_dir: Vec3,
    ambient: f32,
    shininess: f32,
) -> FragmentOutput {
    let n = normal.normalize();
    let l = light_dir.normalize();
    let v = view_dir.normalize();
    
    // Difusa
    let diffuse = n.dot(&l).max(0.0);
    
    // Specular (Blinn-Phong)
    let half_vec = (l + v).normalize();
    let specular = n.dot(&half_vec).max(0.0).powf(shininess);
    
    let intensity = ambient + (1.0 - ambient) * diffuse;
    
    FragmentOutput::new(
        (base_color.0 * intensity + specular * 0.3).min(1.0),
        (base_color.1 * intensity + specular * 0.3).min(1.0),
        (base_color.2 * intensity + specular * 0.3).min(1.0),
    )
}