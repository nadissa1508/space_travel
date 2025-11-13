// main.rs - Sistema Multi-Planeta con Shaders Procedurales

mod camera;
mod fragment;
mod framebuffer;
mod light;
mod line;
mod matrix;
mod obj;
mod shaders;
mod triangle;
mod vertex;

use crate::camera::Camera;
use crate::light::Light;
use crate::matrix::{create_model_matrix, create_projection_matrix, create_viewport_matrix};
use framebuffer::Framebuffer;
use obj::Obj;
use raylib::prelude::*;
use shaders::{fragment_shader, vertex_shader, ShaderType};
use std::f32::consts::PI;
use triangle::triangle;
use vertex::Vertex;

pub struct Uniforms {
    pub model_matrix: Matrix,
    pub view_matrix: Matrix,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
    pub time: f32,
    pub shader_type: ShaderType,
}

fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    light: &Light,
) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], light));
    }

    for fragment in fragments {
        let final_color = fragment_shader(&fragment, uniforms);
        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            final_color,
            fragment.depth,
        );
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Laboratorio 4 - Shaders Planetarios")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    window.set_target_fps(60);
    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Vector3::new(0.0, 0.0, 0.05)); // Espacio oscuro

    framebuffer.init_texture(&mut window, &thread);

    // Camera setup
    let camera_position = Vector3::new(0.0, 3.0, 8.0);
    let camera_target = Vector3::new(0.0, 0.0, 0.0);
    let camera_up = Vector3::new(0.0, 1.0, 0.0);
    let mut camera = Camera::new(camera_position, camera_target, camera_up);

    // Projection setup
    let fov_y = PI / 3.0;
    let aspect = window_width as f32 / window_height as f32;
    let near = 0.1;
    let far = 100.0;

    // Light setup
    let light = Light::new(Vector3::new(10.0, 10.0, 10.0));

    // Cargar esfera base
    let obj = Obj::load("assets/models/sphere.obj").expect("Failed to load sphere.obj");
    let vertex_array = obj.get_vertex_array();

    let mut elapsed_time = 0.0f32;
    let mut current_shader = ShaderType::Rocky;
    let mut auto_rotate = true;
    let mut rotation_y = 0.0f32;

    // Sistema multi-objeto (planeta + anillo + luna)
    let mut show_rings = false;
    let mut show_moon = false;

    println!("=== CONTROLES ===");
    println!("1-6: Cambiar shader de planeta/estrella");
    println!("R: Toggle anillos");
    println!("M: Toggle luna");
    println!("SPACE: Pausar rotación");
    println!("WASD: Rotar cámara");
    println!("Flechas: Zoom y pan");

    while !window.window_should_close() {
        let delta_time = window.get_frame_time();
        elapsed_time += delta_time;

        // Input para cambiar shaders
        if window.is_key_pressed(KeyboardKey::KEY_ONE) {
            current_shader = ShaderType::Rocky;
            println!("Shader: Planeta Rocoso");
        }
        if window.is_key_pressed(KeyboardKey::KEY_TWO) {
            current_shader = ShaderType::GasGiant;
            println!("Shader: Gigante Gaseoso");
        }
        if window.is_key_pressed(KeyboardKey::KEY_THREE) {
            current_shader = ShaderType::Lava;
            println!("Shader: Planeta de Lava (Extra)");
        }
        if window.is_key_pressed(KeyboardKey::KEY_FOUR) {
            current_shader = ShaderType::Ice;
            println!("Shader: Planeta de Hielo (Extra)");
        }
        if window.is_key_pressed(KeyboardKey::KEY_FIVE) {
            current_shader = ShaderType::Alien;
            println!("Shader: Planeta Alien (Extra)");
        }
        if window.is_key_pressed(KeyboardKey::KEY_SIX) {
            current_shader = ShaderType::SolarHeart;
            println!("Shader: Solar Heart (Lab 5)");
        }

        if window.is_key_pressed(KeyboardKey::KEY_R) {
            show_rings = !show_rings;
            println!("Anillos: {}", if show_rings { "ON" } else { "OFF" });
        }

        if window.is_key_pressed(KeyboardKey::KEY_M) {
            show_moon = !show_moon;
            println!("Luna: {}", if show_moon { "ON" } else { "OFF" });
        }

        if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
            auto_rotate = !auto_rotate;
            println!("Auto-rotación: {}", if auto_rotate { "ON" } else { "OFF" });
        }

        camera.process_input(&window);

        if auto_rotate {
            rotation_y += 0.01;
        }

        framebuffer.clear();

        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(fov_y, aspect, near, far);
        let viewport_matrix = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

        // === RENDERIZAR PLANETA PRINCIPAL ===
        let translation = Vector3::new(0.0, 0.0, 0.0);
        let rotation = Vector3::new(0.0, rotation_y, 0.0);
        let model_matrix = create_model_matrix(translation, 2.5, rotation);

        let uniforms = Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time: elapsed_time,
            shader_type: current_shader,
        };

        render(&mut framebuffer, &uniforms, &vertex_array, &light);

        // === RENDERIZAR ANILLOS (si están activos) ===
        if show_rings {
            let ring_rotation = Vector3::new(PI / 4.0, rotation_y * 0.5, 0.0);
            let ring_scale = 4.0;
            let ring_model = create_model_matrix(translation, ring_scale, ring_rotation);

            let ring_uniforms = Uniforms {
                model_matrix: ring_model,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time: elapsed_time,
                shader_type: ShaderType::Rings,
            };

            render(&mut framebuffer, &ring_uniforms, &vertex_array, &light);
        }

        // === RENDERIZAR LUNA (si está activa) ===
        if show_moon {
            let moon_orbit_radius = 5.5;
            let moon_translation = Vector3::new(
                moon_orbit_radius * (elapsed_time * 0.5).cos(),
                0.5 * (elapsed_time * 0.3).sin(),
                moon_orbit_radius * (elapsed_time * 0.5).sin(),
            );
            let moon_model = create_model_matrix(moon_translation, 0.6, Vector3::new(0.0, elapsed_time, 0.0));

            let moon_uniforms = Uniforms {
                model_matrix: moon_model,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time: elapsed_time,
                shader_type: ShaderType::Moon,
            };

            render(&mut framebuffer, &moon_uniforms, &vertex_array, &light);
        }

        framebuffer.swap_buffers();

        // Single drawing context for both framebuffer texture and UI overlay
        let mut d = window.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        // Draw the framebuffer texture
        d.draw_texture(framebuffer.get_texture(), 0, 0, Color::WHITE);

        // UI Overlay
        let shader_name = match current_shader {
            ShaderType::Rocky => "Planeta Rocoso",
            ShaderType::GasGiant => "Gigante Gaseoso",
            ShaderType::Lava => "Planeta de Lava",
            ShaderType::Ice => "Planeta de Hielo",
            ShaderType::Alien => "Planeta Alien",
            ShaderType::SolarHeart => "Solar Heart",
            _ => "Unknown",
        };

        d.draw_text(&format!("Shader: {}", shader_name), 10, 10, 20, Color::WHITE);
        d.draw_text(&format!("Anillos: {} (R)", if show_rings { "ON" } else { "OFF" }), 10, 35, 16, Color::LIGHTGRAY);
        d.draw_text(&format!("Luna: {} (M)", if show_moon { "ON" } else { "OFF" }), 10, 55, 16, Color::LIGHTGRAY);
        d.draw_text("1-6: Cambiar planeta/estrella | SPACE: Pausar", 10, 580, 14, Color::DARKGRAY);
    }
}