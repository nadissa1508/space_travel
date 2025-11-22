use super::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub data: [[f32; 4]; 4],
}

impl Mat4 {
    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn zero() -> Self {
        Self {
            data: [[0.0; 4]; 4],
        }
    }

    /// Matriz de traslación
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut m = Self::identity();
        m.data[0][3] = x;
        m.data[1][3] = y;
        m.data[2][3] = z;
        m
    }

    /// Matriz de escala
    pub fn scale(sx: f32, sy: f32, sz: f32) -> Self {
        let mut m = Self::identity();
        m.data[0][0] = sx;
        m.data[1][1] = sy;
        m.data[2][2] = sz;
        m
    }

    /// Rotación alrededor del eje X
    pub fn rotation_x(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        let mut m = Self::identity();
        m.data[1][1] = c;
        m.data[1][2] = -s;
        m.data[2][1] = s;
        m.data[2][2] = c;
        m
    }

    /// Rotación alrededor del eje Y
    pub fn rotation_y(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        let mut m = Self::identity();
        m.data[0][0] = c;
        m.data[0][2] = s;
        m.data[2][0] = -s;
        m.data[2][2] = c;
        m
    }

    /// Rotación alrededor del eje Z
    pub fn rotation_z(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        let mut m = Self::identity();
        m.data[0][0] = c;
        m.data[0][1] = -s;
        m.data[1][0] = s;
        m.data[1][1] = c;
        m
    }

    /// Matriz de proyección en perspectiva
    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let mut m = Self::zero();
        let tan_half_fov = (fov / 2.0).tan();

        m.data[0][0] = 1.0 / (aspect * tan_half_fov);
        m.data[1][1] = 1.0 / tan_half_fov;
        m.data[2][2] = -(far + near) / (far - near);
        m.data[2][3] = -(2.0 * far * near) / (far - near);
        m.data[3][2] = -1.0;

        m
    }

    /// Matriz de vista (look at)
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let forward = target - eye;
        // Prevenir división por cero si eye == target
        if forward.length() < 0.0001 {
            return Self::identity();
        }
        let f = forward.normalize();
        let r = f.cross(&up).normalize();
        let u = r.cross(&f);

        let mut m = Self::identity();
        m.data[0][0] = r.x;
        m.data[0][1] = r.y;
        m.data[0][2] = r.z;
        m.data[1][0] = u.x;
        m.data[1][1] = u.y;
        m.data[1][2] = u.z;
        m.data[2][0] = -f.x;
        m.data[2][1] = -f.y;
        m.data[2][2] = -f.z;
        m.data[0][3] = -r.dot(&eye);
        m.data[1][3] = -u.dot(&eye);
        m.data[2][3] = f.dot(&eye);

        m
    }

    /// Multiplicación de matrices
    pub fn multiply(&self, other: &Mat4) -> Mat4 {
        let mut result = Mat4::zero();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }

    /// Transformar un punto (Vec3) con la matriz
    pub fn transform_point(&self, p: &Vec3) -> Vec3 {
        let x = self.data[0][0] * p.x + self.data[0][1] * p.y + self.data[0][2] * p.z + self.data[0][3];
        let y = self.data[1][0] * p.x + self.data[1][1] * p.y + self.data[1][2] * p.z + self.data[1][3];
        let z = self.data[2][0] * p.x + self.data[2][1] * p.y + self.data[2][2] * p.z + self.data[2][3];
        let w = self.data[3][0] * p.x + self.data[3][1] * p.y + self.data[3][2] * p.z + self.data[3][3];

        if w != 0.0 && w != 1.0 {
            Vec3::new(x / w, y / w, z / w)
        } else {
            Vec3::new(x, y, z)
        }
    }

    /// Transformar dirección (sin traslación)
    pub fn transform_direction(&self, d: &Vec3) -> Vec3 {
        Vec3::new(
            self.data[0][0] * d.x + self.data[0][1] * d.y + self.data[0][2] * d.z,
            self.data[1][0] * d.x + self.data[1][1] * d.y + self.data[1][2] * d.z,
            self.data[2][0] * d.x + self.data[2][1] * d.y + self.data[2][2] * d.z,
        )
    }
}