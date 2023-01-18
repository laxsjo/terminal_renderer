use super::*;
use crate::math;
use shader::*;
use std::{fmt::Debug, num::NonZeroUsize};

pub struct Renderer {
    buffer: RenderBuffer,
}

pub struct RenderVertex {
    pub pos: Vec2,
    pub normal: Vec3,
    pub depth: f32,
    pub albedo: Rgb,
    pub vertex_color: Rgb, // pub shaders: ShaderProgram<SceneInfo>,
}

pub struct SceneInfo {
    pub light_direction: Vec3,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = RenderBuffer::new(
            NonZeroUsize::new(width as usize).unwrap(),
            NonZeroUsize::new(height as usize).unwrap(),
        );
        Renderer { buffer }
    }

    fn normalized_to_buffer_space(&self, point: Vec2) -> UVec2 {
        let (width, height) = self.get_size();
        Vec2 {
            x: (point.x * width as f32 + point.x) / 2.,
            y: (point.y * height as f32 + point.y) / 2.,
        }
        .into()
    }

    // #[deprecated]
    // fn normalized_to_byte_color(&self, color: f32) -> u8 {
    //     (color.clamp(0., 1.) * 255.) as u8
    // }
    // #[deprecated]
    // fn byte_to_normalized_color(&self, color: u8) -> f32 {
    //     color as f32 / 255.
    // }

    pub fn buffer(&self) -> &RenderBuffer {
        &self.buffer
    }

    pub fn update_size(&mut self) {
        let (height, width) = crossterm::terminal::size().expect("Couldn't get terminal size");

        let buffer = RenderBuffer::new(
            NonZeroUsize::new(width as usize).unwrap(),
            NonZeroUsize::new(height as usize).unwrap(),
        );

        self.buffer = buffer;
    }

    pub fn resize(&mut self, width: usize, height: usize) -> Option<()> {
        self.buffer = RenderBuffer::new(NonZeroUsize::new(width)?, NonZeroUsize::new(height)?);

        Some(())
    }

    /// Returns the number of pixels as `(width, height)`.
    pub fn get_size(&self) -> (usize, usize) {
        (self.buffer.get_width(), self.buffer.get_height())
    }

    pub fn aspect_ratio(&self) -> f32 {
        let size = self.get_size();
        aspect_ratio(vec2(size.0 as f32, size.1 as f32))
    }

    // pub fn get_width(&self) -> usize {
    //     self.buffer.get_width()
    // }

    // pub fn get_height(&self) -> usize {
    //     self.buffer.get_height()
    // }

    pub fn get_pixel_value(&self, coords: UVec2) -> Option<Rgb> {
        self.buffer.get_pixel_color(coords)
    }

    pub fn draw_coords_overlay(&mut self, coords: Vec2, color: Rgb, depth: f32) {
        let coords = self.normalized_to_buffer_space(coords);

        let Some(orig_color) = self.get_pixel_value(coords) else {
            return;
        };

        self.buffer
            .set_pixel_color(coords, color + orig_color, depth)
            .unwrap_or(false);
    }
    pub fn draw_coords(&mut self, coords: Vec2, color: Rgb, depth: f32) {
        let coords = self.normalized_to_buffer_space(coords);

        self.buffer
            .set_pixel_color(coords, color, depth)
            .unwrap_or(false);
    }
    pub fn set_pixel_overlay(&mut self, coords: UVec2, color: Rgb, depth: f32) {
        let Some(orig_color) = self.get_pixel_value(coords) else {
            return;
        };
        self.buffer
            .set_pixel_color(coords, color + orig_color, depth)
            .unwrap_or(false);
    }
    pub fn set_pixel(&mut self, coords: UVec2, color: Rgb, depth: f32) {
        self.buffer
            .set_pixel_color(coords, color, depth)
            .unwrap_or(false);
    }

    /// Draws a line from point a to b, using normalized coordinates from
    /// `(-1,-1)` to `(1,1)`.
    /// Lines outside this range will be clipped.
    pub fn draw_line(&mut self, line: Line, color: Rgb) {
        const RENDER_FRAME: (Vec2, Vec2) = (vec2(-1., -1.), vec2(1., 1.));

        let Some(Line(a, b)) = math::clip_to_frame(line, RENDER_FRAME) else {
            return;
        };

        let drawer = AaLineDrawer::new(self.buffer.get_dimensions());

        drawer.draw(LineDrawParams(a, b, color), |coords, color| {
            self.set_pixel_overlay(coords, color, 0.);
        });
    }

    pub fn draw_triangle(
        &mut self,
        points: (RenderVertex, RenderVertex, RenderVertex),
        shader: &ShaderProgram<'_, SceneInfo>,
    ) {
        // const RENDER_FRAME: (Vec2, Vec2) = (vec2(-1., -1.), vec2(1., 1.));

        let drawer = UglyTriangleDrawer::new(self.buffer.get_dimensions());

        // if points.0.depth.is_nan() || points.1.depth.is_nan() || points.2.depth.is_nan() {
        //     panic!("{}, {}, {}", points.0.depth, points.1.depth, points.2.depth,)
        // }

        drawer.draw(
            TriangleDrawParams {
                vertices: (points.0, points.1, points.2),
            },
            |coords, data| {
                let (color, depth) = shader.shade_pixel(data);

                self.set_pixel(coords, color, depth);
            },
        )
    }

    pub fn render_object_wireframe(&mut self, object: &Object, camera: &impl Camera) {
        let aspect_ratio = self.aspect_ratio();
        for Edge(a, b) in object.mesh.edges_iter() {
            let projected_a =
                camera.project_point(object.transform.transform_point(a), aspect_ratio);
            let projected_b =
                camera.project_point(object.transform.transform_point(b), aspect_ratio);

            self.draw_line(Line(projected_a.xy(), projected_b.xy()), rgb(1., 1., 1.));
        }
    }

    pub fn render_object(
        &mut self,
        object: &Object,
        shader: &ShaderProgram<SceneInfo>,
        camera: &impl Camera,
    ) {
        for mut tri in object
            .mesh
            .triangles_iter()
            .map(|tri| tri.transform(&object.transform))
        {
            (tri.points.0, tri.vertex_colors.0) = shader.shade_vertex({
                VertexData {
                    albedo: object.color,
                    normal: tri.normals.0,
                    vertex_color: tri.vertex_colors.0,
                    pos: tri.points.0,
                }
            });
            (tri.points.1, tri.vertex_colors.1) = shader.shade_vertex({
                VertexData {
                    albedo: object.color,
                    normal: tri.normals.1,
                    vertex_color: tri.vertex_colors.1,
                    pos: tri.points.1,
                }
            });
            (tri.points.2, tri.vertex_colors.2) = shader.shade_vertex({
                VertexData {
                    albedo: object.color,
                    normal: tri.normals.2,
                    vertex_color: tri.vertex_colors.2,
                    pos: tri.points.2,
                }
            });

            let aspect_ratio = self.aspect_ratio();

            let projected_a = camera.project_point(tri.points.0, aspect_ratio);
            let projected_b = camera.project_point(tri.points.1, aspect_ratio);
            let projected_c = camera.project_point(tri.points.2, aspect_ratio);

            let normal = tri.normal();

            if camera.rotation().forward().dot_product(-normal) <= 0. {
                // polygon is facing away.
                continue;
            }

            self.draw_triangle(
                (
                    RenderVertex {
                        pos: projected_a.xy(),
                        albedo: object.color,
                        depth: -projected_a.z as f32,
                        normal: tri.normals.0,
                        vertex_color: tri.vertex_colors.0,
                    },
                    RenderVertex {
                        pos: projected_b.xy(),
                        albedo: object.color,
                        depth: -projected_b.z as f32,
                        normal: tri.normals.1,
                        vertex_color: tri.vertex_colors.1,
                    },
                    RenderVertex {
                        pos: projected_c.xy(),
                        albedo: object.color,
                        depth: -projected_c.z as f32,
                        normal: tri.normals.2,
                        vertex_color: tri.vertex_colors.2,
                    },
                ),
                shader,
            );
        }
    }

    pub fn render_scene(&mut self, scene: &Scene) {
        let Some(camera) = scene.get_camera() else {
            self.buffer.clear();
            return;
        };

        let shader = ShaderProgram::new(
            SceneInfo {
                light_direction: scene.light_direction,
            },
            &|data, uniform| {
                shader_fn::default(
                    PixelData {
                        albedo: data.vertex_color,
                        ..data
                    },
                    uniform,
                )
            },
            &|data, _| {
                let pos = (data.pos + vec3(1., 1., 1.)) / 2.;

                (data.pos, pos.into())
            },
        );

        for object in scene.objects() {
            self.render_object(object, &shader, camera);
        }
    }

    // pub fn render_test(&mut self) {
    //     let a0 = vec2(-0.6, -0.5);
    //     let b0 = vec2(-0.7, 0.3);
    //     let c0 = vec2(-0.2, 0.4);
    //     let d0 = vec2(-0.1, -0.2);
    //     // self.draw_triangle((a0, b0, d0), rgb(0.5, 0.5, 0.5));
    //     // self.draw_triangle((b0, c0, d0), rgb(0.5, 0.5, 0.5));

    //     let a1 = vec2(0.2, -0.1);
    //     let b1 = vec2(0.1, 0.3);
    //     let c1 = vec2(0.6, 0.4);
    //     let d1 = vec2(0.7, -0.2);
    //     // self.draw_triangle((a1, b1, d1), rgb(0.5, 0.5, 0.5));
    //     // self.draw_triangle((b1, c1, d1), rgb(0.5, 0.5, 0.5));

    //     // self.draw_line(Line(a, b), 1.);
    //     // self.draw_line(Line(b, c), 1.);
    //     // self.draw_line(Line(c, a), 1.);
    // }

    pub fn clear(&mut self) {
        self.buffer.clear()
    }
}

impl Debug for Renderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Renderer")
    }
}
