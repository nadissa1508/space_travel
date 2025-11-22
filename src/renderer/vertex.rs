use crate::math::Vec3;

/// Vértice con posición, normal y color
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub color: (f32, f32, f32), // RGB normalizado 0.0-1.0
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, color: (f32, f32, f32)) -> Self {
        Self { position, normal, color }
    }

    /// Interpola entre dos vértices
    pub fn lerp(a: &Vertex, b: &Vertex, t: f32) -> Vertex {
        Vertex {
            position: Vec3::new(
                a.position.x + (b.position.x - a.position.x) * t,
                a.position.y + (b.position.y - a.position.y) * t,
                a.position.z + (b.position.z - a.position.z) * t,
            ),
            normal: Vec3::new(
                a.normal.x + (b.normal.x - a.normal.x) * t,
                a.normal.y + (b.normal.y - a.normal.y) * t,
                a.normal.z + (b.normal.z - a.normal.z) * t,
            ).normalize(),
            color: (
                a.color.0 + (b.color.0 - a.color.0) * t,
                a.color.1 + (b.color.1 - a.color.1) * t,
                a.color.2 + (b.color.2 - a.color.2) * t,
            ),
        }
    }
}