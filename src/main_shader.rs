use raylib::prelude::*;

// ============================================================================
// SHADER SOURCE CODE
// ============================================================================

// Base Vertex Shader (used by all planets)
const VERTEX_SHADER: &str = r#"
#version 330

in vec3 vertexPosition;
in vec3 vertexNormal;

uniform mat4 mvp;
uniform mat4 matModel;
uniform mat4 matNormal;

out vec3 fragPosition;
out vec3 fragNormal;

void main() {
    fragPosition = vec3(matModel * vec4(vertexPosition, 1.0));
    fragNormal = normalize(vec3(matNormal * vec4(vertexNormal, 0.0)));
    gl_Position = mvp * vec4(vertexPosition, 1.0);
}
"#;

// Rocky Planet Fragment Shader
const ROCKY_FRAGMENT_SHADER: &str = r#"
#version 330

in vec3 fragPosition;
in vec3 fragNormal;

out vec4 finalColor;

uniform vec4 colDiffuse;
uniform float time;

// Simple hash function for pseudo-random noise
float hash(vec3 p) {
    p = fract(p * 0.3183099 + 0.1);
    p *= 17.0;
    return fract(p.x * p.y * p.z * (p.x + p.y + p.z));
}

// 3D noise function
float noise(vec3 x) {
    vec3 i = floor(x);
    vec3 f = fract(x);
    f = f * f * (3.0 - 2.0 * f);

    return mix(
        mix(mix(hash(i + vec3(0,0,0)), hash(i + vec3(1,0,0)), f.x),
            mix(hash(i + vec3(0,1,0)), hash(i + vec3(1,1,0)), f.x), f.y),
        mix(mix(hash(i + vec3(0,0,1)), hash(i + vec3(1,0,1)), f.x),
            mix(hash(i + vec3(0,1,1)), hash(i + vec3(1,1,1)), f.x), f.y),
        f.z
    );
}

// Fractal Brownian Motion
float fbm(vec3 p) {
    float value = 0.0;
    float amplitude = 0.5;
    float frequency = 1.0;

    for (int i = 0; i < 5; i++) {
        value += amplitude * noise(p * frequency);
        frequency *= 2.0;
        amplitude *= 0.5;
    }

    return value;
}

void main() {
    vec3 lightDir = normalize(vec3(1.0, 1.0, 2.0));
    vec3 normal = normalize(fragNormal);

    // Calculate noise-based terrain
    float terrain = fbm(fragPosition * 3.0);
    float craters = fbm(fragPosition * 8.0);

    // Color variation based on terrain height
    vec3 color1 = vec3(0.3, 0.25, 0.2);  // Dark brown
    vec3 color2 = vec3(0.6, 0.5, 0.4);   // Light brown
    vec3 color3 = vec3(0.4, 0.35, 0.3);  // Medium brown

    vec3 baseColor = mix(color1, color2, terrain);
    baseColor = mix(baseColor, color3, craters * 0.5);

    // Add some rocky variation
    float rockDetail = noise(fragPosition * 20.0);
    baseColor += vec3(rockDetail * 0.1);

    // Lighting
    float diffuse = max(dot(normal, lightDir), 0.0);
    float ambient = 0.3;

    vec3 finalRGB = baseColor * (ambient + diffuse * 0.7);

    finalColor = vec4(finalRGB, 1.0);
}
"#;

// Gas Giant Fragment Shader
const GAS_GIANT_FRAGMENT_SHADER: &str = r#"
#version 330

in vec3 fragPosition;
in vec3 fragNormal;

out vec4 finalColor;

uniform vec4 colDiffuse;
uniform float time;

// Simple hash function
float hash(vec3 p) {
    p = fract(p * 0.3183099 + 0.1);
    p *= 17.0;
    return fract(p.x * p.y * p.z * (p.x + p.y + p.z));
}

// 3D noise
float noise(vec3 x) {
    vec3 i = floor(x);
    vec3 f = fract(x);
    f = f * f * (3.0 - 2.0 * f);

    return mix(
        mix(mix(hash(i + vec3(0,0,0)), hash(i + vec3(1,0,0)), f.x),
            mix(hash(i + vec3(0,1,0)), hash(i + vec3(1,1,0)), f.x), f.y),
        mix(mix(hash(i + vec3(0,0,1)), hash(i + vec3(1,0,1)), f.x),
            mix(hash(i + vec3(0,1,1)), hash(i + vec3(1,1,1)), f.x), f.y),
        f.z
    );
}

void main() {
    vec3 lightDir = normalize(vec3(1.0, 1.0, 2.0));
    vec3 normal = normalize(fragNormal);

    // Create horizontal bands based on Y position
    float bands = fragPosition.y * 5.0 + time * 0.2;

    // Add swirling effect with noise
    float swirl = noise(vec3(fragPosition.x * 2.0, fragPosition.y * 8.0 + time * 0.1, fragPosition.z * 2.0));
    bands += swirl * 2.0;

    // More turbulence
    float turbulence = noise(vec3(fragPosition.x * 6.0 + time * 0.15, fragPosition.y * 15.0, fragPosition.z * 6.0));
    bands += turbulence * 0.8;

    // Color bands (Jupiter-like colors)
    vec3 color1 = vec3(0.9, 0.7, 0.5);   // Light orange
    vec3 color2 = vec3(0.7, 0.5, 0.3);   // Dark orange
    vec3 color3 = vec3(0.95, 0.85, 0.7); // Cream
    vec3 color4 = vec3(0.6, 0.4, 0.25);  // Brown

    float bandPattern = fract(bands);
    vec3 baseColor;

    if (bandPattern < 0.25) {
        baseColor = mix(color1, color2, bandPattern * 4.0);
    } else if (bandPattern < 0.5) {
        baseColor = mix(color2, color3, (bandPattern - 0.25) * 4.0);
    } else if (bandPattern < 0.75) {
        baseColor = mix(color3, color4, (bandPattern - 0.5) * 4.0);
    } else {
        baseColor = mix(color4, color1, (bandPattern - 0.75) * 4.0);
    }

    // Add atmospheric glow
    float atmosphere = pow(1.0 - abs(dot(normal, vec3(0.0, 0.0, 1.0))), 2.0);
    baseColor += vec3(0.1, 0.05, 0.0) * atmosphere;

    // Lighting
    float diffuse = max(dot(normal, lightDir), 0.0);
    float ambient = 0.4;

    vec3 finalRGB = baseColor * (ambient + diffuse * 0.6);

    finalColor = vec4(finalRGB, 1.0);
}
"#;

// Sci-Fi Planet Fragment Shader (Bioluminescent/Glowing)
const SCIFI_FRAGMENT_SHADER: &str = r#"
#version 330

in vec3 fragPosition;
in vec3 fragNormal;

out vec4 finalColor;

uniform vec4 colDiffuse;
uniform float time;

// Hash function
float hash(vec3 p) {
    p = fract(p * 0.3183099 + 0.1);
    p *= 17.0;
    return fract(p.x * p.y * p.z * (p.x + p.y + p.z));
}

// 3D noise
float noise(vec3 x) {
    vec3 i = floor(x);
    vec3 f = fract(x);
    f = f * f * (3.0 - 2.0 * f);

    return mix(
        mix(mix(hash(i + vec3(0,0,0)), hash(i + vec3(1,0,0)), f.x),
            mix(hash(i + vec3(0,1,0)), hash(i + vec3(1,1,0)), f.x), f.y),
        mix(mix(hash(i + vec3(0,0,1)), hash(i + vec3(1,0,1)), f.x),
            mix(hash(i + vec3(0,1,1)), hash(i + vec3(1,1,1)), f.x), f.y),
        f.z
    );
}

// FBM for more complex patterns
float fbm(vec3 p) {
    float value = 0.0;
    float amplitude = 0.5;
    float frequency = 1.0;

    for (int i = 0; i < 4; i++) {
        value += amplitude * noise(p * frequency);
        frequency *= 2.0;
        amplitude *= 0.5;
    }

    return value;
}

void main() {
    vec3 lightDir = normalize(vec3(1.0, 1.0, 2.0));
    vec3 normal = normalize(fragNormal);

    // Create pulsating bioluminescent patterns
    float pulse = sin(time * 2.0) * 0.5 + 0.5;

    // Organic vein-like patterns
    vec3 samplePos = fragPosition * 8.0;
    samplePos.x += time * 0.3;
    float veins = fbm(samplePos);

    // Secondary pattern that pulses
    float spots = noise(fragPosition * 15.0 + vec3(time * 0.5, 0.0, 0.0));
    spots = pow(spots, 3.0);

    // Tertiary swirling pattern
    vec3 swirl = vec3(
        fragPosition.x + sin(fragPosition.y * 10.0 + time) * 0.1,
        fragPosition.y,
        fragPosition.z + cos(fragPosition.x * 10.0 + time) * 0.1
    );
    float swirls = fbm(swirl * 5.0);

    // Bioluminescent colors
    vec3 glowColor1 = vec3(0.0, 0.8, 1.0);   // Cyan
    vec3 glowColor2 = vec3(0.5, 0.0, 1.0);   // Purple
    vec3 glowColor3 = vec3(0.0, 1.0, 0.5);   // Green-cyan
    vec3 darkColor = vec3(0.05, 0.0, 0.15);  // Dark purple

    // Combine patterns
    vec3 baseColor = darkColor;

    // Add glowing veins
    if (veins > 0.6) {
        float intensity = (veins - 0.6) * 2.5;
        baseColor = mix(baseColor, glowColor1, intensity * pulse);
    }

    // Add pulsating spots
    if (spots > 0.7) {
        float intensity = (spots - 0.7) * 3.0;
        baseColor = mix(baseColor, glowColor2, intensity * (1.0 - pulse * 0.5));
    }

    // Add swirling patterns
    if (swirls > 0.65) {
        float intensity = (swirls - 0.65) * 2.8;
        baseColor = mix(baseColor, glowColor3, intensity * abs(sin(time * 1.5)));
    }

    // Edge glow effect
    float edgeGlow = pow(1.0 - abs(dot(normal, vec3(0.0, 0.0, 1.0))), 3.0);
    baseColor += glowColor1 * edgeGlow * 0.5 * pulse;

    // Minimal lighting (since it's bioluminescent)
    float diffuse = max(dot(normal, lightDir), 0.0);
    float ambient = 0.6;

    vec3 finalRGB = baseColor * (ambient + diffuse * 0.4);

    // Add overall glow
    finalRGB += baseColor * 0.3 * pulse;

    finalColor = vec4(finalRGB, 1.0);
}
"#;

// ============================================================================
// SPHERE GENERATION
// ============================================================================

fn generate_sphere_mesh(_rl: &mut RaylibHandle, _thread: &RaylibThread, radius: f32, rings: i32, slices: i32) -> Mesh {
    unsafe {
        let mut mesh = raylib::ffi::GenMeshSphere(radius, rings, slices);
        raylib::ffi::UploadMesh(&mut mesh as *mut raylib::ffi::Mesh, false);
        Mesh::from_raw(mesh)
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    const SCREEN_WIDTH: i32 = 1280;
    const SCREEN_HEIGHT: i32 = 720;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Shader Planets - Press 1, 2, 3 to switch")
        .msaa_4x()
        .build();

    rl.set_target_fps(60);

    // Create camera
    let camera = Camera3D::perspective(
        Vector3::new(0.0, 2.0, 6.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    // Generate sphere mesh
    let sphere = generate_sphere_mesh(&mut rl, &thread, 1.5, 32, 32);

    // Load shaders
    let mut rocky_shader = rl.load_shader_from_memory(&thread, Some(VERTEX_SHADER), Some(ROCKY_FRAGMENT_SHADER));
    let mut gas_giant_shader = rl.load_shader_from_memory(&thread, Some(VERTEX_SHADER), Some(GAS_GIANT_FRAGMENT_SHADER));
    let mut scifi_shader = rl.load_shader_from_memory(&thread, Some(VERTEX_SHADER), Some(SCIFI_FRAGMENT_SHADER));

    // Get time uniform locations
    let rocky_time_loc = rocky_shader.get_shader_location("time");
    let gas_time_loc = gas_giant_shader.get_shader_location("time");
    let scifi_time_loc = scifi_shader.get_shader_location("time");

    // Create materials and assign shaders
    let mut rocky_material = rl.load_material_default(&thread);
    let mut gas_giant_material = rl.load_material_default(&thread);
    let mut scifi_material = rl.load_material_default(&thread);

    // Manually set shader in the raw ffi::Material
    unsafe {
        let rocky_ptr = rocky_material.as_mut() as *mut raylib::ffi::Material;
        (*rocky_ptr).shader = *rocky_shader;

        let gas_ptr = gas_giant_material.as_mut() as *mut raylib::ffi::Material;
        (*gas_ptr).shader = *gas_giant_shader;

        let scifi_ptr = scifi_material.as_mut() as *mut raylib::ffi::Material;
        (*scifi_ptr).shader = *scifi_shader;
    }

    // Current shader selection (0 = rocky, 1 = gas giant, 2 = sci-fi)
    let mut current_shader = 0;

    // Animation variables
    let mut rotation_angle = 0.0f32;
    let mut orbit_angle = 0.0f32;
    let mut time = 0.0f32;

    // Main loop
    while !rl.window_should_close() {
        // Update
        time += rl.get_frame_time();
        rotation_angle += 20.0 * rl.get_frame_time();
        orbit_angle += 10.0 * rl.get_frame_time();

        // Handle input
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            current_shader = 0;
            println!("Switched to Rocky Planet shader");
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            current_shader = 1;
            println!("Switched to Gas Giant shader");
        }
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
            current_shader = 2;
            println!("Switched to Sci-Fi Planet shader");
        }

        // Update time uniform for current shader
        match current_shader {
            0 => {
                rocky_shader.set_shader_value(rocky_time_loc, time);
            }
            1 => {
                gas_giant_shader.set_shader_value(gas_time_loc, time);
            }
            2 => {
                scifi_shader.set_shader_value(scifi_time_loc, time);
            }
            _ => {}
        }

        // Calculate planet position (orbit)
        let orbit_radius = 0.5;
        let planet_x = orbit_radius * orbit_angle.to_radians().cos();
        let planet_z = orbit_radius * orbit_angle.to_radians().sin();

        // Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(10, 10, 20, 255));

        {
            let mut d3 = d.begin_mode3D(&camera);

            // Draw planet with current shader
            let material = match current_shader {
                0 => rocky_material.clone(),
                1 => gas_giant_material.clone(),
                2 => scifi_material.clone(),
                _ => rocky_material.clone(),
            };

            // Planet rotation and position
            let position = Vector3::new(planet_x, 0.0, planet_z);
            let rotation_axis = Vector3::new(0.0, 1.0, 0.2);

            d3.draw_mesh(
                &sphere,
                material,
                Matrix::rotate(rotation_axis.normalized(), rotation_angle.to_radians()) *
                Matrix::translate(position.x, position.y, position.z)
            );

            // Draw grid for reference
            d3.draw_grid(10, 1.0);
        }

        // Draw UI
        let shader_name = match current_shader {
            0 => "Rocky Planet [1]",
            1 => "Gas Giant [2]",
            2 => "Sci-Fi Planet [3]",
            _ => "Unknown",
        };

        d.draw_text(shader_name, 10, 10, 20, Color::WHITE);
        d.draw_text("Press 1, 2, or 3 to switch shaders", 10, 40, 20, Color::LIGHTGRAY);
        d.draw_fps(SCREEN_WIDTH - 100, 10);
    }
}
