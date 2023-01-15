use crate::{math::uvec2, render_3d::*};
use pixels::Pixels;

pub fn draw_render_buffer(pixels: &mut Pixels, buffer: &RenderBuffer) {
    let size = pixels.context().texture_extent;
    let frame = pixels.get_frame_mut();

    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % size.width as usize) as u32;
        let y = (i / size.width as usize) as u32;

        let color = match buffer.get_pixel_color(uvec2(x, y)) {
            Some(color) => color,
            None => Rgb::default(),
        }
        .to_byte_rgb()
        .to_rgba_slice(255);

        pixel.copy_from_slice(&color);
    }
}
