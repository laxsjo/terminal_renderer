use crate::math;
use crate::utils::DoubleEndedAnyIter;

use super::*;
// use crate::math::*;
use pixels::Pixels;
use std::marker::PhantomData;
use std::num::NonZeroUsize;
use std::thread;

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
        self.pos.x <= pos.x
            && pos.x < self.pos.x + self.size.x as u32
            && self.pos.y <= pos.y
            && pos.y < self.pos.y + self.size.y as u32
    }

    pub fn line_intersects(&self, line: ULine) -> bool {
        let corners = (self.pos, self.pos + self.size.as_vec());
        !matches!(
            math::frame_intersection(line.into(), (corners.0.into(), corners.1.into())),
            FrameIntersection::None
        )
    }

    pub fn triangle_outside(&self, triangle: (UVec2, UVec2, UVec2)) -> bool {
        if self.includes_point(triangle.0)
            || self.includes_point(triangle.1)
            || self.includes_point(triangle.2)
        {
            return false;
        }

        if self.line_intersects(ULine(triangle.0, triangle.1))
            || self.line_intersects(ULine(triangle.1, triangle.2))
            || self.line_intersects(ULine(triangle.0, triangle.2))
        {
            return false;
        }

        true
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

    pub fn normalized_to_buffer_space(&self, point: Vec2) -> UVec2 {
        let UDimensions { x, y } = self.size().get();
        Vec2 {
            x: (point.x * x as f32 + point.x) / 2.,
            y: (point.y * y as f32 + point.y) / 2.,
        }
        .into()
    }

    pub fn coords_exists(&self, coords: UVec2) -> bool {
        (coords.x as usize) < self.size.x.get() && (coords.y as usize) < self.size.y.get()
    }

    fn coords_index(&self, coords: UVec2) -> usize {
        (coords.y * self.size.x.get() as u32 + coords.x) as usize
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

    pub fn aspect_ratio(&self) -> f32 {
        let size = self.size().get();
        aspect_ratio(vec2(size.x as f32, size.y as f32))
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

    pub fn separate_into_segments(
        &self,
        segment_heights: NonZeroUsize,
    ) -> Vec<RenderBufferSegment<'_, &'_ [Rgb], &'_ [f32]>> {
        let mut segments = Vec::new();

        let mut colors = self.colors.as_slice();
        let mut depths = self.depths.as_slice();

        let segment_heights = segment_heights.get();
        let height = self.size.y.get();
        let width = self.size.x.get();
        let rest_height = height % segment_heights;

        let mut count = height / segment_heights;
        if rest_height != 0 {
            count += 1;
        }

        for i in 0..count {
            let own_height = if (i + 1) * segment_heights <= height {
                segment_heights
            } else {
                rest_height
            };

            let slice_len = own_height * width;

            let (colors_slice, new_colors) = colors.split_at(slice_len);
            colors = new_colors;
            let (depths_slice, new_depths) = depths.split_at(slice_len);
            depths = new_depths;

            let segment = RenderBufferSegment {
                colors: colors_slice,
                depths: depths_slice,
                vertical_index: i,
                own_height,
                segment_heights,
                parent_size: self.size,
                _lifetime: PhantomData,
            };

            segments.push(segment);
        }

        segments
    }

    pub fn separate_into_segments_mut(
        &mut self,
        segment_heights: NonZeroUsize,
    ) -> Vec<RenderBufferSegment<'_, &'_ mut [Rgb], &'_ mut [f32]>> {
        let mut segments = Vec::new();

        let mut colors = self.colors.as_mut_slice();
        let mut depths = self.depths.as_mut_slice();

        let segment_heights = segment_heights.get();
        let height = self.size.y.get();
        let width = self.size.x.get();
        let rest_height = height % segment_heights;

        let mut count = height / segment_heights;
        if rest_height != 0 {
            count += 1;
        }

        for i in 0..count {
            let own_height = if (i + 1) * segment_heights <= height {
                segment_heights
            } else {
                rest_height
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
                _lifetime: PhantomData,
            };

            segments.push(segment);
        }

        segments
    }

    pub fn as_single_segment(&mut self) -> RenderBufferSegment<'_, &'_ [Rgb], &'_ [f32]> {
        let colors = self.colors.as_slice();
        let depths = self.depths.as_slice();
        let height = self.size.y.get();

        RenderBufferSegment {
            colors,
            depths,
            vertical_index: 0,
            own_height: height,
            segment_heights: height,
            parent_size: self.size,
            _lifetime: PhantomData,
        }
    }
    pub fn as_single_segment_mut(
        &mut self,
    ) -> RenderBufferSegment<'_, &'_ mut [Rgb], &'_ mut [f32]> {
        let colors = self.colors.as_mut_slice();
        let depths = self.depths.as_mut_slice();
        let height = self.size.y.get();

        RenderBufferSegment {
            colors,
            depths,
            vertical_index: 0,
            own_height: height,
            segment_heights: height,
            parent_size: self.size,
            _lifetime: PhantomData,
        }
    }
}

pub type RenderBufferSegmentRead<'a> = RenderBufferSegment<'a, &'a [Rgb], &'a [f32]>;
pub type RenderBufferSegmentMut<'a> = RenderBufferSegment<'a, &'a mut [Rgb], &'a mut [f32]>;

/// Represents a vertical segment of a larger RenderBuffer that can be edited.
///
/// This is used when dividing up rendering to multiple threads.
pub struct RenderBufferSegment<'a, C, D>
where
    C: 'a,
    D: 'a,
{
    colors: C,
    depths: D,
    vertical_index: usize,
    own_height: usize,
    segment_heights: usize,
    parent_size: NonZeroUDimensions,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a, C, D> RenderBufferSegment<'a, C, D>
where
    C: 'a,
    D: 'a,
{
    pub fn normalized_to_buffer_space(&self, point: Vec2) -> UVec2 {
        let UDimensions {
            x: width,
            y: height,
        } = self.parent_size();
        Vec2 {
            x: (width as f32 * (point.x + 1.)) / 2.,
            y: (height as f32 * (point.y + 1.)) / 2.,
        }
        .into()
    }

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

    pub fn parent_aspect_ratio(&self) -> f32 {
        let size = self.parent_size();
        aspect_ratio(vec2(size.x as f32, size.y as f32))
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

        Some(((coords.y - pos.y) * self.parent_size.x.get() as u32 + coords.x) as usize)
    }
}

impl<'a> RenderBufferSegmentRead<'a> {
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

    pub fn colors(&self) -> &[Rgb] {
        self.colors
    }

    pub fn depths(&self) -> &[f32] {
        self.depths
    }
}

impl<'a> RenderBufferSegmentMut<'a> {
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

    pub fn colors(&self) -> &[Rgb] {
        self.colors
    }

    pub fn depths(&self) -> &[f32] {
        self.depths
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

    pub fn clear(&mut self) {
        for pixel in self.colors.iter_mut() {
            *pixel = Rgb::default();
        }
        for pixel in self.depths.iter_mut() {
            *pixel = f32::MAX;
        }
    }
}

pub trait RenderBufferDrawable {
    fn draw_render_buffer(&mut self, render_buffer: &RenderBuffer, segment_heights: NonZeroUsize);
}

impl RenderBufferDrawable for Pixels {
    fn draw_render_buffer(&mut self, render_buffer: &RenderBuffer, segment_heights: NonZeroUsize) {
        let frame_size = self.context().texture_extent;
        let mut frame = self.get_frame_mut();

        let segments = render_buffer.separate_into_segments(segment_heights);

        // let frame_segments_pixel_count = segments[0].colors.len() * 4;
        // let frame_segments_height = frame_segments_pixel_count as u32 / (frame_size.width * 4);
        let segments_height = segments[0].own_height;

        thread::scope(|scope| {
            let mut handles = Vec::new();
            for (i, segment) in segments.into_iter().enumerate() {
                let this_frame_segment_pixel_count = segment.own_height * frame_size.width as usize;
                let (segment_frame, new_frame) =
                    frame.split_at_mut(this_frame_segment_pixel_count * 4);
                frame = new_frame;

                handles.push(scope.spawn(move || {
                    for (j, pixel) in segment_frame.chunks_exact_mut(4).enumerate() {
                        let x = (j % frame_size.width as usize) as u32;
                        let y = ((j / frame_size.width as usize) + (segments_height * i)) as u32;

                        let color = match segment.pixel_color(uvec2(x, y)) {
                            Some(color) => color.to_rgba(1.),
                            None => rgba(0., 0., 0., 0.),
                        }
                        .to_byte()
                        .to_slice();

                        pixel.copy_from_slice(&color);
                    }
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }
        })
    }
}

mod tests {

    #[test]
    fn render_buffer_indices() {
        use crate::math::*;
        use crate::render_3d::{non_zero_udimensions, RenderBuffer};

        let buffer = RenderBuffer::new(non_zero_udimensions(10, 5).unwrap());

        let coords = uvec2(6, 2);

        assert_eq!(buffer.coords_index(coords), 26);
    }

    #[test]
    fn region_includes_point() {
        use super::*;
        // use crate::math::*;
        // use crate::render_3d::{non_zero_udimensions, RenderBuffer};

        let region = Region {
            pos: uvec2(3, 2),
            size: udimensions(6, 5),
        };

        assert!(!region.includes_point(uvec2(1, 1)));
        assert!(region.includes_point(uvec2(5, 3)));
        assert!(!region.includes_point(uvec2(11, 3)));
        assert!(!region.includes_point(uvec2(5, 10)));
    }
}
