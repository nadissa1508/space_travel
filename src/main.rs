mod math;
mod renderer;
mod camera;
mod scene;
mod threading;
mod shaders;

use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

use math::{Vec3, Mat4};
use renderer::{Framebuffer, draw_line, rgb_to_u32, render_skybox};
use camera::Camera;
use scene::SolarSystem;
use shaders::{ShaderType, FragmentData, apply_shader};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    // Crear ventana
    let mut window = Window::new(
        "Space Travel - Sistema Solar",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            ..WindowOptions::default()
        },
    )
    .expect("No se pudo crear la ventana");

    // Limitar a ~60 FPS
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Inicializar componentes
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut camera = Camera::new(WIDTH as f32 / HEIGHT as f32);
    let mut solar_system = SolarSystem::new();

    // Estado
    let mut last_time = Instant::now();
    let mut current_target: usize = 0; // Índice del planeta que sigue la cámara
    let mut total_time: f32 = 0.0; // Tiempo total para animaciones de shaders

    // Posición inicial de la cámara
    camera.look_at_target(Vec3::zero());
    camera.set_distance(35.0); // Increased to see Glacius (orbit 25.0) better

    println!("Controles:");
    println!("  W/S - Acercar/Alejar cámara");
    println!("  A/D - Rotar cámara alrededor del objetivo");
    println!("  1-6 - Cambiar a planeta (1=Sol, 2-6=Planetas)");
    println!("  ESC - Salir");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Delta time
        let current_time = Instant::now();
        let mut delta_time = current_time.duration_since(last_time).as_secs_f32();
        last_time = current_time;

        // Clamp delta_time to prevent issues when window is paused/minimized
        // Max 0.1 seconds (10 FPS minimum) to prevent huge jumps
        delta_time = delta_time.min(0.1);
        total_time += delta_time;

        // === INPUT ===
        handle_input(&window, &mut camera, &mut current_target, &solar_system);

        // === UPDATE ===
        solar_system.update(delta_time);

        // Actualizar objetivo de la cámara
        let target_pos = solar_system.get_body_position(current_target);
        camera.look_at_target(target_pos);

        // === RENDER ===
        // Renderizar skybox primero (sin depth buffer)
        render_skybox(&mut framebuffer, total_time);
        
        // Limpiar solo el depth buffer (mantener el skybox)
        for depth in framebuffer.zbuffer.iter_mut() {
            *depth = f32::INFINITY;
        }

        // Matriz VP (View-Projection)
        let vp_matrix = camera.view_projection_matrix();

        // Renderizar órbitas (primero, para que estén detrás)
        render_orbits(&mut framebuffer, &solar_system, &vp_matrix);

        // Renderizar cada cuerpo celeste
        for body in &solar_system.bodies {
            let model_matrix = body.get_model_matrix();
            let mvp = vp_matrix.multiply(&model_matrix);

            // Dirección de luz hacia este cuerpo (desde el sol)
            let body_pos = body.get_position();
            let light_dir = if body.is_emissive {
                Vec3::zero() // El sol no necesita luz externa
            } else {
                (-body_pos).normalize() // Luz viene del centro (sol)
            };

            // Renderizar cada triángulo
            for triangle in &body.mesh {
                // Transformar vértices a espacio de clip
                let transformed = [
                    transform_vertex_with_local(&triangle[0], &mvp, &model_matrix),
                    transform_vertex_with_local(&triangle[1], &mvp, &model_matrix),
                    transform_vertex_with_local(&triangle[2], &mvp, &model_matrix),
                ];

                // Back-face culling simple
                let normal = calculate_face_normal(
                    &transformed[0].0,
                    &transformed[1].0,
                    &transformed[2].0,
                );
                if normal.z > 0.0 {
                    continue; // Cara trasera, no renderizar
                }

                // Frustum culling básico (si todos los vértices están fuera, saltar)
                if !is_visible(&transformed[0].0)
                    && !is_visible(&transformed[1].0)
                    && !is_visible(&transformed[2].0)
                {
                    continue;
                }

                // Rasterizar con shader
                rasterize_with_shader(
                    &mut framebuffer,
                    &transformed,
                    &[triangle[0].position, triangle[1].position, triangle[2].position],
                    body.shader_type,
                    total_time,
                    &light_dir,
                );
            }
        }

        // Mostrar en ventana
        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .expect("Error al actualizar ventana");
    }
}

/// Maneja el input del usuario
fn handle_input(
    window: &Window,
    camera: &mut Camera,
    current_target: &mut usize,
    solar_system: &SolarSystem,
) {
    // Zoom
    if window.is_key_down(Key::W) {
        camera.set_distance(camera.distance_from_target - 0.5);
    }
    if window.is_key_down(Key::S) {
        camera.set_distance(camera.distance_from_target + 0.5);
    }

    // Rotación
    if window.is_key_down(Key::A) {
        camera.rotate(-0.03);
    }
    if window.is_key_down(Key::D) {
        camera.rotate(0.03);
    }

    // Cambiar objetivo
    let keys = [Key::Key1, Key::Key2, Key::Key3, Key::Key4, Key::Key5, Key::Key6];
    for (i, key) in keys.iter().enumerate() {
        if window.is_key_pressed(*key, minifb::KeyRepeat::No) {
            if i < solar_system.body_count() {
                *current_target = i;
                // Ajustar distancia según el tamaño del planeta
                let body = solar_system.get_body(i).unwrap();
                camera.set_distance(body.radius * 8.0 + 5.0);
                println!("Siguiendo a: {}", body.name);
            }
        }
    }
}

/// Transforma un vértice con las matrices MVP
fn transform_vertex(
    v: &renderer::Vertex,
    mvp: &Mat4,
    model: &Mat4,
) -> renderer::Vertex {
    let transformed_pos = mvp.transform_point(&v.position);
    let transformed_normal = model.transform_direction(&v.normal).normalize();
    
    renderer::Vertex::new(transformed_pos, transformed_normal, v.color)
}

/// Transforma un vértice y retorna (clip_pos, world_normal, local_pos)
fn transform_vertex_with_local(
    v: &renderer::Vertex,
    mvp: &Mat4,
    model: &Mat4,
) -> (Vec3, Vec3, Vec3) {
    let clip_pos = mvp.transform_point(&v.position);
    let world_normal = model.transform_direction(&v.normal).normalize();
    (clip_pos, world_normal, v.position)
}

/// Rasteriza un triángulo aplicando shaders por fragmento
fn rasterize_with_shader(
    fb: &mut Framebuffer,
    transformed: &[(Vec3, Vec3, Vec3); 3], // (clip_pos, world_normal, local_pos)
    local_positions: &[Vec3; 3],
    shader_type: ShaderType,
    time: f32,
    light_dir: &Vec3,
) {
    // Convertir a coordenadas de pantalla
    let screen = [
        to_screen(transformed[0].0, fb.width, fb.height),
        to_screen(transformed[1].0, fb.width, fb.height),
        to_screen(transformed[2].0, fb.width, fb.height),
    ];

    // Bounding box
    let min_x = screen[0].0.min(screen[1].0).min(screen[2].0).max(0) as usize;
    let max_x = screen[0].0.max(screen[1].0).max(screen[2].0).min(fb.width as i32 - 1) as usize;
    let min_y = screen[0].1.min(screen[1].1).min(screen[2].1).max(0) as usize;
    let max_y = screen[0].1.max(screen[1].1).max(screen[2].1).min(fb.height as i32 - 1) as usize;

    // Área del triángulo
    let area = edge_function(screen[0], screen[1], screen[2]);
    if area.abs() < 0.001 {
        return;
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = (x as i32, y as i32);

            // Coordenadas baricéntricas
            let w0 = edge_function(screen[1], screen[2], p);
            let w1 = edge_function(screen[2], screen[0], p);
            let w2 = edge_function(screen[0], screen[1], p);

            // Verificar si está dentro del triángulo
            if (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0) || (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0) {
                let w0 = w0 / area;
                let w1 = w1 / area;
                let w2 = w2 / area;

                // Interpolar Z
                let z = transformed[0].0.z * w0 + transformed[1].0.z * w1 + transformed[2].0.z * w2;

                // Interpolar normal
                let normal = Vec3::new(
                    transformed[0].1.x * w0 + transformed[1].1.x * w1 + transformed[2].1.x * w2,
                    transformed[0].1.y * w0 + transformed[1].1.y * w1 + transformed[2].1.y * w2,
                    transformed[0].1.z * w0 + transformed[1].1.z * w1 + transformed[2].1.z * w2,
                ).normalize();

                // Interpolar posición local (para el shader)
                let local_pos = Vec3::new(
                    local_positions[0].x * w0 + local_positions[1].x * w1 + local_positions[2].x * w2,
                    local_positions[0].y * w0 + local_positions[1].y * w1 + local_positions[2].y * w2,
                    local_positions[0].z * w0 + local_positions[1].z * w1 + local_positions[2].z * w2,
                );

                // Normalizar posición para shaders (esperan posiciones en esfera unitaria)
                let normalized_pos = local_pos.normalize();

                // Crear datos del fragmento
                let fragment = FragmentData {
                    position: normalized_pos,
                    normal,
                    world_pos: local_pos, // Simplificado
                };

                // Aplicar shader
                let color = apply_shader(shader_type, &fragment, time, light_dir);
                let color_u32 = rgb_to_u32(
                    (color.0.clamp(0.0, 1.0) * 255.0) as u8,
                    (color.1.clamp(0.0, 1.0) * 255.0) as u8,
                    (color.2.clamp(0.0, 1.0) * 255.0) as u8,
                );

                fb.set_pixel(x, y, z, color_u32);
            }
        }
    }
}

fn to_screen(pos: Vec3, width: usize, height: usize) -> (i32, i32) {
    let x = ((pos.x + 1.0) * 0.5 * width as f32) as i32;
    let y = ((1.0 - pos.y) * 0.5 * height as f32) as i32;
    (x, y)
}

fn edge_function(v0: (i32, i32), v1: (i32, i32), p: (i32, i32)) -> f32 {
    ((p.0 - v0.0) * (v1.1 - v0.1) - (p.1 - v0.1) * (v1.0 - v0.0)) as f32
}

/// Calcula la normal de una cara (para back-face culling)
fn calculate_face_normal(v0: &Vec3, v1: &Vec3, v2: &Vec3) -> Vec3 {
    let edge1 = *v1 - *v0;
    let edge2 = *v2 - *v0;
    edge1.cross(&edge2).normalize()
}

/// Verifica si un punto está en el frustum visible
fn is_visible(p: &Vec3) -> bool {
    p.x >= -1.5 && p.x <= 1.5 && p.y >= -1.5 && p.y <= 1.5 && p.z >= 0.0 && p.z <= 1.0
}

/// Renderiza las órbitas de los planetas
fn render_orbits(fb: &mut Framebuffer, solar_system: &SolarSystem, vp_matrix: &Mat4) {
    for (i, orbit) in solar_system.orbit_points.iter().enumerate() {
        if orbit.is_empty() {
            continue;
        }

        // Color de órbita basado en el planeta
        let body = &solar_system.bodies[i];
        let orbit_color = rgb_to_u32(
            (body.color.0 * 100.0) as u8,
            (body.color.1 * 100.0) as u8,
            (body.color.2 * 100.0) as u8,
        );

        for j in 0..orbit.len() {
            let p1 = &orbit[j];
            let p2 = &orbit[(j + 1) % orbit.len()];

            // Transformar puntos
            let sp1 = vp_matrix.transform_point(p1);
            let sp2 = vp_matrix.transform_point(p2);

            // Convertir a coordenadas de pantalla
            if sp1.z > 0.0 && sp2.z > 0.0 && sp1.z < 1.0 && sp2.z < 1.0 {
                let x1 = ((sp1.x + 1.0) * 0.5 * fb.width as f32) as i32;
                let y1 = ((1.0 - sp1.y) * 0.5 * fb.height as f32) as i32;
                let x2 = ((sp2.x + 1.0) * 0.5 * fb.width as f32) as i32;
                let y2 = ((1.0 - sp2.y) * 0.5 * fb.height as f32) as i32;

                draw_line(fb, x1, y1, x2, y2, orbit_color);
            }
        }
    }
}