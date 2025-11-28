use image::error::ImageResult;
use std::vec::Vec;
#[derive(Clone, Copy, Debug)]
struct Pix {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

struct Dimension {
    w: u8,
    h: u8,
}
pub struct Image {
    dim: Dimension,
    data: std::vec::Vec<Pix>,
}

impl Image {
    pub fn open(path: &str) -> ImageResult<Self> {
        return Ok(Self {
            data: vec![],
            ..Default::default()
        });
    }
}

impl Default for Dimension {
    fn default() -> Self {
        Self { w: 0, h: 0 }
    }
}

impl Default for Image {
    fn default() -> Self {
        Self {
            dim: Dimension::default(),
            data: Vec::new(),
        }
    }
}
