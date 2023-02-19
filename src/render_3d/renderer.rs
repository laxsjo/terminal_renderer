use super::*;
use crate::math;
use shader::*;
use std::fmt::Debug;
use std::thread;

pub struct Renderer {
    buffer: RenderBuffer,
    pub thread_count: NonZeroUsize,
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
    pub fn new(width: usize, height: usize, thread_count: usize) -> Self {
        println!("Created renderer of size {}x{}", width, height);
        let buffer = RenderBuffer::new(
            non_zero_udimensions(width, height).unwrap(),
            // NonZeroUsize::new(segment_height).unwrap(),
        );
        Renderer {
            buffer,
            thread_count: NonZeroUsize::new(thread_count).unwrap(),
        }
    }

    fn normalized_to_buffer_space(&self, point: Vec2) -> UVec2 {
        // this is wrong I think /rasmus@Feb2023
        let (width, height) = self.size();
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

    pub fn segment_heights(&self) -> NonZeroUsize {
        NonZeroUsize::new(self.buffer.height().get() / self.thread_count.get())
            .expect("non zero division inputs resulted zero value")
    }

    pub fn update_size(&mut self) {
        let (height, width) = crossterm::terminal::size().expect("Couldn't get terminal size");

        let buffer =
            RenderBuffer::new(non_zero_udimensions(width as usize, height as usize).unwrap());

        self.buffer = buffer;
    }

    pub fn resize(&mut self, width: usize, height: usize) -> Option<()> {
        self.buffer = RenderBuffer::new(non_zero_udimensions(width, height)?);

        Some(())
    }

    /// Returns the number of pixels as `(width, height)`.
    pub fn size(&self) -> (usize, usize) {
        (self.buffer.width().get(), self.buffer.height().get())
    }

    pub fn aspect_ratio(&self) -> f32 {
        let size = self.size();
        aspect_ratio(vec2(size.0 as f32, size.1 as f32))
    }

    // pub fn get_width(&self) -> usize {
    //     self.buffer.get_width()
    // }

    // pub fn get_height(&self) -> usize {
    //     self.buffer.get_height()
    // }

    pub fn get_pixel_value(&self, coords: UVec2) -> Option<Rgb> {
        self.buffer.pixel_color(coords)
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

        let drawer = AaLineDrawer::new(self.buffer.size().get());

        drawer.draw(LineDrawParams(a, b, color), |coords, color| {
            self.set_pixel_overlay(coords, color, 0.);
        });
    }

    // TODO: Actually use the segmented render buffer
    pub fn draw_triangle(
        buffer: &mut RenderBufferSegmentMut,
        points: (RenderVertex, RenderVertex, RenderVertex),
        shader: &ShaderProgram<'_, SceneInfo>,
    ) {
        let triangle = (
            buffer.normalized_to_buffer_space(points.0.pos),
            buffer.normalized_to_buffer_space(points.1.pos),
            buffer.normalized_to_buffer_space(points.2.pos),
        );
        if buffer.region().triangle_outside(triangle) {
            // if !buffer.region().includes_point(triangle.0) {
            // println!(
            //     "region {:?} didn't include point {}, screen coordinates {}",
            //     buffer.region(),
            //     triangle.0,
            //     points.0.pos
            // );
            // points.0.vertex_color = rgb(1., 0., 0.);
            // points.1.vertex_color = rgb(1., 0., 0.);
            // points.2.vertex_color = rgb(1., 0., 0.);
            return;
        }

        let drawer = UglyTriangleDrawer::new(buffer.parent_size());

        drawer.draw(
            TriangleDrawParams {
                vertices: (points.0, points.1, points.2),
            },
            move |coords, data| {
                let (color, depth) = shader.shade_pixel(data);

                buffer.set_pixel(coords, color, depth);

                // self.set_pixel(coords, color, depth);
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
        buffer: &mut RenderBufferSegmentMut,
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

            let aspect_ratio = buffer.parent_aspect_ratio();

            let projected_a = camera.project_point(tri.points.0, aspect_ratio);
            let projected_b = camera.project_point(tri.points.1, aspect_ratio);
            let projected_c = camera.project_point(tri.points.2, aspect_ratio);

            let normal = tri.normal();

            if camera.rotation().forward().dot_product(-normal) <= 0. {
                // polygon is facing away.
                continue;
            }

            Self::draw_triangle(
                buffer,
                (
                    RenderVertex {
                        pos: projected_a.xy(),
                        albedo: object.color,
                        depth: -projected_a.z,
                        normal: tri.normals.0,
                        vertex_color: tri.vertex_colors.0,
                    },
                    RenderVertex {
                        pos: projected_b.xy(),
                        albedo: object.color,
                        depth: -projected_b.z,
                        normal: tri.normals.1,
                        vertex_color: tri.vertex_colors.1,
                    },
                    RenderVertex {
                        pos: projected_c.xy(),
                        albedo: object.color,
                        depth: -projected_c.z,
                        normal: tri.normals.2,
                        vertex_color: tri.vertex_colors.2,
                    },
                ),
                shader,
            );
        }
    }

    pub fn render_scene(&mut self, scene: &Scene) {
        let segment = self.buffer.as_single_segment_mut();

        Self::render_scene_segment(segment, scene);
    }

    pub fn render_scene_multi_thread(&mut self, scene: &Scene) {
        let segments = self
            .buffer
            .separate_into_segments_mut(self.segment_heights());
        // println!("Separated into {} segments", segments.len());

        thread::scope(|scope| {
            let mut handles = Vec::new();

            for segment in segments {
                handles.push(scope.spawn(move || {
                    Renderer::render_scene_segment(segment, scene);
                }));
            }

            for handle in handles {
                handle.join().unwrap();
            }
        })
    }

    fn render_scene_segment(mut segment: RenderBufferSegmentMut, scene: &Scene) {
        let Some(camera) = scene.get_camera() else {
            segment.clear();
            return;
        };

        segment.clear();

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

        // println!("{:?}", segment.region());

        for object in scene.objects() {
            Self::render_object(&mut segment, object, &shader, camera);
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear()
    }
}

impl Debug for Renderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Renderer")
    }
}
