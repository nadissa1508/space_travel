// line.rs
#![allow(dead_code)]

use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::framebuffer::Framebuffer;
use raylib::math::Vector3;
use raylib::prelude::Matrix;
use std::f32::consts::PI;


pub fn line(a: &Vertex, b: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let start = a.transformed_position;
    let end = b.transformed_position;

    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let x1 = end.x as i32;
    let y1 = end.y as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

    loop {
        let z = start.z + (end.z - start.z) * (x0 - start.x as i32) as f32 / (end.x - start.x) as f32;
        // For now, we'll just use white for the line color.
        // A more advanced implementation would interpolate the vertex colors.
        fragments.push(Fragment::new(x0 as f32, y0 as f32, Vector3::new(1.0, 1.0, 1.0), z));

        if x0 == x1 && y0 == y1 { break; }

        let e2 = err;
        if e2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if e2 < dy {
            err += dx;
            y0 += sy;
        }
    }

    fragments
}

// Agregar función para dibujar círculo (órbita)
pub fn draw_orbit_circle(
    framebuffer: &mut Framebuffer,
    center: Vector3,
    radius: f32,
    color: Vector3,
    segments: usize,
    viewport_matrix: &Matrix,
    view_matrix: &Matrix,
    projection_matrix: &Matrix,
) {
    for i in 0..segments {
        let angle1 = (i as f32 / segments as f32) * 2.0 * PI;
        let angle2 = ((i + 1) as f32 / segments as f32) * 2.0 * PI;
        
        // Puntos en el plano XZ (plano eclíptico)
        let p1 = Vector3::new(
            center.x + radius * angle1.cos(),
            center.y,  // Mismo Y (plano)
            center.z + radius * angle1.sin(),
        );
        
        let p2 = Vector3::new(
            center.x + radius * angle2.cos(),
            center.y,
            center.z + radius * angle2.sin(),
        );
        
        // Transformar a screen space
        let (screen_x1, screen_y1) = project_vertex(p1, view_matrix, projection_matrix, viewport_matrix);
        let (screen_x2, screen_y2) = project_vertex(p2, view_matrix, projection_matrix, viewport_matrix);
        
        // Crear vértices para la función line
        use raylib::math::Vector2 as RVec2;
        let vertex1 = Vertex::new(Vector3::new(screen_x1, screen_y1, 0.0), Vector3::zero(), RVec2::zero());
        let vertex2 = Vertex::new(Vector3::new(screen_x2, screen_y2, 0.0), Vector3::zero(), RVec2::zero());
        
        // Dibujar línea y agregar fragmentos al framebuffer
        let fragments = line(&vertex1, &vertex2);
        for fragment in fragments {
            framebuffer.point(fragment.position.x as i32, fragment.position.y as i32, color, fragment.depth);
        }
    }
}

fn project_vertex(
    pos: Vector3,
    view: &Matrix,
    proj: &Matrix,
    viewport: &Matrix
) -> (f32, f32) {
    // Manual matrix-vector multiplication for raylib Matrix
    // Transform by view matrix
    let vx = view.m0 * pos.x + view.m4 * pos.y + view.m8 * pos.z + view.m12;
    let vy = view.m1 * pos.x + view.m5 * pos.y + view.m9 * pos.z + view.m13;
    let vz = view.m2 * pos.x + view.m6 * pos.y + view.m10 * pos.z + view.m14;
    let vw = view.m3 * pos.x + view.m7 * pos.y + view.m11 * pos.z + view.m15;
    
    // Transform by projection matrix
    let cx = proj.m0 * vx + proj.m4 * vy + proj.m8 * vz + proj.m12 * vw;
    let cy = proj.m1 * vx + proj.m5 * vy + proj.m9 * vz + proj.m13 * vw;
    let cz = proj.m2 * vx + proj.m6 * vy + proj.m10 * vz + proj.m14 * vw;
    let cw = proj.m3 * vx + proj.m7 * vy + proj.m11 * vz + proj.m15 * vw;
    
    // Perspective divide
    let ndcx = cx / cw;
    let ndcy = cy / cw;
    let ndcz = cz / cw;
    
    // Transform by viewport matrix
    let sx = viewport.m0 * ndcx + viewport.m4 * ndcy + viewport.m8 * ndcz + viewport.m12;
    let sy = viewport.m1 * ndcx + viewport.m5 * ndcy + viewport.m9 * ndcz + viewport.m13;
    
    (sx, sy)
}