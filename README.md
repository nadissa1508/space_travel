# Space Travel - Sistema Solar

Software renderer de un sistema solar ficticio desarrollado en Rust.

## Video Demo

[Aquí irá el video del sistema solar en funcionamiento]

## Descripción

Este proyecto implementa un **software renderer por rasterización** (sin GPU) que simula un sistema solar con:

- **1 Estrella** (Sol) - Cuerpo emisivo que ilumina el sistema
- **5 Planetas** orbitando en el plano eclíptico:
  - Ignis (tipo Mercurio)
  - Terra (tipo Tierra)
  - Rubeus (tipo Marte)
  - Magnus (tipo Júpiter)
  - Glacius (tipo Neptuno)

## Características Implementadas

- ✅ Renderizado por rasterización con z-buffer
- ✅ Iluminación difusa básica
- ✅ Órbitas circulares visibles
- ✅ Rotación de planetas sobre su eje
- ✅ Traslación orbital
- ✅ Cámara móvil en el plano eclíptico
- ✅ Cambio de objetivo entre planetas

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
src/
├── main.rs           # Loop principal y renderizado
├── math/             # Vectores y matrices
├── renderer/         # Framebuffer, rasterización, formas
├── camera/           # Sistema de cámara
└── scene/            # Cuerpos celestes y sistema solar
```

## Dependencias

- `minifb` - Ventana y buffer de píxeles
- Sin uso de OpenGL, DirectX u otras APIs gráficas

## Autor

[Tu nombre]

## Licencia

MIT