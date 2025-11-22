pub mod framebuffer;
pub mod vertex;
pub mod triangle;
pub mod shapes;
pub mod shader;
pub mod skybox;

pub use framebuffer::{Framebuffer, rgb_to_u32, rgb_f32_to_u32};
pub use vertex::Vertex;
pub use triangle::{rasterize_triangle, draw_line};
pub use shapes::{generate_sphere, generate_orbit_points};
pub use shader::*;
pub use skybox::render_skybox;
