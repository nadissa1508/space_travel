// collision.rs - Collision detection utilities
use nalgebra::Vector3;

/// Check if two spheres are colliding
pub fn check_sphere_collision(
    pos1: Vector3<f32>,
    radius1: f32,
    pos2: Vector3<f32>,
    radius2: f32
) -> bool {
    let diff = pos1 - pos2;
    let distance_squared = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;
    let radii_sum = radius1 + radius2;
    distance_squared < (radii_sum * radii_sum)
}

/// Normalize a vector (returns unit vector in same direction)
pub fn normalize(v: Vector3<f32>) -> Vector3<f32> {
    let magnitude = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    if magnitude > 0.0 {
        Vector3::new(v.x / magnitude, v.y / magnitude, v.z / magnitude)
    } else {
        v
    }
}

// Example usage (add this code to main.rs in your main loop where you handle camera movement):
// 
// for body in &solar_system.bodies {
//     let body_pos = body.get_world_position();
//     let collision_radius = body.scale * 1.2;  // A bit larger for safety margin
//     
//     if check_sphere_collision(camera.eye, 0.1, body_pos, collision_radius) {
//         // Push camera outside the collision sphere
//         let push_dir = normalize(camera.eye - body_pos);
//         camera.eye = body_pos + push_dir * (collision_radius + 0.1);
//     }
// }