use super::*;
// use crate::math::*;

pub struct PixelData {
    pub albedo: Rgb,
    pub normal: Vec3,
    pub depth: f32,
    pub vertex_color: Rgb,
    pub pos: Vec3,
    pub barycentric_weights: Vec3,
    pub screen_pos: Vec2,
}

pub struct VertexData {
    pub albedo: Rgb,
    pub normal: Vec3,
    pub vertex_color: Rgb,
    pub pos: Vec3,
}

pub type PixelOutput = (Rgb, f32);
pub type VertexOutput = (Vec3, Rgb);

pub trait PixelShader<U>: Fn(PixelData, &U) -> PixelOutput {}
pub trait VertexShader<U>: Fn(VertexData, &U) -> VertexOutput {}

impl<T, U> PixelShader<U> for T where T: Fn(PixelData, &U) -> PixelOutput {}
impl<T, U> VertexShader<U> for T where T: Fn(VertexData, &U) -> VertexOutput {}

pub struct ShaderProgram<'a, U> {
    pub uniform: U,
    pixel_shader: &'a dyn PixelShader<U>,
    vertex_shader: &'a dyn VertexShader<U>,
}

impl<'a, U> ShaderProgram<'a, U> {
    pub fn new(
        uniform: U,
        pixel_shader: &'a dyn PixelShader<U>,
        vertex_shader: &'a dyn VertexShader<U>,
    ) -> Self {
        Self {
            uniform,
            pixel_shader,
            vertex_shader,
        }
    }

    pub fn shade_pixel(&self, data: PixelData) -> PixelOutput {
        (self.pixel_shader)(data, &self.uniform)
    }

    pub fn shade_vertex(&self, data: VertexData) -> VertexOutput {
        (self.vertex_shader)(data, &self.uniform)
    }
}

// pub trait PixelShader {
//     type UniformData;

//     fn new(data: Self::UniformData) -> Self;

//     fn shade(&self, data: PixelData) -> (Rgb, f32);
// }

pub struct DefaultShader {
    pub uniform: SceneInfo,
}

// impl PixelShader for DefaultShader {
//     type UniformData = SceneInfo;

//     fn new(uniform: Self::UniformData) -> Self {
//         Self { uniform }
//     }

//     fn shade(&self, data: PixelData) -> (Rgb, f32) {
//         const SHADOW_COLOR: f32 = 0.8;
//         let mut shadow = data.normal.dot_product(-self.uniform.light_direction) as f32;

//         shadow = (1. - SHADOW_COLOR) * shadow + SHADOW_COLOR;

//         (data.albedo * shadow, data.depth)

//         // let color = 1. / (data.depth / 0.5) - 0.5;
//         // let color = (data.depth / 4.) - 0.2;

//         // (color.into(), data.depth)

//         // (Rgb::from_normal(data.normal), data.depth)
//     }
// }

pub fn identity_vertex_shader<T>(data: VertexData, _: &T) -> VertexOutput {
    (data.pos, data.vertex_color)
}
