use super::vec3::Vec3;
use super::mat4::Mat4;

/// Crea una matriz de modelo completa (escala -> rotación -> traslación)
pub fn create_model_matrix(
    position: Vec3,
    rotation: Vec3,  // Ángulos en radianes (x, y, z)
    scale: Vec3,
) -> Mat4 {
    let scale_mat = Mat4::scale(scale.x, scale.y, scale.z);
    let rot_x = Mat4::rotation_x(rotation.x);
    let rot_y = Mat4::rotation_y(rotation.y);
    let rot_z = Mat4::rotation_z(rotation.z);
    let translation = Mat4::translation(position.x, position.y, position.z);

    // Orden: T * Rz * Ry * Rx * S
    translation
        .multiply(&rot_z)
        .multiply(&rot_y)
        .multiply(&rot_x)
        .multiply(&scale_mat)
}

/// Proyecta un punto 3D a coordenadas de pantalla 2D
pub fn project_to_screen(
    point: Vec3,
    mvp: &Mat4,
    screen_width: usize,
    screen_height: usize,
) -> Option<(i32, i32, f32)> {
    let transformed = mvp.transform_point(&point);
    
    // Verificar si está dentro del frustum
    if transformed.z < 0.0 || transformed.z > 1.0 {
        return None;
    }
    
    let x = ((transformed.x + 1.0) * 0.5 * screen_width as f32) as i32;
    let y = ((1.0 - transformed.y) * 0.5 * screen_height as f32) as i32;
    
    Some((x, y, transformed.z))
}

/// Interpola linealmente entre dos valores
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Interpola linealmente entre dos vectores
pub fn lerp_vec3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    Vec3::new(
        lerp(a.x, b.x, t),
        lerp(a.y, b.y, t),
        lerp(a.z, b.z, t),
    )
}

/// Clamp de un valor entre min y max
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    value.max(min).min(max)
}

/// Convierte grados a radianes
pub fn deg_to_rad(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

/// Convierte radianes a grados
pub fn rad_to_deg(radians: f32) -> f32 {
    radians * 180.0 / std::f32::consts::PI
}