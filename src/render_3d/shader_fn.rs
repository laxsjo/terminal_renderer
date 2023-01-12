use super::*;
use shader::*;

pub fn default(data: PixelData, uniform: &SceneInfo) -> PixelOutput {
    const SHADOW_COLOR: f32 = 0.8;
    let mut shadow = data.normal.dot_product(-uniform.light_direction) as f32;

    shadow = (1. - SHADOW_COLOR) * shadow + SHADOW_COLOR;

    (data.albedo * shadow, data.depth)
}
