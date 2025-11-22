/// Framebuffer con soporte para z-buffer
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,      // Color buffer (ARGB)
    pub zbuffer: Vec<f32>,     // Depth buffer
    pub background_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            buffer: vec![0x000000; size],
            zbuffer: vec![f32::INFINITY; size],
            background_color: 0x000510, // Azul muy oscuro para el espacio
        }
    }

    /// Limpia el buffer con el color de fondo
    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
        for depth in self.zbuffer.iter_mut() {
            *depth = f32::INFINITY;
        }
    }

    /// Establece un píxel con z-test
    pub fn set_pixel(&mut self, x: usize, y: usize, z: f32, color: u32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            if z < self.zbuffer[index] {
                self.zbuffer[index] = z;
                self.buffer[index] = color;
            }
        }
    }

    /// Establece un píxel sin z-test (para elementos 2D como órbitas)
    pub fn set_pixel_no_depth(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = color;
        }
    }

    /// Obtiene el índice en el buffer
    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

/// Convierte RGB a u32 (formato 0xAARRGGBB)
pub fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

/// Convierte f32 RGB (0.0-1.0) a u32
pub fn rgb_f32_to_u32(r: f32, g: f32, b: f32) -> u32 {
    let r = (r.clamp(0.0, 1.0) * 255.0) as u8;
    let g = (g.clamp(0.0, 1.0) * 255.0) as u8;
    let b = (b.clamp(0.0, 1.0) * 255.0) as u8;
    rgb_to_u32(r, g, b)
}