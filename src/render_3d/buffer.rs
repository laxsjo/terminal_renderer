use super::*;
// use crate::math::*;
use pixels::Pixels;
use std::num::NonZeroUsize;

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Region {
    pos: UVec2,
    size: UDimensions,
}

impl Region {
    pub fn new(pos: UVec2, size: UDimensions) -> Self {
        Self { pos, size }
    }
}

pub trait WholeRenderBuffer: RenderBuffer {
    fn coords_exists(&self, coords: UVec2) -> bool {
        (coords.x as usize) < self.width().get() && (coords.y as usize) < self.height().get()
    }

    fn width(&self) -> NonZeroUsize;
    fn height(&self) -> NonZeroUsize;

    fn size(&self) -> NonZeroUDimensions {
        NonZeroUDimensions::new(self.width(), self.height())
    }
}

pub trait RenderBuffer {
    fn pixel_color(&self, coords: UVec2) -> Option<Rgb>;

    fn pixel_depth(&self, coords: UVec2) -> Option<f32>;

    fn set_pixel_color(&mut self, coords: UVec2, color: Rgb, depth: f32) -> Option<bool>;

    fn overwrite_pixel_color(&mut self, coords: UVec2, color: Rgb, depth: f32) -> Option<()>;

    fn clear(&mut self);

    fn copy_region_from_other(
        &mut self,
        from_buffer: &(impl RenderBuffer + WholeRenderBuffer),
        from_region: Region,
        to_pos: UVec2,
    );
}

pub struct SingleRenderBuffer {
    pub color: Vec<Vec<Rgb>>,
    pub depth: Vec<Vec<f32>>,
}

impl SingleRenderBuffer {
    pub fn new(size: NonZeroUDimensions) -> Self {
        Self {
            color: vec![vec![Rgb::default(); size.x.get()]; size.y.get()],
            depth: vec![vec![f32::MAX; size.x.get()]; size.y.get()],
        }
    }
}

impl WholeRenderBuffer for SingleRenderBuffer {
    fn width(&self) -> NonZeroUsize {
        // println!(
        //     "Got width {}, depth {}",
        //     self.color[0].len(),
        //     self.depth[0].len()
        // );
        NonZeroUsize::new(self.color[0].len()).expect("color array has zero length")
    }
    fn height(&self) -> NonZeroUsize {
        NonZeroUsize::new(self.color.len()).expect("color array has zero length")
    }
}

impl RenderBuffer for SingleRenderBuffer {
    fn pixel_color(&self, coords: UVec2) -> Option<Rgb> {
        if !self.coords_exists(coords) {
            return None;
        }

        Some(self.color[coords.y as usize][coords.x as usize])
    }

    fn pixel_depth(&self, coords: UVec2) -> Option<f32> {
        if !self.coords_exists(coords) {
            return None;
        }

        Some(self.depth[coords.y as usize][coords.x as usize])
    }

    fn set_pixel_color(&mut self, coords: UVec2, color: Rgb, depth: f32) -> Option<bool> {
        if !self.coords_exists(coords) {
            return None;
        }

        let current_depth = self.pixel_depth(coords).unwrap();

        if current_depth < depth {
            return Some(false);
        }

        self.color[coords.y as usize][coords.x as usize] = color;
        self.depth[coords.y as usize][coords.x as usize] = depth;

        Some(true)
    }

    fn overwrite_pixel_color(&mut self, coords: UVec2, color: Rgb, depth: f32) -> Option<()> {
        if !self.coords_exists(coords) {
            return None;
        }

        self.color[coords.y as usize][coords.x as usize] = color;
        self.depth[coords.y as usize][coords.x as usize] = depth;

        Some(())
    }

    fn clear(&mut self) {
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

    fn copy_region_from_other(
        &mut self,
        from_buffer: &(impl RenderBuffer + WholeRenderBuffer),
        from_region: Region,
        to_pos: UVec2,
    ) {
        for y in 0..from_region.size.y {
            for x in 0..from_region.size.x {
                let offset = uvec2(x as u32, y as u32);
                let Some(color) = from_buffer.pixel_color(from_region.pos + offset) else {
                    continue;
                };
                let Some(depth) = from_buffer.pixel_depth(from_region.pos + offset) else {
                    continue;
                };
                self.overwrite_pixel_color(to_pos + offset, color, depth);
            }
        }
    }
}

pub struct SegmentedRenderBuffer {
    segments: Vec<RenderBufferSegment>,
    segment_height: NonZeroUsize,
    height: NonZeroUsize,
}

impl SegmentedRenderBuffer {
    pub fn new(size: NonZeroUDimensions, segment_height: NonZeroUsize) -> Self {
        let segments = RenderBufferSegment::create_segments(size, segment_height);

        Self {
            segments,
            segment_height,
            height: size.y,
        }
    }

    pub fn segment_height(&self) -> NonZeroUsize {
        self.segment_height
    }

    fn coords_segment_index(&self, coords: UVec2) -> usize {
        coords.y as usize / self.segment_height.get()
    }

    pub fn coords_segment(&self, coords: UVec2) -> Option<&RenderBufferSegment> {
        if !self.coords_exists(coords) {
            return None;
        }

        Some(&self.segments[self.coords_segment_index(coords)])
    }

    pub fn coords_segment_mut(&mut self, coords: UVec2) -> Option<&mut RenderBufferSegment> {
        if !self.coords_exists(coords) {
            return None;
        }

        let index = self.coords_segment_index(coords);

        Some(&mut self.segments[index])
    }

    pub fn coords_to_segment_space(&self, mut coords: UVec2) -> UVec2 {
        coords.y %= self.segment_height.get() as u32;

        coords
    }

    pub fn iter(&self) -> std::slice::Iter<RenderBufferSegment> {
        self.segments.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<RenderBufferSegment> {
        self.segments.iter_mut()
    }
}

impl WholeRenderBuffer for SegmentedRenderBuffer {
    fn width(&self) -> NonZeroUsize {
        NonZeroUsize::new(self.segments[0].buffer.color[0].len())
            .expect("color array has zero length")
    }
    fn height(&self) -> NonZeroUsize {
        self.height
    }
}

impl RenderBuffer for SegmentedRenderBuffer {
    fn pixel_color(&self, coords: UVec2) -> Option<Rgb> {
        if !self.coords_exists(coords) {
            return None;
        }

        let local_coords = self.coords_to_segment_space(coords);

        Some(
            self.coords_segment(coords)?.buffer.color[local_coords.y as usize]
                [local_coords.x as usize],
        )
    }

    fn pixel_depth(&self, coords: UVec2) -> Option<f32> {
        if !self.coords_exists(coords) {
            return None;
        }

        let local_coords = self.coords_to_segment_space(coords);

        Some(
            self.coords_segment(coords)?.buffer.depth[local_coords.y as usize]
                [local_coords.x as usize],
        )
    }

    fn set_pixel_color(&mut self, coords: UVec2, color: Rgb, depth: f32) -> Option<bool> {
        if !self.coords_exists(coords) {
            return None;
        }

        let current_depth = self.pixel_depth(coords).unwrap();

        if current_depth < depth {
            return Some(false);
        }

        self.overwrite_pixel_color(coords, color, depth)?;

        Some(true)
    }

    fn overwrite_pixel_color(&mut self, coords: UVec2, color: Rgb, depth: f32) -> Option<()> {
        if !self.coords_exists(coords) {
            return None;
        }

        let local_coords = self.coords_to_segment_space(coords);
        self.coords_segment_mut(coords)?.buffer.color[local_coords.y as usize]
            [local_coords.x as usize] = color;
        self.coords_segment_mut(coords)?.buffer.depth[local_coords.y as usize]
            [local_coords.x as usize] = depth;

        Some(())
    }

    fn clear(&mut self) {
        for segment in self.segments.iter_mut() {
            for row in segment.buffer.color.iter_mut() {
                for pixel in row.iter_mut() {
                    *pixel = Rgb::default();
                }
            }
            for row in segment.buffer.depth.iter_mut() {
                for pixel in row.iter_mut() {
                    *pixel = f32::MAX;
                }
            }
        }
    }

    fn copy_region_from_other(
        &mut self,
        from_buffer: &(impl RenderBuffer + WholeRenderBuffer),
        from_region: Region,
        to_pos: UVec2,
    ) {
        for y in 0..from_region.size.y {
            for x in 0..from_region.size.x {
                let offset = uvec2(x as u32, y as u32);
                let Some(color) = from_buffer.pixel_color(from_region.pos + offset) else {
                    continue;
                };
                let Some(depth) = from_buffer.pixel_depth(from_region.pos + offset) else {
                    continue;
                };
                self.overwrite_pixel_color(to_pos + offset, color, depth);
            }
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

/// Represents a vertical segment of a larger RenderBuffer.
///
/// This is used when dividing up rendering to multiple threads.
pub struct RenderBufferSegment {
    pub buffer: SingleRenderBuffer,
    pub vertical_index: usize,
    pub parent_size: NonZeroUDimensions,
}

impl RenderBufferSegment {
    pub fn create_segments(
        buffer_size: NonZeroUDimensions,
        segment_max_height: NonZeroUsize,
    ) -> Vec<Self> {
        let mut buffers = Vec::new();

        if buffer_size.y.get() <= segment_max_height.get() {
            buffers.push(RenderBufferSegment {
                buffer: SingleRenderBuffer::new(buffer_size),
                vertical_index: 0,
                parent_size: buffer_size,
            });

            return buffers;
        }

        let mut count_y = buffer_size.y.get() / segment_max_height.get();
        if buffer_size.y.get() % segment_max_height.get() != 0 {
            count_y += 1;
        }

        for y in 0..count_y {
            let mut frag_size = NonZeroUDimensions::new(buffer_size.x, segment_max_height);
            if (y + 1) * segment_max_height.get() > buffer_size.y.get() {
                frag_size.y =
                    NonZeroUsize::new(buffer_size.y.get() - (y + 1) * segment_max_height.get())
                        .expect("got zero height render buffer segment");
            }

            let frag_buffer = SingleRenderBuffer::new(frag_size);

            buffers.push(RenderBufferSegment {
                buffer: frag_buffer,
                vertical_index: y,
                parent_size: buffer_size,
            })
        }

        buffers
    }

    pub fn coords_within_bounds(&self, coords: UVec2) -> bool {
        let height = self.buffer.height().get();
        (coords.x as usize) < self.buffer.width().get()
            && (coords.y as usize) >= height * self.vertical_index
            && (coords.y as usize) < height * (self.vertical_index + 1)
    }

    pub fn coords_to_segment_space(&self, mut coords: UVec2) -> UVec2 {
        coords.y %= self.buffer.height().get() as u32;

        coords
    }
}

impl RenderBuffer for RenderBufferSegment {
    fn pixel_color(&self, coords: UVec2) -> Option<Rgb> {
        self.buffer
            .pixel_color(self.coords_to_segment_space(coords))
    }

    fn pixel_depth(&self, coords: UVec2) -> Option<f32> {
        self.buffer
            .pixel_depth(self.coords_to_segment_space(coords))
    }

    fn set_pixel_color(&mut self, coords: UVec2, color: Rgb, depth: f32) -> Option<bool> {
        self.buffer
            .set_pixel_color(self.coords_to_segment_space(coords), color, depth)
    }

    fn overwrite_pixel_color(&mut self, coords: UVec2, color: Rgb, depth: f32) -> Option<()> {
        self.buffer
            .overwrite_pixel_color(self.coords_to_segment_space(coords), color, depth)
    }

    fn clear(&mut self) {
        self.buffer.clear();
    }

    fn copy_region_from_other(
        &mut self,
        from_buffer: &(impl RenderBuffer + WholeRenderBuffer),
        from_region: Region,
        to_pos: UVec2,
    ) {
        self.buffer.copy_region_from_other(
            from_buffer,
            from_region,
            self.coords_to_segment_space(to_pos),
        );
    }
}

pub trait RenderBufferDrawable {
    fn draw_render_buffer(&mut self, render_buffer: &impl RenderBuffer);
}

impl RenderBufferDrawable for Pixels {
    fn draw_render_buffer(&mut self, render_buffer: &impl RenderBuffer) {
        let size = self.context().texture_extent;
        let frame = self.get_frame_mut();

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % size.width as usize) as u32;
            let y = (i / size.width as usize) as u32;

            let color = match render_buffer.pixel_color(uvec2(x, y)) {
                Some(color) => color.to_rgba(1.),
                None => rgba(0., 0., 0., 0.),
            }
            .to_byte()
            .to_slice();

            pixel.copy_from_slice(&color);
        }
    }
}
