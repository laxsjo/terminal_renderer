use super::*;
use shader::*;

pub trait Drawer {
    type Params;
    type Out;

    fn draw<F>(&self, params: Self::Params, plotter: F)
    where
        F: FnMut(UVec2, Self::Out);
}

pub struct LineDrawParams(pub Vec2, pub Vec2, pub Rgb);
pub struct AaLineDrawer {
    buffer_dimensions: UDimensions,
}

impl AaLineDrawer {
    pub fn new(dimensions: UDimensions) -> Self {
        Self {
            buffer_dimensions: dimensions,
        }
    }
}

impl Drawer for AaLineDrawer {
    type Params = LineDrawParams;
    type Out = Rgb;

    fn draw<F>(&self, params: Self::Params, mut plotter: F)
    where
        F: FnMut(UVec2, Rgb),
    {
        // Source: https://en.wikipedia.org/wiki/Xiaolin_Wu's_line_algorithm

        let mut a = normalized_to_buffer_space(params.0, self.buffer_dimensions).round();
        let mut b = normalized_to_buffer_space(params.1, self.buffer_dimensions).round();

        // fn float_color_to_integer(color: f32) -> u8 {
        //     (color * 255.) as u8
        // }

        let mut plot = |point: UVec2, color: f32| {
            plotter(point, color * params.2);
        };

        let is_steep = (b.y - a.y).abs() > (b.x - a.x).abs();

        if is_steep {
            a = Vec2::new(a.y, a.x);
            b = Vec2::new(b.y, b.x);
        }

        if a.x > b.x {
            std::mem::swap(&mut a, &mut b);
        }

        let delta = b - a;

        let gradient = if delta.x == 0. { 1. } else { delta.y / delta.x };

        // handle first endpoint
        let end = vec2(a.x.round(), a.y + gradient * (a.x.round() - a.x));
        let gap_x = 1. - (a.x + 0.5).fract();
        let pxl_1: UVec2 = end.into();

        let offset = if is_steep { uvec2(1, 0) } else { uvec2(0, 1) };
        let coords = if is_steep { pxl_1.swap() } else { pxl_1 };

        plot(coords, ((1. - end.y.fract()) * gap_x) as f32);
        plot(coords + offset, (end.y.fract() * gap_x) as f32);

        let mut inter_y = end.y + gradient; // first y-intersection for the main loop

        // handle second endpoint
        let end = vec2(b.x.round(), b.y + gradient * (b.x.round() - b.x));
        let gap_x = (b.x + 0.5).fract();
        let pxl_2: UVec2 = end.into();

        let offset = if is_steep { uvec2(1, 0) } else { uvec2(0, 1) };
        let coords = if is_steep { pxl_2.swap() } else { pxl_2 };

        plot(coords, ((1. - end.y.fract()) * gap_x) as f32);
        plot(coords + offset, (end.y.fract() * gap_x) as f32);

        // main loop
        for x in pxl_1.x + 1..pxl_2.x {
            let mut coords = uvec2(x, inter_y.floor() as u32);
            let mut offset = uvec2(0, 1);
            if is_steep {
                coords = coords.swap();
                offset = offset.swap();
            }

            plot(coords, (1. - inter_y.fract()) as f32);
            plot(coords + offset, (inter_y.fract()) as f32);
            inter_y += gradient;
        }
    }
}

pub struct TriangleDrawParams {
    pub vertices: (RenderVertex, RenderVertex, RenderVertex),
}

pub struct UglyTriangleDrawer {
    buffer_dimensions: UDimensions,
}

impl UglyTriangleDrawer {
    pub fn new(dimensions: UDimensions) -> Self {
        Self {
            buffer_dimensions: dimensions,
        }
    }
}

impl Drawer for UglyTriangleDrawer {
    type Params = TriangleDrawParams;
    type Out = PixelData;

    fn draw<F>(&self, params: Self::Params, mut plotter: F)
    where
        F: FnMut(UVec2, Self::Out),
    {
        // let shader = params.shader;

        let mut vertices = params.vertices;

        fn weigh_vertex_data(
            vertices: &(RenderVertex, RenderVertex, RenderVertex),
            weights: &(f32, f32, f32),
            screen_pos: Vec2,
        ) -> PixelData {
            let albedo = vertices.0.albedo * weights.0 as f32
                + vertices.1.albedo * weights.1 as f32
                + vertices.2.albedo * weights.2 as f32;

            let normal = vertices.0.normal * weights.0
                + vertices.1.normal * weights.1
                + vertices.2.normal * weights.2;

            let depth = vertices.0.depth * weights.0 as f32
                + vertices.1.depth * weights.1 as f32
                + vertices.2.depth * weights.2 as f32;

            let vertex_color = apply_3_weights(
                (
                    vertices.0.vertex_color,
                    vertices.1.vertex_color,
                    vertices.2.vertex_color,
                ),
                *weights,
            );

            PixelData {
                albedo,
                normal,
                depth,
                vertex_color,
                pos: Vec3::ZERO,
                barycentric_weights: (*weights).into(),
                screen_pos,
            }
        }

        // todo!();
        let mut p0: IVec2 = normalized_to_buffer_space(vertices.0.pos, self.buffer_dimensions)
            .floor()
            .into();
        let mut p1: IVec2 = normalized_to_buffer_space(vertices.1.pos, self.buffer_dimensions)
            .floor()
            .into();
        let mut p2: IVec2 = normalized_to_buffer_space(vertices.2.pos, self.buffer_dimensions)
            .floor()
            .into();

        if p0.y == p1.y && p1.y == p2.y || p0.x == p1.x && p1.x == p2.x {
            return;
        }

        // if p0.y == p1.y && p1.y == p2.y {
        // // Sort by increasing x
        // if p1.x < p0.x {
        //     std::mem::swap(&mut p1, &mut p0);
        //     std::mem::swap(&mut params.vertices.1, &mut params.vertices.0);
        // }
        // if p2.x < p0.x {
        //     std::mem::swap(&mut p2, &mut p0);
        //     std::mem::swap(&mut params.vertices.2, &mut params.vertices.0);
        // }
        // if p2.x < p1.x {
        //     std::mem::swap(&mut p2, &mut p1);
        //     std::mem::swap(&mut params.vertices.2, &mut params.vertices.1);
        // }

        // let y = p0.y;

        // let x0 = p0.x;
        // let x1 = p1.x;
        // let x2 = p2.x;

        // let length_1 = x1 as f32 - x0 as f32;
        // let length_2 = x2 as f32 - x1 as f32;

        // let vertices = params.vertices;
        // let shader = params.shader;

        // for x in x0..x1 {
        //     let weight = ((x - x0) as f32) / (length_1);
        //     let weights = (1. - weight, weight, 0.);

        //     let data = weigh_vertex_data(&vertices, &weights, vec2(x as f32, y as f32));

        //     plotter(uvec2(x as u32, y as u32), shader.shade(data));
        // }
        // for x in x1..x2 {
        //     let weight = ((x - x1) as f32) / (length_2);
        //     let weights = (1. - weight, weight, 0.);

        //     let data = weigh_vertex_data(&vertices, &weights, vec2(x as f32, y as f32));

        //     plotter(uvec2(x as u32, y as u32), shader.shade(data));
        // }

        // return;
        // }

        // source: https://gabrielgambetta.com/computer-graphics-from-scratch/07-filled-triangles.html

        // Sort by increasing y
        if p1.y < p0.y {
            std::mem::swap(&mut p1, &mut p0);
            std::mem::swap(&mut vertices.1, &mut vertices.0);
        }
        if p2.y < p0.y {
            std::mem::swap(&mut p2, &mut p0);
            std::mem::swap(&mut vertices.2, &mut vertices.0);
        }
        if p2.y < p1.y {
            std::mem::swap(&mut p2, &mut p1);
            std::mem::swap(&mut vertices.2, &mut vertices.1);
        }

        // if p0.x == p1.x && p1.x == p2.x {
        //     let x = p0.y;

        //     let y0 = p0.y;
        //     let y1 = p1.y;
        //     let y2 = p2.y;

        //     let length_1 = y1 as f32 - y0 as f32;
        //     let length_2 = y2 as f32 - y1 as f32;

        //     let vertices = params.vertices;
        //     let shader = params.shader;

        //     for y in y0..y1 {
        //         let weight = ((y - y0) as f32) / (length_1);
        //         let weights = (1. - weight, weight, 0.);

        //         let data = weigh_vertex_data(&vertices, &weights, vec2(x as f32, y as f32));

        //         plotter(uvec2(x as u32, y as u32), shader.shade(data));
        //     }
        //     for y in y1..y2 {
        //         let weight = ((y - y1) as f32) / (length_2);
        //         let weights = (1. - weight, weight, 0.);

        //         let data = weigh_vertex_data(&vertices, &weights, vec2(x as f32, y as f32));

        //         plotter(uvec2(x as u32, y as u32), shader.shade(data));
        //     }

        //     return;
        // }

        fn interpolate(p0: IVec2, p1: IVec2) -> Vec<i32> {
            // source: https://gabrielgambetta.com/computer-graphics-from-scratch/06-lines.html#the-linear-interpolation-function

            if p0.y == p1.y {
                return vec![p0.x];
            }

            let slope = (p1.x - p0.x) as f32 / (p1.y - p0.y) as f32;

            let mut acc = p0.x as f32;

            (p0.y..=p1.y)
                .map(|_| {
                    let out = acc.floor() as i32;
                    acc += slope;
                    out
                })
                .collect()
        }

        // I know this code sucks ._.
        let widths_0_to_2 = interpolate(p0, p2);

        let widths_0_to_1 = interpolate(p0, p1);

        let mut widths_1_to_2 = interpolate(p1, p2);

        let mut widths_left = widths_0_to_2;
        let mut widths_right = widths_0_to_1;
        widths_right.pop();
        widths_right.append(&mut widths_1_to_2);

        let i = (widths_left.len() - 1) / 2;
        if widths_left[i] >= widths_right[i] {
            std::mem::swap(&mut widths_left, &mut widths_right);
        }

        for ((width_left, width_right), y) in
            std::iter::zip(std::iter::zip(&widths_left, &widths_right), p0.y..=p2.y)
        {
            for x in *width_left..*width_right {
                let weights = barycentric_weights(
                    (p0.into(), p1.into(), p2.into()),
                    vec2(x as f32, y as f32),
                );

                let data = weigh_vertex_data(&vertices, &weights, vec2(x as f32, y as f32));

                plotter(uvec2(x as u32, y as u32), data);
            }
        }

        // My own broken code
        // let h0 = p1.y - p0.y;
        // let h1 = p2.y - p1.y;
        // let h_tot = p2.y - p0.y;

        // let width_0 = (p2.x - p0.x) as f32;
        // let width_1 = (p1.x - p0.x) as f32;
        // let width_2 = (p1.x - p2.x) as f32;

        // for i in 0..h_tot {
        //     let p = i as f32 / h_tot as f32;
        //     let x_left = lerp(width_0, 0., p);
        //     let x_right = if i <= h0 {
        //         let p = i as f32 / h0 as f32;
        //         lerp(0., width_1, p) + (p0.x - p2.x) as f32
        //     } else {
        //         let p = (i - h0) as f32 / h1 as f32;
        //         lerp(width_2, 0., p)
        //     };

        //     // let color = if i <= h0 { params.3 } else { params.3 * 1.5 };
        //     let color = params.3;

        //     // Not sure if it should be `..` or `..=`
        //     let start = (p2.x - x_left.round() as i32) as u32;
        //     let stop = (p2.x + x_right.round() as i32) as u32;
        //     for x in start..stop {
        //         let point = uvec2(x, (p0.y + i) as u32);

        //         plotter(point, color);
        //     }
        // }
    }
}

pub struct BarycentricTriangleDrawer {
    buffer_dimensions: UDimensions,
}

impl BarycentricTriangleDrawer {
    pub fn new(dimensions: UDimensions) -> Self {
        Self {
            buffer_dimensions: dimensions,
        }
    }
}

impl Drawer for BarycentricTriangleDrawer {
    type Params = TriangleDrawParams;
    type Out = PixelData;

    fn draw<F>(&self, params: Self::Params, mut plotter: F)
    where
        F: FnMut(UVec2, Self::Out),
    {
        // let shader =
        let vertices = params.vertices;

        // let weights = barycentric_weights(vertices, point)

        let pixel_0: UVec2 = normalized_to_buffer_space(vertices.0.pos, self.buffer_dimensions)
            .floor()
            .into();
        let pixel_1: UVec2 = normalized_to_buffer_space(vertices.1.pos, self.buffer_dimensions)
            .floor()
            .into();
        let pixel_2: UVec2 = normalized_to_buffer_space(vertices.2.pos, self.buffer_dimensions)
            .floor()
            .into();

        let min_x = pixel_0.x.min(pixel_1.x.min(pixel_2.x));
        let max_x = pixel_0.x.max(pixel_1.x.max(pixel_2.x));

        let min_y = pixel_0.y.min(pixel_1.y.min(pixel_2.y));
        let max_y = pixel_0.y.max(pixel_1.y.max(pixel_2.y));

        // if min_x == max_x || min_y == max_y {
        //     return;
        // }

        for y in min_y..max_y {
            for x in min_x..max_x {
                let weights = barycentric_weights(
                    (pixel_0.into(), pixel_1.into(), pixel_2.into()),
                    vec2(x as f32, y as f32),
                );

                if weights.0 < 0.
                    || weights.1 < 0.
                    || weights.2 < 0.
                    || weights.0 > 1.
                    || weights.1 > 1.
                    || weights.2 > 1.
                {
                    // if any weights are negative, the point lies outside the triangle
                    // source: https://codeplea.com/triangular-interpolation
                    continue;
                }

                let albedo = apply_3_weights(
                    (vertices.0.albedo, vertices.1.albedo, vertices.2.albedo),
                    weights,
                );
                let normal = apply_3_weights(
                    (vertices.0.normal, vertices.1.normal, vertices.2.normal),
                    weights,
                );
                let depth = apply_3_weights(
                    (vertices.0.depth, vertices.1.depth, vertices.2.depth),
                    weights,
                );
                let vertex_color = apply_3_weights(
                    (
                        vertices.0.vertex_color,
                        vertices.1.vertex_color,
                        vertices.2.vertex_color,
                    ),
                    weights,
                );
                let screen_pos =
                    apply_3_weights((vertices.0.pos, vertices.1.pos, vertices.2.pos), weights);

                let data = PixelData {
                    albedo,
                    normal,
                    depth,
                    vertex_color,
                    pos: Vec3::ZERO, // TODO: Find a way to avoid this
                    barycentric_weights: weights.into(),
                    screen_pos,
                };

                plotter(uvec2(x, y), data);
            }
        }
    }
}
