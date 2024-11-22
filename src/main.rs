use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::time::{Instant, Duration};
use std::f32::consts::PI;
use rand::Rng;
use std::sync::{Arc, Mutex};

mod framebuffer;
mod triangle;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod camera;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use camera::Camera;
use triangle::triangle;
//use shaders::{vertex_shader, star, earth,shader_nave};
use shaders::{vertex_shader, star, luna, neptuno,  mercurio, earth,saturno,marte,urano1,planetaE1,planetaE2,venus,jupiter,shader_nave};

use fragment::Fragment;
use color::Color;
use fastnoise_lite::{FastNoiseLite, NoiseType};

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: FastNoiseLite,
}

fn create_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, cos_x, -sin_x, 0.0,
        0.0, sin_x, cos_x, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y, 0.0, sin_y, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -sin_y, 0.0, cos_y, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z, cos_z, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0, 0.0, translation.x,
        0.0, scale, 0.0, translation.y,
        0.0, 0.0, scale, translation.z,
        0.0, 0.0, 0.0, 1.0,
    );

    transform_matrix * rotation_matrix
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}
fn handle_input(window: &Window, camera: &mut Camera) {
    let movement_speed = 1.0;
    let rotation_speed = PI / 50.0;

    // Movimiento de la cámara
    if window.is_key_down(Key::W) {
        camera.eye.z -= movement_speed;
    }
    if window.is_key_down(Key::S) {
        camera.eye.z += movement_speed;
    }
    if window.is_key_down(Key::A) {
        camera.eye.x -= movement_speed;
    }
    if window.is_key_down(Key::D) {
        camera.eye.x += movement_speed;
    }
    if window.is_key_down(Key::Q) {
        camera.eye.y += movement_speed;
    }
    if window.is_key_down(Key::E) {
        camera.eye.y -= movement_speed;
    }

    // Rotación de la cámara
    if window.is_key_down(Key::Left) {
        camera.orbit(rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Right) {
        camera.orbit(-rotation_speed, 0.0);
    }
    if window.is_key_down(Key::Up) {
        camera.orbit(0.0, -rotation_speed);
    }
    if window.is_key_down(Key::Down) {
        camera.orbit(0.0, rotation_speed);
    }
}

fn handle_nave_input(
    window: &Window,
    nave_pos: &mut Vec3,
    nave_rot: &mut Vec3,
    speed: &mut f32,
    objects: &[(Vec3, f32)],
) {
    let rotation_speed = 0.05;
    let movement_speed = 0.2;
    let mut new_position = *nave_pos;

    // Rotación de la nave (girar)
    if window.is_key_down(Key::Left) {
        nave_rot.y += rotation_speed;
    }
    if window.is_key_down(Key::Right) {
        nave_rot.y -= rotation_speed;
    }

    // Movimiento de la nave (traslación)
    if window.is_key_down(Key::S) {
        new_position.x += nave_rot.y.sin() * movement_speed;
        new_position.z += nave_rot.y.cos() * movement_speed;
    }
    if window.is_key_down(Key::W) {
        new_position.x -= nave_rot.y.sin() * movement_speed;
        new_position.z -= nave_rot.y.cos() * movement_speed;
    }
    if window.is_key_down(Key::D) {
        new_position.x += nave_rot.y.cos() * movement_speed;
        new_position.z -= nave_rot.y.sin() * movement_speed;
    }
    if window.is_key_down(Key::A) {
        new_position.x -= nave_rot.y.cos() * movement_speed;
        new_position.z += nave_rot.y.sin() * movement_speed;
    }

    // Verificar colisión
    if !check_collision(new_position, objects) {
        *nave_pos = new_position;
    }
}

fn calculate_orbital_position(center: Vec3, radius: f32, speed: f32, time: f32) -> Vec3 {
    let angle = time * speed;
    Vec3::new(
        center.x + radius * angle.cos(),
        center.y,
        center.z + radius * angle.sin(),
    )
}

fn render_stars(framebuffer: &mut Framebuffer, star_count: usize) {
    let mut rng = rand::thread_rng();

    for _ in 0..star_count {
        let x = rng.gen_range(0..framebuffer.width);
        let y = rng.gen_range(0..framebuffer.height);
        framebuffer.set_current_color(0xFFFFFF); // Color blanco
        framebuffer.point(x, y, 1.0); // Profundidad de las estrellas arbitraria
    }
}

fn check_collision(position: Vec3, objects: &[(Vec3, f32)]) -> bool {
    for (center, radius) in objects {
        let distance = nalgebra_glm::distance(&position, center);
        if distance < *radius {
            return true;
        }
    }
    false
}



fn main() {
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Nave y Sistema Solar - Movimientos",
        framebuffer_width,
        framebuffer_height,
        WindowOptions::default(),
    )
    .unwrap();

    let mut cam_offset = Vec3::new(0.0, 5.0, 10.0); // Offset detrás y arriba de la nave

    let nave_obj = Obj::load("assets/models/nave_pro1.obj").expect("Failed to load nave_pro.obj");
    let nave_vertex_array = nave_obj.get_vertex_array();

    let obj_sun = Obj::load("assets/models/esfera_pvertices.obj").expect("Failed to load sun model");
    let vertex_arrays_sun = obj_sun.get_vertex_array();



    let obj = Obj::load("assets/models/esfera_pvertices.obj").expect("Failed to load obj");
    let obj1 = Obj::load("assets/models/esfera_pvertices.obj").expect("Failed to load obj");
    //let obj2 = Obj::load("assets/models/esfera_anillo2.obj").expect("Failed to load obj");
    let obj2 = Obj::load("assets/models/esfera_anillo2.obj").expect("Failed to load obj");
    let mut time = 0.0;


    let vertex_arrays = obj.get_vertex_array();       // Planetas y Sol
    let vertex_arrays1 = obj1.get_vertex_array();     // Luna
    let vertex_arrays2 = obj2.get_vertex_array();     // Saturno


    
    let projection_matrix = create_perspective_matrix(framebuffer_width as f32, framebuffer_height as f32);
    let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

    let mut uniforms = Uniforms {
        model_matrix: Mat4::identity(),
        view_matrix: Mat4::identity(),
        projection_matrix,
        viewport_matrix,
        time: 0,
        noise: create_noise(),
    };
    //varaibles planetas
    let shaders = [
        mercurio, // Mercurio
        venus,    // Venus
        earth,    // Tierra
        marte,    // Marte

    ];

    // Radios y velocidades de las órbitas
    //let orbital_radii = [3.0, 5.0, 7.5,9.0];
    let orbital_speeds = [0.02, 0.015, 0.01, 0.008];
    let orbital_radii = [3.0, 5.0, 7.5, 9.0];

    let mut planet_objects: Vec<(Vec3, f32)> = vec![
        (Vec3::new(0.0, 0.0, 0.0), 1.5), // Sol
    ];

    // Agregar planetas dinámicamente
    for (i, &radius) in orbital_radii.iter().enumerate() {
        let size = match i {
            0 => 0.5, // Mercurio
            1 => 0.6, // Venus
            2 => 0.8, // Tierra
            3 => 0.7, // Marte
            _ => 0.5,
        };
        planet_objects.push((Vec3::new(radius, 0.0, 0.0), size));
    }


    // Parámetros para la órbita de la Luna alrededor de la Tierra
    let luna_radius = 2.0;
    let luna_speed = 0.05;

    // Posición y rotación inicial de la nave
    let mut nave_pos = Vec3::new(0.0, 0.0, 10.0);
    let mut nave_rot = Vec3::new(0.0, 0.0, 0.0);
    let mut nave_speed = 0.0;

    let frame_time = Duration::from_secs_f32(1.0 / 60.0);
    let mut last_frame = Instant::now();

    while window.is_open() {
        let now = Instant::now();
        if now - last_frame < frame_time {
            continue;
        }
        last_frame = now;
    
        framebuffer.clear();
        render_stars(&mut framebuffer, 500);
    
        time = (time + 1.0) % 360.0;
        let mut tierra_position = Vec3::new(0.0, 0.0, 0.0);

        // Actualizar las posiciones de los planetas en planet_objects
        planet_objects.clear();
        planet_objects.push((Vec3::new(0.0, 0.0, 0.0), 1.5)); // Sol
        for (i, &radius) in orbital_radii.iter().enumerate() {
            let position = calculate_orbital_position(Vec3::new(0.0, 0.0, 0.0), radius, orbital_speeds[i], time);
            let size = match i {
                0 => 0.5, // Mercurio
                1 => 0.6, // Venus
                2 => 0.8, // Tierra
                3 => 0.7, // Marte
                _ => 0.5,
            };
            planet_objects.push((position, size));
        }
    
        // Procesar entradas de la nave
        let original_position = nave_pos; // Guardar posición original
        handle_nave_input(&window, &mut nave_pos, &mut nave_rot, &mut nave_speed, &planet_objects);
    
        // Verificar colisión y revertir si es necesario
        if check_collision(nave_pos, &planet_objects) {
            nave_pos = original_position; // Revertir posición
        }
    
        // Actualizar la cámara
        let new_cam_eye = nave_pos + cam_offset;
        uniforms.view_matrix = create_view_matrix(new_cam_eye, nave_pos, Vec3::new(0.0, 1.0, 0.0));
    
        // Renderizar el sistema solar
        uniforms.model_matrix = create_model_matrix(Vec3::new(0.0, 0.0, 0.0), 1.5, Vec3::new(0.0, 0.0, 0.0));
        render_with_shader(&mut framebuffer, &uniforms, &vertex_arrays, star);
    
        for (i, &radius) in orbital_radii.iter().enumerate() {
            let position = calculate_orbital_position(Vec3::new(0.0, 0.0, 0.0), radius, orbital_speeds[i], time);
            uniforms.model_matrix = create_model_matrix(position, 1.0, Vec3::new(0.0, 0.0, 0.0));
            render_with_shader(&mut framebuffer, &uniforms, &vertex_arrays, shaders[i]);
    
            if i == 2 {
                tierra_position = position;
            }
        }
    
        // Renderizar la Luna
        let luna_position = calculate_orbital_position(tierra_position, luna_radius, luna_speed, time);
        uniforms.model_matrix = create_model_matrix(luna_position, 0.3, Vec3::new(0.0, 0.0, 0.0));
        render_with_shader(&mut framebuffer, &uniforms, &vertex_arrays1, luna);
    
        // Renderizar Saturno
        let saturno_position = calculate_orbital_position(Vec3::new(0.0, 0.0, 0.0), 15.0, 0.002, time);
        uniforms.model_matrix = create_model_matrix(saturno_position, 1.1, Vec3::new(0.0, 0.0, 0.0));
        render_with_shader(&mut framebuffer, &uniforms, &vertex_arrays2, saturno);
    
        // Renderizar la nave
        uniforms.model_matrix = create_model_matrix(nave_pos, 0.5, nave_rot);
        render_with_shader(&mut framebuffer, &uniforms, &nave_vertex_array, shader_nave);
    
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
    }
}
fn render_with_shader(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    fragment_shader: fn(&Fragment, &Uniforms) -> Color,
) {
    // Vertex Shader Stage
    let transformed_vertices: Vec<_> = vertex_array
        .iter()
        .map(|vertex| vertex_shader(vertex, uniforms))
        .collect();

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            // Apply fragment shader
            let shaded_color = fragment_shader(&fragment, uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}