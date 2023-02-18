use crate::utils::DoubleEndedAnyIter;

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

    pub fn includes_point(&self, pos: UVec2) -> bool {
        ((self.pos.x)..(self.pos.x + self.size.x as u32)).contains(&pos.x)
            && ((self.pos.y)..(self.pos.y + self.size.y as u32)).contains(&pos.y)
    }
}

pub struct RenderBuffer {
    colors: Vec<Rgb>,
    depths: Vec<f32>,
    size: NonZeroUDimensions,
}

impl RenderBuffer {
    pub fn new(size: NonZeroUDimensions) -> Self {
        let len = size.x.get() * size.y.get();
        Self {
            colors: vec![Rgb::default(); len],
            depths: vec![f32::MAX; len],
            size,
        }
    }

    pub fn coords_exists(&self, coords: UVec2) -> bool {
        (coords.x as usize) < self.size.x.get() && (coords.y as usize) < self.size.y.get()
    }

    fn coords_index(&self, coords: UVec2) -> usize {
        (coords.y * coords.x + coords.x) as usize
    }
    fn index_coords(&self, index: usize) -> UVec2 {
        let x = (index % self.size.x.get()) as u32;
        let y = (index / self.size.x.get()) as u32;
        uvec2(x, y)
    }

    pub fn size(&self) -> NonZeroUDimensions {
        self.size
    }

    pub fn width(&self) -> NonZeroUsize {
        self.size.x
    }
    pub fn height(&self) -> NonZeroUsize {
        self.size.y
    }

    // fn colors_grid(&self) -> Self::PixelGridIter<'_, Rgb> {
    //     AnyIter::new(
    //         self.color
    //             .iter()
    //             .map(|row| AnyIter::new(row.iter().cloned())),
    //     )
    // }

    // fn depths_grid(&self) -> Self::PixelGridIter<'_, f32> {
    //     AnyIter::new(
    //         self.depth
    //             .iter()
    //             .map(|row| AnyIter::new(row.iter().cloned())),
    //     )
    // }

    pub fn pixel_color(&self, coords: UVec2) -> Option<Rgb> {
        if !self.coords_exists(coords) {
            return None;
        }

        Some(self.colors[self.coords_index(coords)])
    }

    pub fn pixel_depth(&self, coords: UVec2) -> Option<f32> {
        if !self.coords_exists(coords) {
            return None;
        }

        Some(self.depths[self.coords_index(coords)])
    }

    pub fn set_pixel_color(&mut self, coords: UVec2, color: Rgb, depth: f32) -> Option<bool> {
        let current_depth = self.pixel_depth(coords)?;

        if current_depth < depth {
            return Some(false);
        }

        self.overwrite_pixel_color(coords, color, depth)?;

        Some(true)
    }

    pub fn overwrite_pixel_color(&mut self, coords: UVec2, color: Rgb, depth: f32) -> Option<()> {
        if !self.coords_exists(coords) {
            return None;
        }

        let index = self.coords_index(coords);

        self.colors[index] = color;
        self.depths[index] = depth;

        Some(())
    }

    pub fn clear(&mut self) {
        for pixel in self.colors.iter_mut() {
            *pixel = Rgb::default();
        }
        for pixel in self.depths.iter_mut() {
            *pixel = f32::MAX;
        }
    }

    pub fn copy_region_from_other(
        &mut self,
        from_buffer: &RenderBuffer,
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

    pub fn colors(&self) -> AnyIter<'_, (UVec2, Rgb)> {
        AnyIter::new(
            self.colors
                .iter()
                .enumerate()
                .map(|(index, color)| (self.index_coords(index), *color)),
        )
    }
    pub fn depths(&self) -> AnyIter<'_, (UVec2, f32)> {
        AnyIter::new(
            self.depths
                .iter()
                .enumerate()
                .map(|(index, depth)| (self.index_coords(index), *depth)),
        )
    }
    pub fn color_rows(&self) -> DoubleEndedAnyIter<'_, std::slice::Iter<Rgb>> {
        DoubleEndedAnyIter::new((0..(self.size.y.get())).map(|i| {
            let start = i * self.size.x.get();
            let end = start + self.size.x.get();

            self.colors[start..end].iter()
        }))
    }
    pub fn depth_rows(&self) -> DoubleEndedAnyIter<'_, std::slice::Iter<f32>> {
        DoubleEndedAnyIter::new((0..(self.size.y.get())).map(|i| {
            let start = i * self.size.x.get();
            let end = start + self.size.x.get();

            self.depths[start..end].iter()
        }))
    }

    pub fn divide_segments(
        &mut self,
        segment_heights: NonZeroUsize,
    ) -> Vec<RenderBufferSegment<'_>> {
        let mut segments = Vec::new();

        let mut colors = self.colors.as_mut_slice();
        let mut depths = self.depths.as_mut_slice();

        let segment_heights = segment_heights.get();
        let height = self.size.y.get();
        let width = self.size.x.get();

        let mut i = 0_usize;
        loop {
            let own_height = if (i + 1) * segment_heights > height {
                segment_heights
            } else {
                ((i + 1) * segment_heights) % height
            };

            let slice_len = own_height * width;

            let (colors_slice, new_colors) = colors.split_at_mut(slice_len);
            colors = new_colors;
            let (depths_slice, new_depths) = depths.split_at_mut(slice_len);
            depths = new_depths;

            let segment = RenderBufferSegment {
                colors: colors_slice,
                depths: depths_slice,
                vertical_index: i,
                own_height,
                segment_heights,
                parent_size: self.size,
            };

            segments.push(segment);

            if i * segment_heights >= height {
                break;
            }
            i += 1;
        }

        // colors.split_at_mut(mid)

        segments
    }
}

/// Represents a vertical segment of a larger RenderBuffer.
///
/// This is used when dividing up rendering to multiple threads.
pub struct RenderBufferSegment<'a> {
    colors: &'a mut [Rgb],
    depths: &'a mut [f32],
    vertical_index: usize,
    own_height: usize,
    segment_heights: usize,
    parent_size: NonZeroUDimensions,
}

impl<'a> RenderBufferSegment<'a> {
    pub fn position(&self) -> UVec2 {
        let y = self.vertical_index * self.segment_heights;
        uvec2(0, y as u32)
    }

    pub fn size(&self) -> UDimensions {
        udimensions(self.parent_size.x.get(), self.own_height)
    }

    pub fn parent_size(&self) -> UDimensions {
        self.parent_size.get()
    }

    pub fn region(&self) -> Region {
        Region {
            pos: self.position(),
            size: self.size(),
        }
    }

    pub fn coords_inside(&self, coords: UVec2) -> bool {
        self.region().includes_point(coords)
    }

    fn coords_index(&self, coords: UVec2) -> Option<usize> {
        if !self.coords_inside(coords) {
            return None;
        }
        let pos = self.position();

        Some(((coords.y - pos.y) * coords.x + coords.x) as usize)
    }

    pub fn pixel_color(&self, coords: UVec2) -> Option<Rgb> {
        let Some(index) = self.coords_index(coords) else {
            return None;
        };

        Some(self.colors[index])
    }

    pub fn pixel_depth(&self, coords: UVec2) -> Option<f32> {
        let Some(index) = self.coords_index(coords) else {
            return None;
        };

        Some(self.depths[index])
    }

    pub fn overwrite_pixel(&mut self, coords: UVec2, color: Rgb, depth: f32) -> Option<()> {
        let Some(index) = self.coords_index(coords) else {
            return None;
        };

        self.colors[index] = color;
        self.depths[index] = depth;

        Some(())
    }

    pub fn set_pixel(&mut self, coords: UVec2, color: Rgb, depth: f32) -> Option<bool> {
        let current_depth = self.pixel_depth(coords)?;

        if current_depth < depth {
            return Some(false);
        }

        self.overwrite_pixel(coords, color, depth)?;

        Some(true)
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
