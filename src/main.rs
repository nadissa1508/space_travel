// main.rs - Sistema Multi-Planeta con Shaders Procedurales

mod camera;
mod celestial_body;
mod fragment;
mod framebuffer;
mod light;
mod line;
mod matrix;
mod obj;
mod orbit;
mod shaders;
mod skybox;
mod solar_system;
mod triangle;
mod vertex;

use crate::camera::Camera;
use crate::light::Light;
use crate::line::draw_orbit_circle;
use crate::matrix::{create_model_matrix, create_projection_matrix, create_viewport_matrix};
use crate::skybox::Skybox;
use crate::solar_system::SolarSystem;
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

    // Camera setup - positioned to see the whole solar system
    let camera_position = Vector3::new(0.0, 20.0, 20.0);  // High up and far to see all planets
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

    // Crear skybox
    let skybox = Skybox::new();
    let skybox_vertices = skybox.get_vertex_array();

    let mut elapsed_time = 0.0f32;
    let mut auto_rotate = true;
    let mut rotation_y = 0.0f32;

    // Sistema multi-objeto (planeta + anillo + luna)
    let mut show_rings = false;
    let mut show_moon = false;

    // Sistema solar
    let mut solar_system = SolarSystem::new();

    // Debug: print planet positions
    println!("=== SISTEMA SOLAR INICIALIZADO ===");
    for body in &solar_system.bodies {
        let pos = body.get_world_position();
        println!("{}: orbit_radius={}, orbit_angle={}, position=({:.2}, {:.2}, {:.2})", 
            body.name, body.orbit_radius, body.orbit_angle, pos.x, pos.y, pos.z);
    }

    println!("=== CONTROLES ===");
    println!("1: Warp a Mercurio");
    println!("2: Warp a Venus");
    println!("3: Warp a Tierra");
    println!("4: Warp a Marte");
    println!("5: Warp a Júpiter (con anillos)");
    println!("6: Warp a Xenos (planeta alien con 2 lunas)");
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
            if let Some(body) = solar_system.get_body_by_name("Mercurio") {
                let body_pos = body.get_world_position();
                let warp_pos = Vector3::new(body_pos.x, body_pos.y + 1.0, body_pos.z + 3.0);
                camera.warp_to(warp_pos, 1.5); // 1.5 segundos de animación
            }
        }
        if window.is_key_pressed(KeyboardKey::KEY_TWO) {
            if let Some(body) = solar_system.get_body_by_name("Venus") {
                let body_pos = body.get_world_position();
                let warp_pos = Vector3::new(body_pos.x, body_pos.y + 1.0, body_pos.z + 3.0);
                camera.warp_to(warp_pos, 1.5);
            }
        }
        if window.is_key_pressed(KeyboardKey::KEY_THREE) {
            if let Some(body) = solar_system.get_body_by_name("Tierra") {
                let body_pos = body.get_world_position();
                let warp_pos = Vector3::new(body_pos.x, body_pos.y + 1.5, body_pos.z + 3.5);
                camera.warp_to(warp_pos, 1.5);
                println!("Warping a Tierra (con Luna)");
            }
        }
        if window.is_key_pressed(KeyboardKey::KEY_FOUR) {
            if let Some(body) = solar_system.get_body_by_name("Marte") {
                let body_pos = body.get_world_position();
                let warp_pos = Vector3::new(body_pos.x, body_pos.y + 1.5, body_pos.z + 3.0);
                camera.warp_to(warp_pos, 1.5);
                println!("Warping a Marte (con luna Fobos)");
            }
        }
        if window.is_key_pressed(KeyboardKey::KEY_FIVE) {
            if let Some(body) = solar_system.get_body_by_name("Júpiter") {
                let body_pos = body.get_world_position();
                let warp_pos = Vector3::new(body_pos.x, body_pos.y + 2.5, body_pos.z + 5.0);
                camera.warp_to(warp_pos, 1.5);
                println!("Warping a Júpiter (con anillos)");
            }
        }
        if window.is_key_pressed(KeyboardKey::KEY_SIX) {
            if let Some(body) = solar_system.get_body_by_name("Xenos") {
                let body_pos = body.get_world_position();
                let warp_pos = Vector3::new(body_pos.x, body_pos.y + 2.0, body_pos.z + 4.0);
                camera.warp_to(warp_pos, 1.5);
                println!("Warping a Xenos (planeta alien con 2 lunas)");
            }
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
        // verificar si aqui es el update loop
        camera.update_warp(window.get_frame_time());

        // Actualizar el sistema solar
        solar_system.update(delta_time);

        if auto_rotate {
            rotation_y += 0.01;
        }

        framebuffer.clear();

        let view_matrix = camera.get_view_matrix();
        let projection_matrix = create_projection_matrix(fov_y, aspect, near, far);
        let viewport_matrix =
            create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

        // === RENDERIZAR SKYBOX ===
        // El skybox se renderiza primero como fondo
        let skybox_model = create_model_matrix(
            camera.target,  // Skybox centered on camera target
            1.0,
            Vector3::new(0.0, 0.0, 0.0)
        );

        let skybox_uniforms = Uniforms {
            model_matrix: skybox_model,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time: elapsed_time,
            shader_type: ShaderType::Skybox,
        };

        render(&mut framebuffer, &skybox_uniforms, skybox_vertices, &light);

        // En el render loop, antes de renderizar los planetas:
        if solar_system.orbit_lines_enabled {
            let orbit_color = Vector3::new(0.0, 0.8, 1.0); // Cyan brillante

            for body in &solar_system.bodies {
                if body.orbit_radius > 0.0 {
                    // No dibujar órbita del sol
                    let center = body.orbit_center;
                    let center_raylib = Vector3::new(center.x, center.y, center.z);
                    draw_orbit_circle(
                        &mut framebuffer,
                        center_raylib,
                        body.orbit_radius,
                        orbit_color,
                        64, // Segmentos (más = más suave)
                        &viewport_matrix,
                        &view_matrix,
                        &projection_matrix,
                    );

                    // También dibujar órbitas de lunas
                    for moon in &body.satellites {
                        let pos = body.get_world_position();
                        let pos_raylib = Vector3::new(pos.x, pos.y, pos.z);
                        draw_orbit_circle(
                            &mut framebuffer,
                            pos_raylib,
                            moon.orbit_radius,
                            orbit_color * 0.6, // Más tenue para lunas
                            32,
                            &viewport_matrix,
                            &view_matrix,
                            &projection_matrix,
                        );
                    }
                }
            }
        }

        // === RENDERIZAR SISTEMA SOLAR ===
        for body in &solar_system.bodies {
            let model_matrix = body.get_model_matrix();

            let uniforms = Uniforms {
                model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time: elapsed_time,
                shader_type: body.shader_type,
            };

            render(&mut framebuffer, &uniforms, &vertex_array, &light);

            // Renderizar anillos si el planeta los tiene
            if body.has_rings {
                let body_pos = body.get_world_position();
                let ring_position = Vector3::new(body_pos.x, body_pos.y, body_pos.z);
                let ring_rotation = Vector3::new(PI / 4.0, body.rotation_angle.to_radians() * 0.3, 0.0);
                let ring_scale = body.ring_outer_radius;
                let ring_model = create_model_matrix(ring_position, ring_scale, ring_rotation);

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

            // Renderizar satélites (lunas)
            for satellite in &body.satellites {
                let satellite_model = satellite.get_model_matrix();

                let satellite_uniforms = Uniforms {
                    model_matrix: satellite_model,
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    time: elapsed_time,
                    shader_type: satellite.shader_type,
                };

                render(&mut framebuffer, &satellite_uniforms, &vertex_array, &light);
            }
        }

        // === RENDERIZAR ANILLOS (si están activos) ===
        if show_rings {
            let ring_translation = Vector3::new(0.0, 0.0, 0.0);
            let ring_rotation = Vector3::new(PI / 4.0, rotation_y * 0.5, 0.0);
            let ring_scale = 4.0;
            let ring_model = create_model_matrix(ring_translation, ring_scale, ring_rotation);

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
            let moon_model =
                create_model_matrix(moon_translation, 0.6, Vector3::new(0.0, elapsed_time, 0.0));

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
        d.draw_text(
            "Sistema Solar - Presiona 1-6 para warp",
            10,
            10,
            20,
            Color::WHITE,
        );
        d.draw_text(
            &format!("Planetas: {} | Cámara: ({:.1}, {:.1}, {:.1})", 
                solar_system.bodies.len(), camera.eye.x, camera.eye.y, camera.eye.z),
            10,
            35,
            16,
            Color::LIGHTGRAY,
        );
        d.draw_text(
            "1-6: Cambiar planeta/estrella | SPACE: Pausar",
            10,
            580,
            14,
            Color::DARKGRAY,
        );
    }
}
