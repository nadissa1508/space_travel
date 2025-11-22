# Space Travel - Sistema Solar

Software renderer de un sistema solar ficticio desarrollado en Rust.

## Demo

![Funcionamiento del Sistema Solar](screenshots/funcionamiento.gif)

### Video Demo

https://youtu.be/z-Ace6WUDcE

## Descripción

Este proyecto que simula un sistema solar con:

- **1 Estrella** (Sol heart) - Cuerpo emisivo que ilumina el sistema
- **5 Planetas** orbitando en el plano eclíptico:
  - Lava 
  - Rocky 
  - Alien 
  - Gas Giant 
  - Ice 

  Elegí no hacer un sistema solar tradicional

## Características Implementadas

- Órbitas circulares visibles
- Rotación de planetas sobre su eje
- Traslación orbital
- Cámara móvil en el plano eclíptico
- Cambio de objetivo entre planetas
- Renderizado por rasterización con z-buffer
- Iluminación difusa básica


## Controles

| Tecla | Acción |
|-------|--------|
| W | Acercar cámara |
| S | Alejar cámara |
| A | Rotar cámara izquierda |
| D | Rotar cámara derecha |
| 1-6 | Cambiar planeta objetivo |
| ESC | Salir |

## Compilación y Ejecución

```bash
# Compilar en modo release (recomendado para mejor rendimiento)
cargo build --release

# Ejecutar
cargo run --release
```

## Estructura del Proyecto

```
space_travel/
├── src/
│   ├── main.rs                    # Loop principal, renderizado y lógica del juego
│   ├── lib.rs                     # Punto de entrada de la biblioteca
│   │
│   ├── camera/                    # Sistema de cámara
│   │   ├── mod.rs                 # Módulo de exportación
│   │   └── camera.rs              # Implementación de cámara orbital y warp
│   │
│   ├── math/                      # Sistema matemático personalizado
│   │   ├── mod.rs                 # Módulo de exportación
│   │   ├── vec3.rs                # Vectores 3D
│   │   ├── mat4.rs                # Matrices 4x4
│   │   └── transforms.rs          # Transformaciones (model, view, projection)
│   │
│   ├── renderer/                  # Sistema de renderizado por software
│   │   ├── mod.rs                 # Módulo de exportación
│   │   ├── framebuffer.rs         # Buffer de píxeles y depth buffer
│   │   ├── vertex.rs              # Estructura de vértice
│   │   ├── triangle.rs            # Rasterización de triángulos
│   │   ├── shapes.rs              # Generación de geometría (esferas, órbitas)
│   │   ├── shader.rs              # Sistema de shaders (vertex y fragment)
│   │   └── skybox.rs              # Renderizado de skybox con estrellas
│   │
│   ├── scene/                     # Escena del sistema solar
│   │   ├── mod.rs                 # Módulo de exportación
│   │   ├── celestial_body.rs      # Cuerpo celeste (planetas, estrellas)
│   │   ├── solar_system.rs        # Sistema solar con todos los cuerpos
│   │   └── orbit.rs               # Sistema de órbitas circulares
│   │
│   ├── shaders/                   # Shaders procedurales por planeta
│   │   ├── mod.rs                 # Módulo de exportación y tipos
│   │   ├── common.rs              # Funciones comunes (noise, fbm, hash)
│   │   ├── solar_heart.rs         # Shader del sol (emisivo con corazones)
│   │   ├── rocky.rs               # Shader de planeta rocoso
│   │   ├── gas_giant.rs           # Shader de gigante gaseoso
│   │   ├── ice.rs                 # Shader de planeta helado
│   │   ├── lava.rs                # Shader de planeta de lava
│   │   └── alien.rs               # Shader de planeta alienígena
│   │
│   └── threading/                 # Sistema de multithreading
│       ├── mod.rs                 # Módulo de exportación
│       └── planet_thread.rs       # Renderizado paralelo de planetas
│
├── assets/                        # Recursos del proyecto
│   ├── models/                    # Modelos 3D (opcional)
│   └── textures/                  # Texturas (opcional)
│
├── screenshots/                   # Capturas de pantalla del proyecto
│
├── Cargo.toml                     # Dependencias y configuración
├── Cargo.lock                     # Versiones exactas de dependencias
└── README.md                      # Documentación del proyecto
```

### Optimizaciones

- **Multithreading con Rayon**: El renderizado de planetas se realiza en paralelo
- **Profile Release**: Optimización nivel 3 con LTO (Link Time Optimization)
- **Z-Buffer**: Depth testing para correcta visibilidad de objetos
- **Shaders Procedurales**: Texturas generadas matemáticamente en tiempo real

