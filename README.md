# Lab 4: Carga de modelos


## 🧱 Estructura del Proyecto

```
src/
 ├── main.rs          # Lógica principal, render loop
 ├── framebuffer.rs   # Framebuffer + funciones de dibujo y guardado
 ├── triangle.rs      # Rasterizador de triángulos
 ├── vertex.rs        # Estructura de vértices
 ├── fragment.rs      # Representa un fragmento (píxel)
 ├── shaders.rs       # Vertex shader básico
 ├── obj.rs           # Carga de modelos .obj
 └── matrix.rs        # Utilidades de matrices 4x4
```

---

## ⚙️ Requisitos

- **Rust** (>= 1.70)
- **Raylib** (instalada en el sistema)
- Dependencias en `Cargo.toml`:
  ```toml
  [dependencies]
  raylib = "3.7"
  tobj = "3.0"
  ```

---

## 🕹️ Controles

| ↑ / ↓ / ← / → | Mover el modelo |

| A / S | Escalar |

| Q / W / E / R | Rotar |

| **P** | Guardar imagen (`render.png`) |

---

## 🖼️ Guardar Imagen

El framebuffer puede exportarse a una imagen **PNG** o **JPG**.  
Cuando presiones **P**, el programa generará el archivo:

```
render.png
```

Este archivo se guardará en la carpeta raíz del proyecto (junto al `Cargo.toml`).

**Ejemplo:**

![Render de ejemplo](render.png)

---

## 📦 Ejecución

Coloca tu modelo `.obj` en la carpeta `src/` y asegúrate de nombrarlo:

```
nave_espacial.obj
```

Luego ejecuta:

```bash
cargo run
```

Y presiona **P** para guardar una captura del render.

---
