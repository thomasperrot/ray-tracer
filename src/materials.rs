#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub color: image::Rgb<u8>,
    pub specular: bool,
    pub refractive_index: f32,
}
