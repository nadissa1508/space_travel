// skybox.rs - Skybox rendering
use crate::vertex::Vertex;
use raylib::prelude::*;

pub struct Skybox {
    pub vertices: Vec<Vertex>,
}

impl Skybox {
    pub fn new() -> Self {
        // Create a large cube for the skybox
        // The skybox is a large cube that surrounds the scene
        let size = 50.0; // Large enough to contain the solar system
        
        let mut vertices = Vec::new();
        
        // We'll create a cube with 6 faces (12 triangles, 36 vertices)
        // Each face will use the same texture
        
        // Front face (+Z)
        Self::add_face(&mut vertices, 
            Vector3::new(-size, -size, size),
            Vector3::new(size, -size, size),
            Vector3::new(size, size, size),
            Vector3::new(-size, size, size)
        );
        
        // Back face (-Z)
        Self::add_face(&mut vertices,
            Vector3::new(size, -size, -size),
            Vector3::new(-size, -size, -size),
            Vector3::new(-size, size, -size),
            Vector3::new(size, size, -size)
        );
        
        // Right face (+X)
        Self::add_face(&mut vertices,
            Vector3::new(size, -size, size),
            Vector3::new(size, -size, -size),
            Vector3::new(size, size, -size),
            Vector3::new(size, size, size)
        );
        
        // Left face (-X)
        Self::add_face(&mut vertices,
            Vector3::new(-size, -size, -size),
            Vector3::new(-size, -size, size),
            Vector3::new(-size, size, size),
            Vector3::new(-size, size, -size)
        );
        
        // Top face (+Y)
        Self::add_face(&mut vertices,
            Vector3::new(-size, size, size),
            Vector3::new(size, size, size),
            Vector3::new(size, size, -size),
            Vector3::new(-size, size, -size)
        );
        
        // Bottom face (-Y)
        Self::add_face(&mut vertices,
            Vector3::new(-size, -size, -size),
            Vector3::new(size, -size, -size),
            Vector3::new(size, -size, size),
            Vector3::new(-size, -size, size)
        );
        
        Skybox { vertices }
    }
    
    fn add_face(vertices: &mut Vec<Vertex>, v0: Vector3, v1: Vector3, v2: Vector3, v3: Vector3) {
        let normal = Vector3::new(0.0, 0.0, 1.0); // Will be recalculated
        
        // First triangle (v0, v1, v2)
        vertices.push(Vertex::new(v0, normal, Vector2::new(0.0, 1.0)));
        vertices.push(Vertex::new(v1, normal, Vector2::new(1.0, 1.0)));
        vertices.push(Vertex::new(v2, normal, Vector2::new(1.0, 0.0)));
        
        // Second triangle (v0, v2, v3)
        vertices.push(Vertex::new(v0, normal, Vector2::new(0.0, 1.0)));
        vertices.push(Vertex::new(v2, normal, Vector2::new(1.0, 0.0)));
        vertices.push(Vertex::new(v3, normal, Vector2::new(0.0, 0.0)));
    }
    
    pub fn get_vertex_array(&self) -> &[Vertex] {
        &self.vertices
    }
}
