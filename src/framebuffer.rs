pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub zbuffer: Vec<f32>,
    background_color: u32,
    current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
            zbuffer: vec![f32::INFINITY; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF,
        }
    }
    pub fn line_with_depth(&mut self, x1: usize, y1: usize, z1: f32, x2: usize, y2: usize, z2: f32) {
        let dx = (x2 as isize - x1 as isize).abs();
        let dy = (y2 as isize - y1 as isize).abs();
        let steps = dx.max(dy) as f32;

        let x_step = (x2 as f32 - x1 as f32) / steps;
        let y_step = (y2 as f32 - y1 as f32) / steps;
        let z_step = (z2 - z1) / steps;

        let mut x = x1 as f32;
        let mut y = y1 as f32;
        let mut z = z1;

        for _ in 0..=steps as usize {
            let xi = x.round() as usize;
            let yi = y.round() as usize;

            if xi < self.width && yi < self.height {
                self.point(xi, yi, z);
            }

            x += x_step;
            y += y_step;
            z += z_step;
        }
    }

    
    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
        for depth in self.zbuffer.iter_mut() {
            *depth = f32::INFINITY;
        }
    }

    pub fn point(&mut self, x: usize, y: usize, depth: f32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            if self.zbuffer[index] > depth {
                self.buffer[index] = self.current_color;
                self.zbuffer[index] = depth;
            }
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    /// Método para dibujar una línea usando el algoritmo de Bresenham
    pub fn line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        let mut x0 = x0 as isize;
        let mut y0 = y0 as isize;
        let x1 = x1 as isize;
        let y1 = y1 as isize;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        while x0 != x1 || y0 != y1 {
            // Dibujar punto solo si está dentro de los límites del framebuffer
            if x0 >= 0 && x0 < self.width as isize && y0 >= 0 && y0 < self.height as isize {
                self.point(x0 as usize, y0 as usize, 1.0); // Profundidad fija
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}
