use super::*;
// use crate::math::*;
use pixels::Pixels;
use std::num;

pub struct RenderBuffer {
    pub color: Vec<Vec<Rgb>>,
    pub depth: Vec<Vec<f32>>,
}

impl RenderBuffer {
    pub fn new(width: num::NonZeroUsize, height: num::NonZeroUsize) -> Self {
        Self {
            color: vec![vec![Rgb::default(); width.get()]; height.get()],
            depth: vec![vec![f32::MAX; width.get()]; height.get()],
        }
    }

    fn coords_exists(&self, coords: UVec2) -> bool {
        (coords.x as usize) < self.get_width() && (coords.y as usize) < self.get_height()
    }

    pub fn get_width(&self) -> usize {
        self.color[0].len()
    }
    pub fn get_height(&self) -> usize {
        self.color.len()
    }

    pub fn get_dimensions(&self) -> UDimensions {
        udimensions(self.get_width(), self.get_height())
    }

    pub fn get_pixel_color(&self, coords: UVec2) -> Option<Rgb> {
        if !self.coords_exists(coords) {
            return None;
        }

        Some(self.color[coords.y as usize][coords.x as usize])
    }

    pub fn get_pixel_depth(&self, coords: UVec2) -> Option<f32> {
        if !self.coords_exists(coords) {
            return None;
        }

        Some(self.depth[coords.y as usize][coords.x as usize])
    }

    pub fn set_pixel_color(&mut self, coords: UVec2, color: Rgb, depth: f32) -> Option<bool> {
        if !self.coords_exists(coords) {
            return None;
        }

        let current_depth = self.get_pixel_depth(coords).unwrap();

        // if current_depth > 0. {
        //     panic!("{} < {}", current_depth, depth);
        // }
        // if depth.is_nan() {
        //     panic!("{} -> {}", current_depth, depth);
        // }

        if current_depth < depth {
            // print!("{} < {}", current_depth, depth);
            // self.color[coords.y as usize][coords.x as usize] = rgb(1., 1., 0.);
            // panic!("{} < {}", current_depth, depth);
            return Some(false);
        }

        self.color[coords.y as usize][coords.x as usize] = color;
        self.depth[coords.y as usize][coords.x as usize] = depth;

        Some(true)
    }

    pub fn clear(&mut self) {
        for rows in self.color.iter_mut() {
            for pixel in rows.iter_mut() {
                *pixel = Rgb::default();
            }
        }
        for rows in self.depth.iter_mut() {
            for pixel in rows.iter_mut() {
                *pixel = f32::MAX;
            }
        }
    }
}

pub trait RenderBufferDrawable {
    fn draw_render_buffer(&mut self, render_buffer: &RenderBuffer);
}

impl RenderBufferDrawable for Pixels {
    fn draw_render_buffer(&mut self, render_buffer: &RenderBuffer) {
        let size = self.context().texture_extent;
        let frame = self.get_frame_mut();

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % size.width as usize) as u32;
            let y = (i / size.width as usize) as u32;

            let color = match render_buffer.get_pixel_color(uvec2(x, y)) {
                Some(color) => color.to_rgba(1.),
                None => rgba(0., 0., 0., 0.),
            }
            .to_byte()
            .to_slice();

            pixel.copy_from_slice(&color);
        }
    }
}

// pub fn draw_render_buffer(pixels: &mut Pixels, buffer: &RenderBuffer) {
//     let size = pixels.context().texture_extent;
//     let frame = pixels.get_frame_mut();

//     for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
//         let x = (i % size.width as usize) as u32;
//         let y = (i / size.width as usize) as u32;

//         let color = match buffer.get_pixel_color(uvec2(x, y)) {
//             Some(color) => color,
//             None => Rgb::default(),
//         }
//         .to_byte()
//         .to_rgba_slice(255);

//         pixel.copy_from_slice(&color);
//     }
// }
