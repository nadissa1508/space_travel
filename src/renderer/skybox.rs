use crate::math::Vec3;
use super::framebuffer::{Framebuffer, rgb_to_u32};

/// Renderiza un skybox con estrellas en el fondo
pub fn render_skybox(fb: &mut Framebuffer, time: f32) {
    let width = fb.width;
    let height = fb.height;
    
    for y in 0..height {
        for x in 0..width {
            // Convertir coordenadas de pantalla a dirección de raycast
            let u = (x as f32 / width as f32) * 2.0 - 1.0;
            let v = (y as f32 / height as f32) * 2.0 - 1.0;
            
            // Dirección del ray (apuntando hacia el cielo)
            let dir = Vec3::new(u * 2.0, v * 1.5, 1.0).normalize();
            
            // Color base del cielo (azul oscuro espacial)
            let sky_color = get_sky_color(&dir);
            
            // Añadir estrellas
            let star_color = get_star(&dir, time);
            
            // Combinar cielo y estrellas
            let final_r = (sky_color.0 + star_color.0).min(1.0);
            let final_g = (sky_color.1 + star_color.1).min(1.0);
            let final_b = (sky_color.2 + star_color.2).min(1.0);
            
            let color = rgb_to_u32(
                (final_r * 255.0) as u8,
                (final_g * 255.0) as u8,
                (final_b * 255.0) as u8,
            );
            
            fb.buffer[y * width + x] = color;
        }
    }
}

/// Calcula el color del cielo basado en la dirección
fn get_sky_color(dir: &Vec3) -> (f32, f32, f32) {
    // Gradiente vertical: más oscuro arriba, ligeramente más claro abajo
    let t = (dir.y + 1.0) * 0.5; // Normalizar de [-1,1] a [0,1]
    
    // Azul muy oscuro (casi negro espacial)
    let dark_blue = (0.01, 0.02, 0.05);
    // Azul ligeramente más claro cerca del horizonte
    let horizon_blue = (0.02, 0.04, 0.12);
    
    // Interpolar entre los dos colores
    (
        dark_blue.0 + (horizon_blue.0 - dark_blue.0) * t,
        dark_blue.1 + (horizon_blue.1 - dark_blue.1) * t,
        dark_blue.2 + (horizon_blue.2 - dark_blue.2) * t,
    )
}

/// Genera estrellas proceduralmente
fn get_star(dir: &Vec3, time: f32) -> (f32, f32, f32) {
    // Usar la dirección como coordenadas para generar estrellas
    let scale = 150.0; // Cuántas estrellas (más alto = más densidad)
    
    let x = dir.x * scale;
    let y = dir.y * scale;
    let z = dir.z * scale;
    
    // Hash function para generar posiciones de estrellas pseudo-aleatorias
    let star_value = star_hash(x, y, z);
    
    // Threshold para determinar si hay una estrella
    let star_threshold = 0.985; // Ajusta para más/menos estrellas (más alto = menos estrellas)
    
    if star_value > star_threshold {
        // Intensidad de la estrella basada en qué tan por encima del threshold está
        let intensity = ((star_value - star_threshold) / (1.0 - star_threshold)).powf(0.5);
        
        // Parpadeo sutil
        let twinkle = ((time * 3.0 + star_value * 100.0).sin() * 0.5 + 0.5) * 0.3 + 0.7;
        let final_intensity = intensity * twinkle;
        
        // Color de estrella (blanco con ligero tinte)
        let star_base = if star_value > 0.995 {
            // Estrellas brillantes - blanco puro
            (1.0, 1.0, 1.0)
        } else if star_value > 0.992 {
            // Estrellas azuladas
            (0.8, 0.9, 1.0)
        } else {
            // Estrellas normales - blanco cálido
            (1.0, 0.95, 0.9)
        };
        
        (
            star_base.0 * final_intensity,
            star_base.1 * final_intensity,
            star_base.2 * final_intensity,
        )
    } else {
        (0.0, 0.0, 0.0)
    }
}

/// Función hash para generar estrellas de manera procedural
fn star_hash(x: f32, y: f32, z: f32) -> f32 {
    // Tomar la parte entera de las coordenadas
    let ix = x.floor();
    let iy = y.floor();
    let iz = z.floor();
    
    // Hash simple pero efectivo
    let mut h = ix * 127.1 + iy * 311.7 + iz * 758.5453;
    h = (h.sin() * 43758.5453).fract();
    
    h
}

/// Función fract (parte fraccionaria)
fn fract(x: f32) -> f32 {
    x - x.floor()
}
