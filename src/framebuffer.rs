// framebuffer.rs
use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pixel_buffer: Vec<u8>, // Buffer de píxeles en RGBA (4 bytes por píxel)
    background_color: Vector3,
    texture: Option<Texture2D>,
    depth_buffer: Vec<f32>,
}

impl Framebuffer {
    const DEPTH_EPSILON: f32 = 1e-6;

    pub fn new(width: u32, height: u32) -> Self {
        let buffer_size = (width * height * 4) as usize; // 4 bytes per pixel (RGBA)
        let pixel_buffer = vec![0u8; buffer_size];

        let depth_buffer_size = (width * height) as usize;
        let depth_buffer = vec![f32::INFINITY; depth_buffer_size];

        Framebuffer {
            width,
            height,
            pixel_buffer,
            background_color: Vector3::zero(),
            texture: None,
            depth_buffer,
        }
    }

    pub fn init_texture(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        // Crear una imagen vacía con el buffer de píxeles
        let image = Image::gen_image_color(self.width as i32, self.height as i32, Color::BLACK);

        // Load texture from image directly
        self.texture = Some(rl.load_texture_from_image(thread, &image).unwrap());
    }

    pub fn clear(&mut self) {
        let bg_color_r = (self.background_color.x.clamp(0.0, 1.0) * 255.0) as u8;
        let bg_color_g = (self.background_color.y.clamp(0.0, 1.0) * 255.0) as u8;
        let bg_color_b = (self.background_color.z.clamp(0.0, 1.0) * 255.0) as u8;
        let bg_color_a = 255u8;

        // Llenar el buffer de píxeles con el color de fondo
        // Raylib usa formato RGBA
        for i in (0..self.pixel_buffer.len()).step_by(4) {
            self.pixel_buffer[i] = bg_color_r;
            self.pixel_buffer[i + 1] = bg_color_g;
            self.pixel_buffer[i + 2] = bg_color_b;
            self.pixel_buffer[i + 3] = bg_color_a;
        }

        // Clear depth buffer to far plane
        self.depth_buffer.fill(f32::INFINITY);
    }

    pub fn point(&mut self, x: i32, y: i32, color: Vector3, depth: f32) -> bool {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            let index = (y * self.width as i32 + x) as usize;

            // Depth test with small bias to avoid acne/z-fighting:
            if depth < (self.depth_buffer[index] - Self::DEPTH_EPSILON) {
                self.depth_buffer[index] = depth;

                // Convert coords to RGBA index
                let pixel_index = index * 4;

                self.pixel_buffer[pixel_index] = (color.x.clamp(0.0, 1.0) * 255.0) as u8;
                self.pixel_buffer[pixel_index + 1] = (color.y.clamp(0.0, 1.0) * 255.0) as u8;
                self.pixel_buffer[pixel_index + 2] = (color.z.clamp(0.0, 1.0) * 255.0) as u8;
                self.pixel_buffer[pixel_index + 3] = 255;

                return true;
            }
        }
        false
    }

    pub fn set_background_color(&mut self, color: Vector3) {
        self.background_color = color;
    }

    pub fn swap_buffers(&mut self) {
        if let Some(texture) = &mut self.texture {
            // Actualizar textura directamente desde nuestro buffer
            // Sin conversiones ni copias innecesarias
            texture.update_texture(&self.pixel_buffer).unwrap();
        } else {
            panic!(
                "Framebuffer texture has not been initialized. Call init_texture after creating the RaylibHandle."
            );
        }
    }

    pub fn get_texture(&self) -> &Texture2D {
        self.texture.as_ref().expect("Texture not initialized")
    }
}
