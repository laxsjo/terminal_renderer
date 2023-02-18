use super::*;
// use crate::math::*;
use pixels::Pixels;
use std::{iter::Copied, marker::PhantomData, num::NonZeroUsize};

pub enum PixelValueType {
    Color,
    Depth,
}

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

/* pub trait BufferPixelIter<'a, T>: Iterator<Item = (UVec2, T)> {} */

// impl<'a, T> BufferPixelIter<'a, T> {
//     pub fn new(iter: impl Iterator<Item = (UVec2, T)> + 'a) -> Self {
//         Self {
//             internal_iter: AnyIter::new(iter),
//         }
//     }
// }

// impl<'a, T> Iterator for BufferPixelIter<'a, T> {
//     type Item = (UVec2, T);

//     fn next(&mut self) -> Option<Self::Item> {
//         self.internal_iter.next()
//     }
// }

/* pub trait BufferPixelGridIter<'a, T>: Iterator<Item = Self::RowIter> {
    type RowIter: Iterator<Item = T>;
}
 */
// impl<'a, T> BufferPixelGridIter<'a, T> {
//     pub fn new(iter: impl Iterator<Item = AnyIter<'a, T>> + 'a) -> Self {
//         Self {
//             internal_iter: AnyIter::new(iter),
//         }
//     }
// }

// impl<'a, T> Iterator for BufferPixelGridIter<'a, T> {
//     type Item = AnyIter<'a, T>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.internal_iter.next()
//     }
// }

pub trait RenderBuffer {
    // type PixelIter<'a, T>: BufferPixelIter<'a, T>
    // where
    //     Self: 'a,
    //     T: 'a;
    // type PixelGridIter<'a, T>: BufferPixelGridIter<'a, T>
    // where
    //     Self: 'a,
    //     T: 'a + Copy;
    type PixelIter<'a, T>: Iterator<Item = (UVec2, T)>
    where
        Self: 'a,
        T: 'a;

    type PixelRowIter<'a, T>: Iterator<Item = T>
    where
        Self: 'a,
        T: 'a;

    type PixelGridIter<'a, T>: Iterator<Item = Self::PixelRowIter<'a, T>>
    where
        Self: 'a,
        T: 'a;

    fn colors(&self) -> Self::PixelIter<'_, Rgb>;
    fn depths(&self) -> Self::PixelIter<'_, f32>;

    fn colors_grid(&self) -> Self::PixelGridIter<'_, Rgb>;
    fn depths_grid(&self) -> Self::PixelGridIter<'_, f32>;

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

/*
pub struct SingleRenderBufferIter<'a, T> {
    buffer: &'a Vec<Vec<T>>,
    index: usize,
    back_index: usize,
    len: usize,
    _marker: PhantomData<T>,
}
impl<'a> SingleRenderBufferIter<'a, Rgb> {
    fn new(buffer: SingleRenderBuffer) -> Self {
        let len = buffer.width().get() * buffer.height().get();
        Self {
            buffer: &buffer.color,
            index: 0,
            back_index: len - 1,
            len,
            _marker: PhantomData,
        }
    }
}
impl<'a> SingleRenderBufferIter<'a, f32> {
    fn new(buffer: SingleRenderBuffer) -> Self {
        let len = buffer.width().get() * buffer.height().get();
        Self {
            buffer: &buffer.depth,
            index: 0,
            back_index: len - 1,
            len,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for SingleRenderBufferIter<'a, T> {
    type Item = (UVec2, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.back_index {
            return None;
        }
        let x = self.index % self.buffer[0].len() as usize;
        let y = self.index / self.buffer[0].len() as usize;

        let value = self.buffer[y][x];

        self.index += 1;

        Some((uvec2(x as u32, y as u32), value))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for SingleRenderBufferIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back_index < self.index {
            return None;
        }
        let x = self.back_index % self.buffer[0].len() as usize;
        let y = self.back_index / self.buffer[0].len() as usize;

        let value = self.buffer[y][x];

        self.back_index -= 1;

        Some((uvec2(x as u32, y as u32), value))
    }
}

impl<'a, T> BufferPixelIter<'a, T> for SingleRenderBufferIter<'a, T> {}

pub struct SingleRenderBufferGridIter<'a, T>
where
    T: Copy,
{
    buffer: &'a Vec<Vec<T>>,
    index: usize,
    back_index: usize,
    len: usize,
    _marker: PhantomData<T>,
}
impl<'a> SingleRenderBufferGridIter<'a, Rgb> {
    fn new(buffer: SingleRenderBuffer) -> Self {
        let len = buffer.color.len();

        Self {
            buffer: &buffer.color,
            index: 0,
            back_index: len - 1,
            len,
            _marker: PhantomData,
        }
    }
}
impl<'a> SingleRenderBufferGridIter<'a, f32> {
    fn new(buffer: SingleRenderBuffer) -> Self {
        let len = buffer.depth.len();

        Self {
            buffer: &buffer.depth,
            index: 0,
            back_index: len - 1,
            len,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> BufferPixelGridIter<'a, T> for SingleRenderBufferGridIter<'a, T>
where
    T: Copy,
{
    type RowIter = Copied<std::slice::Iter<'a, T>>;
}

impl<'a, T> Iterator for SingleRenderBufferGridIter<'a, T>
where
    T: Copy,
{
    type Item = <Self as BufferPixelGridIter<'a, T>>::RowIter;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.back_index {
            return None;
        }

        let iter = self.buffer[self.index].iter().copied();

        self.index += 1;

        Some(iter)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for SingleRenderBufferGridIter<'a, T>
where
    T: Copy,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.back_index < self.index {
            return None;
        }

        let iter = self.buffer[self.back_index].iter().copied();

        self.index -= 1;

        Some(iter)
    }
} */

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
    // type PixelIter<'a, T: 'a> = SingleRenderBufferIter<'a, T>;
    // type PixelGridIter<'a, T: 'a> = SingleRenderBufferGridIter<'a, T> where T: Copy;
    type PixelIter<'a, T: 'a> = AnyIter<'a, (UVec2, T)>;
    type PixelRowIter<'a, T: 'a> = AnyIter<'a, T>;
    type PixelGridIter<'a, T: 'a> = AnyIter<'a, Self::PixelRowIter<'a, T>>;

    fn colors(&self) -> Self::PixelIter<'_, Rgb> {
        AnyIter::new(self.color.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, color)| (uvec2(x as u32, y as u32), *color))
        }))
    }

    fn depths(&self) -> Self::PixelIter<'_, f32> {
        AnyIter::new(self.depth.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, depth)| (uvec2(x as u32, y as u32), *depth))
        }))
    }

    fn colors_grid(&self) -> Self::PixelGridIter<'_, Rgb> {
        AnyIter::new(
            self.color
                .iter()
                .map(|row| AnyIter::new(row.iter().cloned())),
        )
    }

    fn depths_grid(&self) -> Self::PixelGridIter<'_, f32> {
        AnyIter::new(
            self.depth
                .iter()
                .map(|row| AnyIter::new(row.iter().cloned())),
        )
    }

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
/*
pub struct SegmentedRenderBufferIter<'a, T> {
    segments: &'a Vec<RenderBufferSegment>,
    index: usize,
    back_index: usize,
    total_index: usize,
    total_back_index: usize,
    current_iter: BufferSegmentIter<'a, T>,
    current_back_iter: BufferSegmentIter<'a, T>,
    total_len: usize,
    _marker: PhantomData<T>,
}
impl<'a> SegmentedRenderBufferIter<'a, Rgb> {
    fn new(buffer: &SegmentedRenderBuffer) -> Self {
        let count = buffer.segments.len();
        let total_len = buffer.width().get() * buffer.height().get();

        let current_iter = BufferSegmentIter::<'a, Rgb>::new(&buffer.segments[0]);
        let current_back_iter = BufferSegmentIter::<'a, Rgb>::new(&buffer.segments[count - 1]);

        Self {
            segments: &buffer.segments,
            index: 0,
            back_index: count - 1,
            total_index: 0,
            total_back_index: total_len - 1,
            current_iter,
            current_back_iter,
            total_len,
            _marker: PhantomData,
        }
    }
}
impl<'a> SegmentedRenderBufferIter<'a, f32> {
    fn new(buffer: &SegmentedRenderBuffer) -> Self {
        let count = buffer.segments.len();
        let total_len = buffer.width().get() * buffer.height().get();

        let current_iter = BufferSegmentIter::<'a, f32>::new(&buffer.segments[0]);
        let current_back_iter = BufferSegmentIter::<'a, f32>::new(&buffer.segments[count - 1]);

        Self {
            segments: &buffer.segments,
            index: 0,
            back_index: count - 1,
            total_index: 0,
            total_back_index: total_len - 1,
            current_iter,
            current_back_iter,
            total_len,
            _marker: PhantomData,
        }
    }
}
// impl<'a> SegmentedRenderBufferIter<'a, f32> {
//     fn new(buffer: SingleRenderBuffer) -> Self {
//         let len = buffer.width() * buffer.height();
//         Self {
//             buffer: &buffer.depth,
//             index: 0,
//             back_index: len - 1,
//             len,
//             _marker: PhantomData,
//         }
//     }
// }

impl<'a, T> Iterator for SegmentedRenderBufferIter<'a, T> {
    type Item = (UVec2, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.total_index > self.total_back_index {
            return None;
        }

        let value = match self.current_iter.next() {
            Some(value) => value,
            None => {
                self.index += 1;
                if self.index >= self.segments.len() {
                    return None;
                }

                self.current_iter = self.segments[self.index];
                if let Some(value) = self.current_iter.next() {
                    value
                } else {
                    return None;
                }
            }
        };

        self.total_index += 1;

        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.total_len, Some(self.total_len))
    }
}

impl<'a, T> DoubleEndedIterator for SegmentedRenderBufferIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.total_index > self.total_back_index {
            return None;
        }

        let value = match self.current_back_iter.next() {
            Some(value) => value,
            None => {
                if self.back_index == 0 {
                    return None;
                }
                self.back_index -= 1;

                self.current_back_iter = self.segments[self.back_index];
                if let Some(value) = self.current_back_iter.next() {
                    value
                } else {
                    return None;
                }
            }
        };

        self.total_back_index.saturating_sub(1);

        Some(value)
    }
}

impl<'a, T> BufferPixelIter<'a, T> for SegmentedRenderBufferIter<'a, T> {}

pub struct SegmentedRenderBufferGridIter<'a, T>
where
    T: Copy,
{
    segments: &'a Vec<RenderBufferSegment>,
    index: usize,
    back_index: usize,
    total_index: usize,
    total_back_index: usize,
    current_iter: SingleRenderBufferGridIter<'a, T>,
    current_back_iter: SingleRenderBufferGridIter<'a, T>,
    total_len: usize,
    _marker: PhantomData<T>,
}
impl<'a, T> SegmentedRenderBufferGridIter<'a, T>
where
    T: Copy,
{
    fn new(buffer: SegmentedRenderBuffer) -> Self {
        let count = buffer.segments.len();
        let total_len = buffer.height().get();

        let current_iter = SingleRenderBufferGridIter::<'a, T>::new(buffer.segments[0]);
        let current_back_iter =
            SingleRenderBufferGridIter::<'a, T>::new(buffer.segments[count - 1]);

        Self {
            segments: &buffer.segments,
            index: 0,
            back_index: count - 1,
            total_index: 0,
            total_back_index: total_len - 1,
            current_iter,
            current_back_iter,
            total_len,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> BufferPixelGridIter<'a, T> for SegmentedRenderBufferGridIter<'a, T>
where
    T: Copy,
{
    type RowIter = std::slice::Iter<'a, T>;
}

impl<'a, T> Iterator for SegmentedRenderBufferGridIter<'a, T>
where
    T: Copy,
{
    type Item = <Self as BufferPixelGridIter<'a, T>>::RowIter;

    // fn next(&mut self) -> Option<Self::Item> {
    //     if self.index > self.back_index {
    //         return None;
    //     }

    //     let iter = self.buffer[self.index].iter();

    //     self.index += 1;

    //     Some(iter)
    // }

    fn next(&mut self) -> Option<Self::Item> {
        if self.total_index > self.total_back_index {
            return None;
        }

        let value = match self.current_iter.next() {
            Some(value) => value,
            None => {
                self.index += 1;
                if self.index >= self.segments {
                    return None;
                }

                self.current_iter = self.segments[self.index];
                if let Some(value) = self.current_iter.next() {
                    value
                } else {
                    return None;
                }
            }
        };

        self.total_index += 1;

        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.total_len, Some(self.total_len))
    }
}

impl<'a, T> DoubleEndedIterator for SegmentedRenderBufferGridIter<'a, T>
where
    T: Copy,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.total_index > self.total_back_index {
            return None;
        }

        let value = match self.current_back_iter.next() {
            Some(value) => value,
            None => {
                if self.back_index == 0 {
                    return None;
                }
                self.back_index -= 1;

                self.current_back_iter = self.segments[self.back_index];
                if let Some(value) = self.current_back_iter.next() {
                    value
                } else {
                    return None;
                }
            }
        };

        self.total_back_index.saturating_sub(1);

        Some(value)
    }
}
 */
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
    // type PixelIter<'a, T: 'a> = SegmentedRenderBufferIter<'a, T>;
    // type PixelGridIter<'a, T: 'a> = SegmentedRenderBufferGridIter<'a, T>where
    // T: Copy,;
    type PixelIter<'a, T: 'a> = AnyIter<'a, (UVec2, T)>;
    type PixelRowIter<'a, T: 'a> = AnyIter<'a, T>;
    type PixelGridIter<'a, T: 'a> = AnyIter<'a, Self::PixelRowIter<'a, T>>;

    fn colors(&self) -> Self::PixelIter<'_, Rgb> {
        Self::PixelIter::new(self)
    }
    fn depths(&self) -> Self::PixelIter<'_, f32> {
        Self::PixelIter::new(self)
    }

    fn colors_grid(&self) -> Self::PixelGridIter<'_, Rgb> {
        Self::PixelGridIter::new(self)
    }
    fn depths_grid(&self) -> Self::PixelGridIter<'_, f32> {
        Self::PixelGridIter::new(self)
    }

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

/* pub struct BufferSegmentIter<'a, T> {
    iter: SingleRenderBufferIter<'a, T>,
    y_offset: u32,
}

impl<'a> BufferSegmentIter<'a, Rgb> {
    pub fn new(buffer: &RenderBufferSegment) -> Self {
        Self {
            iter: SingleRenderBufferIter::new(buffer),
            y_offset: buffer.vertical_index * buffer.buffer.height(),
        }
    }
}
impl<'a> BufferSegmentIter<'a, f32> {
    pub fn new(buffer: &RenderBufferSegment) -> Self {
        Self {
            iter: SingleRenderBufferIter::new(buffer),
            y_offset: buffer.vertical_index * buffer.buffer.height(),
        }
    }
}

impl<'a, T> Iterator for BufferSegmentIter<'a, T> {
    type Item = (UVec2, T);

    fn next(&mut self) -> Option<Self::Item> {
        let mut value = self.iter.next();

        value.0.y += self.y_offset;

        return value;
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for BufferSegmentIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut value = self.iter.next_back();
        value.0.y += self.y_offset;

        return value;
    }
}
 */

/// Represents a vertical segment of a larger RenderBuffer.
///
/// This is used when dividing up rendering to multiple threads.
pub struct RenderBufferSegment {
    pub buffer: SingleRenderBuffer,
    pub vertical_index: usize,
    pub pos_y: usize,
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
                pos_y: 0,
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
                pos_y: segment_max_height * y,
                parent_size: buffer_size,
            })
        }

        buffers
    }

    pub fn coords_within_bounds(&self, coords: UVec2) -> bool {
        let height = self.buffer.height().get();
        (coords.x as usize) < self.buffer.width().get()
            && (coords.y as usize) >= self.pos_y
            && (coords.y as usize) < (self.pos_y - height)
    }

    pub fn coords_to_segment_space(&self, mut coords: UVec2) -> UVec2 {
        coords.y %= self.buffer.height().get() as u32;

        coords
    }
}

impl RenderBuffer for RenderBufferSegment {
    // type PixelIter<'a, T: 'a> = SingleRenderBufferIter<'a, T>;
    // type PixelGridIter<'a, T: 'a> = SingleRenderBufferGridIter<'a, T>where
    // T: Copy,;
    type PixelIter<'a, T: 'a> = AnyIter<'a, (UVec2, T)>;
    type PixelRowIter<'a, T: 'a> = AnyIter<'a, T>;
    type PixelGridIter<'a, T: 'a> = AnyIter<'a, Self::PixelRowIter<'a, T>>;

    fn colors(&self) -> Self::PixelIter<'_, Rgb> {
        AnyIter::new(
            self.buffer
                .colors()
                .map(|value| (value.0 + uvec2(0, self.vertical_index),)),
        )
    }
    fn depths(&self) -> Self::PixelIter<'_, f32> {
        self.buffer.depths()
    }

    fn colors_grid(&self) -> Self::PixelGridIter<'_, Rgb> {
        self.buffer.colors_grid()
    }
    fn depths_grid(&self) -> Self::PixelGridIter<'_, f32> {
        self.buffer.depths_grid()
    }

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
