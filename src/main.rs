mod framebuffer;
mod triangle;
mod vertex;
mod fragment;
mod shaders;
mod obj;
mod matrix;

use framebuffer::Framebuffer;
use shaders::vertex_shader;
use obj::Obj;
use raylib::prelude::*;
use std::time::Duration;
use std::thread;
use std::f32::consts::PI;
use crate::matrix::new_matrix4;

pub struct Uniforms {
    pub model_matrix: Matrix,
    pub shader_type: u32,
    pub base_color1: Vector3,
    pub base_color2: Vector3,
    // NUEVO:
    pub light_intensity: f32,   // difusa (Lambert) para rocoso/gaseoso
    pub ambient_strength: f32,  // luz ambiente mínima
    pub emission_strength: f32, // brillo de la estrella (emisiva)
}

// Genera la matriz de modelo combinando escala, rotación y traslación
fn create_model_matrix(translation: Vector3, scale: f32, rotation: Vector3) -> Matrix {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_x = new_matrix4(
        1.0, 0.0, 0.0, 0.0,
        0.0, cos_x, -sin_x, 0.0,
        0.0, sin_x, cos_x, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    let rotation_y = new_matrix4(
        cos_y, 0.0, sin_y, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -sin_y, 0.0, cos_y, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    let rotation_z = new_matrix4(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,   1.0, 0.0,
        0.0,    0.0,   0.0, 1.0
    );

    let rotation = rotation_z * rotation_y * rotation_x;

    let scale_matrix = new_matrix4(
        scale, 0.0, 0.0, 0.0,
        0.0, scale, 0.0, 0.0,
        0.0, 0.0, scale, 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    let translation_matrix = new_matrix4(
        1.0, 0.0, 0.0, translation.x,
        0.0, 1.0, 0.0, translation.y,
        0.0, 0.0, 1.0, translation.z,
        0.0, 0.0, 0.0, 1.0
    );

    scale_matrix * rotation * translation_matrix
}

// Render principal: recorre caras y dibuja triángulos
fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, obj: &Obj) {
    let mut transformed = Vec::with_capacity(obj.vertices.len());
    for v in &obj.vertices {
        transformed.push(vertex_shader(v, uniforms));
    }

    let mut fragments = Vec::new();
    for face in obj.indices.chunks(3) {
        let v0 = &transformed[face[0] as usize];
        let v1 = &transformed[face[1] as usize];
        let v2 = &transformed[face[2] as usize];
        fragments.extend(triangle::triangle(v0, v1, v2));
    }

    for frag in fragments {
        framebuffer.point(
            frag.position.x as i32,
            frag.position.y as i32,
            frag.color,
        );
    }
}

// Movimiento simple con teclas
fn handle_input(window: &mut RaylibHandle, translation: &mut Vector3, rotation: &mut Vector3, scale: &mut f32) {
    if window.is_key_down(KeyboardKey::KEY_RIGHT) { translation.x += 10.0; }
    if window.is_key_down(KeyboardKey::KEY_LEFT)  { translation.x -= 10.0; }
    if window.is_key_down(KeyboardKey::KEY_UP)    { translation.y -= 10.0; }
    if window.is_key_down(KeyboardKey::KEY_DOWN)  { translation.y += 10.0; }
    if window.is_key_down(KeyboardKey::KEY_S) { *scale += 0.1; }
    if window.is_key_down(KeyboardKey::KEY_A) { *scale -= 0.1; }
    if window.is_key_down(KeyboardKey::KEY_Q) { rotation.x -= PI / 10.0; }
    if window.is_key_down(KeyboardKey::KEY_W) { rotation.x += PI / 10.0; }
    if window.is_key_down(KeyboardKey::KEY_E) { rotation.y -= PI / 10.0; }
    if window.is_key_down(KeyboardKey::KEY_R) { rotation.y += PI / 10.0; }

}




fn main() {
    let (mut window, thread) = raylib::init()
        .size(800, 600)
        .title("Lab 5: Shaders")
        .build();

    let mut framebuffer = Framebuffer::new(800, 600);
    framebuffer.set_background_color(Vector3::new(0.05, 0.05, 0.10)); // espacio
    framebuffer.init_texture(&mut window, &thread);

    // Modelo esfera
    let ring = Obj::load("src/ring.obj").expect("Failed to load ring");
    let obj = Obj::load("src/planeta.obj").expect("Failed to load obj");

    // Transform
    let mut translation = Vector3::new(400.0, 300.0, 0.0);
    let mut rotation = Vector3::new(0.0, 0.0, 0.0);
    let mut scale = 120.0;

    // Shader actual: 0=estrella, 1=rocoso, 2=gaseoso
    let mut shader_type: u32 = 0;

    // -------- Colores fijos --------

    // ☀️ Sol suave amarillo
    let star_color1 = Vector3::new(1.00, 0.90, 0.45); // núcleo claro
    let star_color2 = Vector3::new(0.25, 0.18, 0.08); // borde cálido oscuro

    // 🌎 Tierra estilizada (azul/verde)
    let rocky_color1 = Vector3::new(0.22, 0.55, 0.85);
    let rocky_color2 = Vector3::new(0.05, 0.20, 0.10);

    // ☁️ Gas tipo Júpiter
    let gas_color1 = Vector3::new(0.92, 0.74, 0.46);
    let gas_color2 = Vector3::new(0.62, 0.52, 0.34);

    // 🤖 Cibernético — tonos metálicos y azul eléctrico
let cyber_color1 = Vector3::new(0.15, 0.18, 0.22); // metal
let cyber_color2 = Vector3::new(0.00, 0.75, 1.00); // luz neón azul

// 🌋 Magma — roca oscura + lava
let lava_color1 = Vector3::new(0.75, 0.15, 0.05);  // lava brillante
let lava_color2 = Vector3::new(0.10, 0.02, 0.01);  // roca volcánica

// 🟦 Plano — color sólido simple
let flat_color1 = Vector3::new(0.3, 0.8, 0.4);     // puedes cambiarlo luego
let flat_color2 = flat_color1;                     // igual, sin gradiente


let ring_color_inner = Vector3::new(0.65, 0.60, 0.50);
let ring_color_outer = Vector3::new(0.85, 0.80, 0.70);

    // Iluminación fija
    let light_intensity: f32   = 1.0;
    let ambient_strength: f32  = 0.18;
    let emission_strength: f32 = 1.1; // brillo del sol suave


    let mut moon_angle: f32 = 0.0;
let moon_distance: f32 = 180.0; // qué tan lejos de su planeta
let moon_scale_factor: f32 = 0.35; // tamaño relativo de la luna



    while !window.window_should_close() {
        moon_angle += 0.01; // velocidad de órbita
        // Movimiento/rotación/escala
        handle_input(&mut window, &mut translation, &mut rotation, &mut scale);

        // Cambiar objeto: 1 = sol, 2 = roca, 3 = gaseoso
        if window.is_key_pressed(KeyboardKey::KEY_ONE)   { shader_type = 0; }
        if window.is_key_pressed(KeyboardKey::KEY_TWO)   { shader_type = 1; }
        if window.is_key_pressed(KeyboardKey::KEY_THREE) { shader_type = 2; }
            if window.is_key_pressed(KeyboardKey::KEY_FOUR)  { shader_type = 3; } // cibernético
if window.is_key_pressed(KeyboardKey::KEY_FIVE)  { shader_type = 4; } // magma
if window.is_key_pressed(KeyboardKey::KEY_SIX)   { shader_type = 5; } // plano

        framebuffer.clear();

        let model_matrix = create_model_matrix(translation, scale, rotation);

        // Seleccionar color según cuerpo
        let (base_color1, base_color2) = match shader_type {
    0 => (star_color1,  star_color2),
    1 => (rocky_color1, rocky_color2),
    2 => (gas_color1,   gas_color2),
    3 => (cyber_color1, cyber_color2),
    4 => (lava_color1,  lava_color2),
    5 => (flat_color1,  flat_color2),
    _ => (star_color1,  star_color2),
};

        let uniforms = Uniforms {
            model_matrix,
            shader_type,
            base_color1,
            base_color2,
            light_intensity,
            ambient_strength,
            emission_strength,
        };

        render(&mut framebuffer, &uniforms, &obj);

        if shader_type == 1 {
    moon_angle += 0.01; // velocidad horizontal

    let moon_x = translation.x + moon_distance * moon_angle.cos();
    let moon_y = translation.y; // horizontal, sin componente vertical

    let moon_translation = Vector3::new(moon_x, moon_y, translation.z);
    let moon_rotation = Vector3::new(0.0, moon_angle * 2.0, 0.0);
    let moon_scale = scale * moon_scale_factor;

    let moon_model_matrix = create_model_matrix(moon_translation, moon_scale, moon_rotation);

    let moon_uniforms = Uniforms {
        model_matrix: moon_model_matrix,
        shader_type: 1, // rocoso
        base_color1: Vector3::new(0.7, 0.7, 0.7),
        base_color2: Vector3::new(0.3, 0.3, 0.3),
        light_intensity,
        ambient_strength,
        emission_strength,
    };

    render(&mut framebuffer, &moon_uniforms, &obj);
}

// Si planeta gaseoso, render anillos
if shader_type == 2 {
    // posición del anillo: centrado en el planeta
    let ring_translation = translation;

    // giro lento del anillo
    let mut ring_rotation = rotation;
    ring_rotation.y += 0.3; 

    // escala del anillo; un poco mayor que el planeta
    let ring_scale = scale * 1.8; 

    let ring_model_matrix = create_model_matrix(
        ring_translation,
        ring_scale,
        ring_rotation
    );

    let ring_uniforms = Uniforms {
        model_matrix: ring_model_matrix,
        shader_type: 6, // shader de anillo
        base_color1: ring_color_inner,
        base_color2: ring_color_outer,
        light_intensity,
        ambient_strength,
        emission_strength,
    };

    render(&mut framebuffer, &ring_uniforms, &ring);
}


        // Guardar imagen
        if window.is_key_pressed(KeyboardKey::KEY_P) {
            framebuffer.save_image("space_render.png");
        }

        framebuffer.swap_buffers(&mut window, &thread);

        thread::sleep(Duration::from_millis(16));
    }
}