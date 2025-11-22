use crate::math::Vec3;
use super::framebuffer::{Framebuffer, rgb_f32_to_u32};
use super::vertex::Vertex;

/// Rasteriza un triángulo usando interpolación baricéntrica
pub fn rasterize_triangle(
    fb: &mut Framebuffer,
    v0: &Vertex,
    v1: &Vertex,
    v2: &Vertex,
    light_dir: &Vec3,
    is_emissive: bool, // Para el sol, sin iluminación
) {
    // Convertir a coordenadas de pantalla
    let screen_v0 = to_screen_coords(v0.position, fb.width, fb.height);
    let screen_v1 = to_screen_coords(v1.position, fb.width, fb.height);
    let screen_v2 = to_screen_coords(v2.position, fb.width, fb.height);

    // Bounding box
    let min_x = screen_v0.0.min(screen_v1.0).min(screen_v2.0).max(0) as usize;
    let max_x = screen_v0.0.max(screen_v1.0).max(screen_v2.0).min(fb.width as i32 - 1) as usize;
    let min_y = screen_v0.1.min(screen_v1.1).min(screen_v2.1).max(0) as usize;
    let max_y = screen_v0.1.max(screen_v1.1).max(screen_v2.1).min(fb.height as i32 - 1) as usize;

    // Área del triángulo (para coordenadas baricéntricas)
    let area = edge_function(screen_v0, screen_v1, screen_v2);
    if area.abs() < 0.001 {
        return; // Triángulo degenerado
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = (x as i32, y as i32);
            
            // Coordenadas baricéntricas
            let w0 = edge_function(screen_v1, screen_v2, p);
            let w1 = edge_function(screen_v2, screen_v0, p);
            let w2 = edge_function(screen_v0, screen_v1, p);

            // Verificar si el punto está dentro del triángulo
            if (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0) || (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0) {
                let w0 = w0 / area;
                let w1 = w1 / area;
                let w2 = w2 / area;

                // Interpolar Z para depth test
                let z = v0.position.z * w0 + v1.position.z * w1 + v2.position.z * w2;

                // Interpolar normal
                let normal = Vec3::new(
                    v0.normal.x * w0 + v1.normal.x * w1 + v2.normal.x * w2,
                    v0.normal.y * w0 + v1.normal.y * w1 + v2.normal.y * w2,
                    v0.normal.z * w0 + v1.normal.z * w1 + v2.normal.z * w2,
                ).normalize();

                // Interpolar color base
                let base_color = (
                    v0.color.0 * w0 + v1.color.0 * w1 + v2.color.0 * w2,
                    v0.color.1 * w0 + v1.color.1 * w1 + v2.color.1 * w2,
                    v0.color.2 * w0 + v1.color.2 * w1 + v2.color.2 * w2,
                );

                // Calcular iluminación
                let final_color = if is_emissive {
                    // El sol brilla sin iluminación externa
                    base_color
                } else {
                    // Iluminación difusa simple
                    let diffuse = normal.dot(light_dir).max(0.1); // 0.1 = luz ambiente
                    (
                        base_color.0 * diffuse,
                        base_color.1 * diffuse,
                        base_color.2 * diffuse,
                    )
                };

                let color = rgb_f32_to_u32(final_color.0, final_color.1, final_color.2);
                fb.set_pixel(x, y, z, color);
            }
        }
    }
}

/// Convierte coordenadas normalizadas (-1 a 1) a coordenadas de pantalla
fn to_screen_coords(pos: Vec3, width: usize, height: usize) -> (i32, i32) {
    let x = ((pos.x + 1.0) * 0.5 * width as f32) as i32;
    let y = ((1.0 - pos.y) * 0.5 * height as f32) as i32; // Y invertido
    (x, y)
}

/// Función de borde para coordenadas baricéntricas
fn edge_function(v0: (i32, i32), v1: (i32, i32), p: (i32, i32)) -> f32 {
    ((p.0 - v0.0) * (v1.1 - v0.1) - (p.1 - v0.1) * (v1.0 - v0.0)) as f32
}

/// Dibuja una línea (para las órbitas)
pub fn draw_line(fb: &mut Framebuffer, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut x = x0;
    let mut y = y0;

    loop {
        if x >= 0 && x < fb.width as i32 && y >= 0 && y < fb.height as i32 {
            fb.set_pixel_no_depth(x as usize, y as usize, color);
        }
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}