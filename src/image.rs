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
    w: u32,
    h: u32,
}
pub struct Image {
    dim: Dimension,
    data: std::vec::Vec<Pix>,
}

impl Image {
    pub fn open(path: &str) -> ImageResult<Self> {
        let img = image::open(path)?.to_rgba8();
        let dim = img.dimensions().into();
        let raw = img.into_raw();

        let data: Vec<Pix> = raw
            .chunks_exact(4)
            .map(|dot| Pix {
                r: dot[0],
                g: dot[1],
                b: dot[2],
                a: dot[3],
            })
            .collect();
        return Ok(Self { data, dim });
    }
}

impl Default for Dimension {
    fn default() -> Self {
        Self { w: 0, h: 0 }
    }
}

impl From<(u32, u32)> for Dimension {
    fn from((w, h): (u32, u32)) -> Self {
        Self { w, h }
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
