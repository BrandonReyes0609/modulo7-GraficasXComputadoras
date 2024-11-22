use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
//use minifb::{Key, Window, WindowOptions};
use std::time::{Instant, Duration};
use std::f32::consts::PI;
use rand::Rng;
use std::sync::{Arc, Mutex};
use minifb::{Key, MouseMode, MouseButton, Window, WindowOptions};

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
            return true; // Colisión detectada
        }
    }
    false
}

//----------------------------------
// Estado del mouse
struct MouseState {
    last_position: Option<(f32, f32)>, // Última posición del mouse
    scroll_delta: f32,                // Cambio en el scroll
    middle_button_pressed: bool,      // Botón central del mouse presionado
}

impl MouseState {
    fn new() -> Self {
        Self {
            last_position: None,
            scroll_delta: 0.0,
            middle_button_pressed: false,
        }
    }
}

fn handle_mouse_input(
    window: &Window,
    mouse_state: &mut MouseState,
    cam_offset: &mut Vec3,
    nave_rot: &mut Vec3,
) {
    // Manejar el scroll para acercar/alejar
    if let Some(scroll) = window.get_scroll_wheel() {
        mouse_state.scroll_delta += scroll.1;
    }

    if mouse_state.scroll_delta > 0.0 {
        cam_offset.z -= 1.0; // Acercar zoom
        mouse_state.scroll_delta = 0.0;
    } else if mouse_state.scroll_delta < 0.0 {
        cam_offset.z += 1.0; // Alejar zoom
        mouse_state.scroll_delta = 0.0;
    }

    // Verificar si el botón central está presionado
    mouse_state.middle_button_pressed = window.get_mouse_down(MouseButton::Middle);

    // Manejar el movimiento del mouse para rotar la nave
    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
        if mouse_state.middle_button_pressed {
            if let Some((last_x, last_y)) = mouse_state.last_position {
                let delta_x = x - last_x;
                let delta_y = y - last_y;

                nave_rot.y += delta_x * 0.01; // Rotación horizontal
                nave_rot.x -= delta_y * 0.01; // Rotación vertical
            }
        }
        mouse_state.last_position = Some((x, y));
    } else {
        mouse_state.last_position = None;
    }
}
//----------------------------------
//---------rayos

struct Laser {
    position: Vec3,
    direction: Vec3,
}

fn handle_laser_input(
    window: &Window,
    lasers: &mut Vec<Laser>,
    nave_pos: Vec3,
    nave_rot: Vec3,
) {
    if window.is_key_down(Key::Space) {
        // Offset del láser relativo a la nave
        let laser_offset = Vec3::new(0.0, 0.0, -1.5);
        let rotation_matrix = Mat4::from_euler_angles(nave_rot.x, nave_rot.y, nave_rot.z);
        let rotated_offset = rotation_matrix * laser_offset.to_homogeneous();

        // Calcular posición inicial del láser
        let laser_position = nave_pos + rotated_offset.xyz();

        // Dirección del láser según la rotación de la nave
        let direction = Vec3::new(nave_rot.y.sin(), nave_rot.x.sin(), nave_rot.y.cos());

        // Agregar el láser a la lista
        lasers.push(Laser {
            position: laser_position,
            direction,
        });
    }
}
fn update_lasers(lasers: &mut Vec<Laser>, speed: f32) {
    lasers.iter_mut().for_each(|laser| {
        // Actualizar posición del láser según su dirección
        laser.position += laser.direction * speed;
    });

    // Eliminar láseres fuera del rango visible
    lasers.retain(|laser| laser.position.z > -100.0 && laser.position.z < 100.0);
}
fn render_lasers(framebuffer: &mut Framebuffer, lasers: &[Laser], uniforms: &Uniforms) {
    for laser in lasers {
        // Transformar la posición del láser al espacio de la ventana
        let position_clip = uniforms.projection_matrix * uniforms.view_matrix * laser.position.to_homogeneous();
        if position_clip[3] != 0.0 {
            let position_ndc = Vec3::new(
                position_clip[0] / position_clip[3],
                position_clip[1] / position_clip[3],
                position_clip[2] / position_clip[3],
            );

            let x_screen = ((position_ndc.x + 1.0) * framebuffer.width as f32 * 0.5) as usize;
            let y_screen = ((1.0 - position_ndc.y) * framebuffer.height as f32 * 0.5) as usize;

            // Renderizar el láser como un punto verde fuerte
            if x_screen < framebuffer.width && y_screen < framebuffer.height {
                framebuffer.set_current_color(0x00FF00); // Verde fuerte
                framebuffer.point(x_screen, y_screen, laser.position.z);
            }
        }
    }
}
fn draw_orbit(framebuffer: &mut Framebuffer, center: Vec3, radius: f32, uniforms: &Uniforms) {
    let segments = 100; // Cantidad de segmentos para aproximar el círculo
    let color = 0x00FFFF; // Color celeste (hexadecimal)
    let depth = 0.5; // Profundidad fija para las órbitas (ajusta según sea necesario)

    let angle_increment = 2.0 * PI / segments as f32;

    for i in 0..segments {
        let angle1 = i as f32 * angle_increment;
        let angle2 = (i + 1) as f32 * angle_increment;

        // Coordenadas de los puntos en el círculo (usando el centro y el radio)
        let x1 = center.x + radius * angle1.cos();
        let z1 = center.z + radius * angle1.sin();
        let x2 = center.x + radius * angle2.cos();
        let z2 = center.z + radius * angle2.sin();

        let point1 = Vec3::new(x1, center.y, z1);
        let point2 = Vec3::new(x2, center.y, z2);

        // Transformar puntos a espacio de pantalla
        let clip1 = uniforms.projection_matrix * uniforms.view_matrix * point1.to_homogeneous();
        let clip2 = uniforms.projection_matrix * uniforms.view_matrix * point2.to_homogeneous();

        if clip1[3] != 0.0 && clip2[3] != 0.0 {
            let ndc1 = Vec3::new(clip1[0] / clip1[3], clip1[1] / clip1[3], clip1[2] / clip1[3]);
            let ndc2 = Vec3::new(clip2[0] / clip2[3], clip2[1] / clip2[3], clip2[2] / clip2[3]);

            let x_screen1 = ((ndc1.x + 1.0) * framebuffer.width as f32 * 0.5) as usize;
            let y_screen1 = ((1.0 - ndc1.y) * framebuffer.height as f32 * 0.5) as usize;

            let x_screen2 = ((ndc2.x + 1.0) * framebuffer.width as f32 * 0.5) as usize;
            let y_screen2 = ((1.0 - ndc2.y) * framebuffer.height as f32 * 0.5) as usize;

            // Dibujar líneas entre los puntos adyacentes para formar el círculo
            if x_screen1 < framebuffer.width && y_screen1 < framebuffer.height &&
               x_screen2 < framebuffer.width && y_screen2 < framebuffer.height {
                framebuffer.set_current_color(color);
                framebuffer.line_with_depth(x_screen1, y_screen1, depth, x_screen2, y_screen2, depth);
            }
        }
    }
}


//----------fin rayos
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

    //let mut cam_offset = Vec3::new(0.0, 5.0, 10.0); // Offset detrás y arriba de la nave
    let mut cam_offset = Vec3::new(0.0, 5.0, 20.0);
    let mut nave_pos = Vec3::new(0.0, 0.0, 10.0);
    let mut nave_rot = Vec3::new(0.0, 0.0, 0.0);
    let mut lasers: Vec<Laser> = Vec::new(); // Lista de láseres



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
            0 => 3.5, // Mercurio
            1 => 3.6, // Venus
            2 => 3.8, // Tierra
            3 => 3.7, // Marte
            _ => 3.5,
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


    let mut mouse_state = MouseState::new();
    
    let mut last_frame = Instant::now();
    while window.is_open() {
        let now = Instant::now();
        if now - last_frame < frame_time {
            continue;
        }
        last_frame = now;
    
        framebuffer.clear();
        render_stars(&mut framebuffer, 500);
        handle_mouse_input(&window, &mut mouse_state, &mut cam_offset, &mut nave_rot);
        let new_cam_eye = nave_pos + cam_offset;
        uniforms.view_matrix = create_view_matrix(new_cam_eye, nave_pos, Vec3::new(0.0, 1.0, 0.0));

    
        time = (time + 1.0) % 360.0;
        let mut tierra_position = Vec3::new(0.0, 0.0, 0.0);

        // Actualizar las posiciones de los planetas en planet_objects
        planet_objects.clear();
        planet_objects.push((Vec3::new(0.0, 0.0, 0.0), 1.5)); // Sol
        
        for (i, &radius) in orbital_radii.iter().enumerate() {
            let position = calculate_orbital_position(Vec3::new(0.0, 0.0, 0.0), radius, orbital_speeds[i], time);
            let size = match i {
                0 => 3.5, // Mercurio
                1 => 3.6, // Venus
                2 => 3.8, // Tierra
                3 => 3.7, // Marte
                _ => 3.5,
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
        /// rayos lase
        // Manejar entrada del mouse y teclado
        handle_mouse_input(&window, &mut MouseState::new(), &mut cam_offset, &mut nave_rot);
        handle_laser_input(&window, &mut lasers, nave_pos, nave_rot);

        // Actualizar posiciones de los láseres
        update_lasers(&mut lasers, 0.5);
        
        // fin rayos 
        // Actualizar la cámara
        let new_cam_eye = nave_pos + cam_offset;
        let mut uniforms = Uniforms {
            model_matrix: Mat4::identity(),
            view_matrix: create_view_matrix(new_cam_eye, nave_pos, Vec3::new(0.0, 1.0, 0.0)),
            projection_matrix: create_perspective_matrix(
                framebuffer_width as f32,
                framebuffer_height as f32,
            ),
            viewport_matrix: create_viewport_matrix(
                framebuffer_width as f32,
                framebuffer_height as f32,
            ),
            time: 0,
            noise: create_noise(),
        };


        //for (i, &radius) in orbital_radii.iter().enumerate() {
        //    draw_orbit(&mut framebuffer, Vec3::new(0.0, 0.0, 0.0), radius, &uniforms);
        //
        //    let position = calculate_orbital_position(Vec3::new(0.0, 0.0, 0.0), radius, orbital_speeds[i], time);
        //    uniforms.model_matrix = create_model_matrix(position, 1.0, Vec3::new(0.0, 0.0, 0.0));
        //    render_with_shader(&mut framebuffer, &uniforms, &vertex_arrays, shaders[i]);
        //}
        



        // Renderizar láseres
        render_lasers(&mut framebuffer, &lasers, &uniforms);
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
        //render_with_shader(&mut framebuffer, &uniforms, &[], shader_nave);

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