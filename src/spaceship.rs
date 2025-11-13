use nalgebra::Vector3;

pub struct Spaceship {
    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: f32,
    pub mesh: Vec<Vertex>,
    pub offset_from_camera: Vector3,  // Offset relativo a la cámara
}

impl Spaceship {
    pub fn new(mesh: Vec<Vertex>) -> Self {
        Spaceship {
            position: Vector3::zeros(),
            rotation: Vector3::zeros(),
            scale: 0.1,
            mesh,
            offset_from_camera: Vector3::new(0.5, -0.3, -2.0),  // Derecha, abajo, adelante
        }
    }
    
    pub fn follow_camera(&mut self, camera: &Camera) {
        // La nave sigue la cámara con un offset
        let camera_forward = normalize(camera.target - camera.eye);
        let camera_right = normalize(cross(camera_forward, camera.up));
        let camera_up = camera.up;
        
        self.position = camera.eye
            + camera_right * self.offset_from_camera.x
            + camera_up * self.offset_from_camera.y
            + camera_forward * self.offset_from_camera.z;
        
        // Rotar la nave para que apunte en la dirección de la cámara
        self.rotation.y = camera.yaw;
    }
}