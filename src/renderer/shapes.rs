use crate::math::Vec3;
use super::vertex::Vertex;

/// Genera una esfera UV (para planetas)
/// Retorna lista de triángulos (cada triángulo = 3 vértices)
pub fn generate_sphere(
    radius: f32,
    segments: usize,
    rings: usize,
    color: (f32, f32, f32),
) -> Vec<[Vertex; 3]> {
    let mut triangles = Vec::new();

    for ring in 0..rings {
        let theta1 = std::f32::consts::PI * (ring as f32) / (rings as f32);
        let theta2 = std::f32::consts::PI * ((ring + 1) as f32) / (rings as f32);

        for seg in 0..segments {
            let phi1 = 2.0 * std::f32::consts::PI * (seg as f32) / (segments as f32);
            let phi2 = 2.0 * std::f32::consts::PI * ((seg + 1) as f32) / (segments as f32);

            // Cuatro vértices del quad
            let p1 = sphere_point(theta1, phi1, radius);
            let p2 = sphere_point(theta1, phi2, radius);
            let p3 = sphere_point(theta2, phi1, radius);
            let p4 = sphere_point(theta2, phi2, radius);

            // Normales (para una esfera centrada en origen, normal = posición normalizada)
            let n1 = p1.normalize();
            let n2 = p2.normalize();
            let n3 = p3.normalize();
            let n4 = p4.normalize();

            // Añadir variación de color basada en la posición (opcional, para más estética)
            let color_var = |n: &Vec3| {
                let factor = 0.8 + 0.2 * ((n.y + 1.0) * 0.5); // Variación sutil
                (
                    (color.0 * factor).min(1.0),
                    (color.1 * factor).min(1.0),
                    (color.2 * factor).min(1.0),
                )
            };

            let v1 = Vertex::new(p1, n1, color_var(&n1));
            let v2 = Vertex::new(p2, n2, color_var(&n2));
            let v3 = Vertex::new(p3, n3, color_var(&n3));
            let v4 = Vertex::new(p4, n4, color_var(&n4));

            // Dos triángulos por quad
            if ring != 0 {
                triangles.push([v1, v2, v3]);
            }
            if ring != rings - 1 {
                triangles.push([v2, v4, v3]);
            }
        }
    }

    triangles
}

fn sphere_point(theta: f32, phi: f32, radius: f32) -> Vec3 {
    Vec3::new(
        radius * theta.sin() * phi.cos(),
        radius * theta.cos(),
        radius * theta.sin() * phi.sin(),
    )
}

/// Genera puntos para dibujar una órbita circular
pub fn generate_orbit_points(radius: f32, segments: usize) -> Vec<Vec3> {
    let mut points = Vec::with_capacity(segments);
    for i in 0..segments {
        let angle = 2.0 * std::f32::consts::PI * (i as f32) / (segments as f32);
        points.push(Vec3::new(
            radius * angle.cos(),
            0.0, // En el plano eclíptico
            radius * angle.sin(),
        ));
    }
    points
}