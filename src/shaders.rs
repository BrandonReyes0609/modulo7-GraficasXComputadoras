use nalgebra_glm::{Vec3, Vec4, Mat3, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  // Transform position
  let position = Vec4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );
  let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

  // Perform perspective division
  let w = transformed.w;
  let ndc_position = Vec4::new(
    transformed.x / w,
    transformed.y / w,
    transformed.z / w,
    1.0
  );

  // apply viewport matrix
  let screen_position = uniforms.viewport_matrix * ndc_position;

  // Transform normal
  let model_mat3 = mat4_to_mat3(&uniforms.model_matrix); 
  let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

  let transformed_normal = normal_matrix * vertex.normal;

  // Create a new Vertex with transformed attributes
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
    transformed_normal,
  }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  star(fragment, uniforms)

}



pub fn mercurio(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  //152, 221, 255
  let zoom = 100.0;
  let ox = 0.0;
  let oy = 0.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  let noise_value = uniforms.noise.get_noise_2d(
    (x + ox) * zoom,
    (y + oy) * zoom,
  );

  let spot_threshold = 0.5;
  let spot_color = Color::new(223, 223, 223); // gris
  let base_color = Color::new(  223, 223, 223); // gris
  let noise_color = if noise_value < spot_threshold {
    spot_color
  } else {
    base_color
  };

  noise_color * fragment.intensity
}


pub fn neptuno(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  //152, 221, 255
  let zoom = 100.0;
  let ox = 0.0;
  let oy = 0.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  let noise_value = uniforms.noise.get_noise_2d(
    (x + ox) * zoom,
    (y + oy) * zoom,
  );

  let spot_threshold = 0.5;
  let spot_color = Color::new(152, 221, 255); // 
  let base_color = Color::new(152, 221, 255); // 

  let noise_color = if noise_value < spot_threshold {
    spot_color
  } else {
    base_color
  };

  noise_color * fragment.intensity
}


pub fn luna(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 100.0;
  let ox = 0.0;
  let oy = 0.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  let noise_value = uniforms.noise.get_noise_2d(
    (x + ox) * zoom,
    (y + oy) * zoom,
  );

  let spot_threshold = 0.5;
  let spot_color = Color::new(135, 135, 135); // gris oscuro
  let base_color = Color::new(191, 191, 191); // Black

  let noise_color = if noise_value < spot_threshold {
    spot_color
  } else {
    base_color
  };

  noise_color * fragment.intensity
}



pub fn star(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Colores base para el efecto de la estrella
  //let bright_color = Color::new(255, 240, 0); // Naranja brillante (efecto lava)
  let bright_color = Color::new(255, 253, 190); // Color blanco

  let dark_color = Color::new(255, 193, 108);   // Naranja rojizo oscuro

  // Obtener la posición del fragmento
  let position = Vec3::new(
    fragment.vertex_position.x,
    fragment.vertex_position.y,
    fragment.depth
  );

  // Frecuencia base y amplitud para el efecto de pulsación
  let base_frequency = 0.6;
  let pulsate_amplitude = 0.5;
  let t = uniforms.time as f32 * 0.01;

  // Pulsación en el eje z para cambiar el tamaño del punto
  let pulsate = (t * base_frequency).sin() * pulsate_amplitude;

  // Aplicar ruido a las coordenadas con una pulsación sutil en el eje z
  let zoom = 1000.0; // Factor de zoom constante
  let noise_value1 = uniforms.noise.get_noise_3d(
    position.x * zoom,
    position.y * zoom,
    (position.z + pulsate) * zoom
  );
  let noise_value2 = uniforms.noise.get_noise_3d(
    (position.x + 1000.0) * zoom,
    (position.y + 1000.0) * zoom,
    (position.z + 1000.0 + pulsate) * zoom
  );
  let noise_value = (noise_value1 + noise_value2) * 0.5;  // Promediar el ruido para transiciones más suaves

  // Usar interpolación lineal (lerp) para mezclar colores según el valor del ruido
  let color = dark_color.lerp(&bright_color, noise_value);

  color * fragment.intensity
}

pub fn earth(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 30.0;  // Zoom factor to adjust the scale of the cell pattern
  let ox = 50.0;    // Offset x in the noise map
  let oy = 50.0;    // Offset y in the noise map
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let t = uniforms.time as f32 * 0.5;

  // Use a cellular noise function to create the plant cell pattern
  let cell_noise_value = uniforms.noise.get_noise_2d(x * zoom + ox, y * zoom + oy).abs();

  // Define different shades of green for the plant cells
  let base_color = if cell_noise_value < 0.15 {
      Color::new(85, 107, 47)   // Dark olive green
  } else if cell_noise_value < 0.7 {
      Color::new(2, 100, 177)   // Celeste
  } else if cell_noise_value < 0.75 {
      Color::new(85, 107, 47)   // Forest green
  } else {
      Color::new(133, 98, 57)   // Cafe
  };

  // Add cloud effect with blending
  let cloud_zoom = 100.0;  // to move our values 
  let cloud_ox = 100.0; // offset x in the noise map
  let cloud_oy = 100.0;
  let cloud_noise_value = uniforms.noise.get_noise_2d(x * cloud_zoom + cloud_ox + t, y * cloud_zoom + cloud_oy);
  let cloud_threshold = 0.5; // Adjust this value to change cloud density
  let cloud_color = Color::new(255, 255, 255); // White for clouds

  let blended_color = if cloud_noise_value > cloud_threshold {
      // Blend the cloud color with the base color using a blending factor to keep the base visible
      base_color.blend_normal(&cloud_color) * 0.5 + base_color * 0.5
  } else {
      base_color
  };

  // Adjust intensity to simulate lighting effects (optional)
  blended_color * fragment.intensity
}

pub fn saturno(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 50.0;
  let ox = 0.0;
  let oy = 0.0;
  let y = fragment.vertex_position.y;

  let noise_value = uniforms.noise.get_noise_2d(
      (y + oy) * zoom,
      (ox) * zoom,
  );

  let spot_threshold = 0.0;
  let spot_color = Color::new(255, 233, 11); //amarillo-naranja
  let base_color = Color::new(224, 142, 104);       // naranja

  let noise_color = if noise_value.sin() > spot_threshold {
      spot_color
  } else {
      base_color
  };

  noise_color * fragment.intensity
}


pub fn marte(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 50.0;
  let ox = 0.0;
  let oy = 0.0;
  let y = fragment.vertex_position.y;

  let noise_value = uniforms.noise.get_noise_2d(
      (y + oy) * zoom,
      (ox) * zoom,
  );

  let spot_threshold = 0.0;
  let spot_color = Color::new(143, 78, 54); //amarillo-naranja
  let base_color = Color::new(204, 22, 0);       // naranja

  let noise_color = if noise_value.sin() > spot_threshold {
      spot_color
  } else {
      base_color
  };

  noise_color * fragment.intensity
}


pub fn urano1(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 50.0;
  let ox = 0.0;
  let oy = 0.0;
  let y = fragment.vertex_position.y;

  let noise_value = uniforms.noise.get_noise_2d(
      (y + oy) * zoom,
      (ox) * zoom,
  );

  let spot_threshold = 0.0;
  let spot_color = Color::new(0, 255, 212); //celeste claro
  let base_color = Color::new(0, 220, 255);       // celeste

  let noise_color = if noise_value.sin() > spot_threshold {
      spot_color
  } else {
      base_color
  };

  noise_color * fragment.intensity
}


pub fn planetaE1(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 50.0;
  let ox = 0.0;
  let oy = 0.0;
  let y = fragment.vertex_position.y;

  let noise_value = uniforms.noise.get_noise_2d(
      (y + oy) * zoom,
      (ox) * zoom,
  );

  let spot_threshold = 0.0;
  let spot_color = Color::new(131, 255, 0); //celeste claro
  let base_color = Color::new(131, 255, 0);       // celeste

  let noise_color = if noise_value.sin() > spot_threshold {
      spot_color
  } else {
      base_color
  };

  noise_color * fragment.intensity
}


pub fn planetaE2(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 50.0;
  let ox = 0.0;
  let oy = 0.0;
  let y = fragment.vertex_position.y;

  let noise_value = uniforms.noise.get_noise_2d(
      (y + oy) * zoom,
      (ox) * zoom,
  );

  let spot_threshold = 0.0;
  let spot_color = Color::new(255, 0, 243); 
  let base_color = Color::new(255, 0, 243);       

  let noise_color = if noise_value.sin() > spot_threshold {
      spot_color
  } else {
      base_color
  };

  noise_color * fragment.intensity
}

pub fn planetaE3(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 50.0;
  let ox = 0.0;
  let oy = 0.0;
  let y = fragment.vertex_position.y;

  let noise_value = uniforms.noise.get_noise_2d(
      (y + oy) * zoom,
      (ox) * zoom,
  );

  let spot_threshold = 0.0;
  let spot_color = Color::new(166, 0, 255); 
  let base_color = Color::new(166, 0, 255);       

  let noise_color = if noise_value.sin() > spot_threshold {
      spot_color
  } else {
      base_color
  };

  noise_color * fragment.intensity
}

pub fn venus(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 50.0;
  let ox = 0.0;
  let oy = 0.0;
  let y = fragment.vertex_position.y;

  let noise_value = uniforms.noise.get_noise_2d(
      (y + oy) * zoom,
      (ox) * zoom,
  );

  let spot_threshold = 0.0;
  let spot_color = Color::new(255, 0, 243); 
  let base_color = Color::new(255, 0, 243);       

  let noise_color = if noise_value.sin() > spot_threshold {
      spot_color
  } else {
      base_color
  };

  noise_color * fragment.intensity
}

pub fn jupiter(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 50.0;
  let ox = 0.0;
  let oy = 0.0;
  let y = fragment.vertex_position.y;

  let noise_value = uniforms.noise.get_noise_2d(
      (y + oy) * zoom,
      (ox) * zoom,
  );

  let spot_threshold = 0.0;
  let spot_color = Color::new(255, 0, 243); 
  let base_color = Color::new(255, 0, 243);       

  let noise_color = if noise_value.sin() > spot_threshold {
      spot_color
  } else {
      base_color
  };

  noise_color * fragment.intensity
}