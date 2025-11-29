use image::error::ImageResult;
use std::vec::Vec;
#[derive(Clone, Copy, Debug)]
pub struct Pix {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Default)]
struct Dimension {
    w: u32,
    h: u32,
}

#[derive(Default)]
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
        Ok(Self { data, dim })
    }

    pub fn write(&self, path: &str) -> ImageResult<()> {
        let raw_data: Vec<u8> = self
            .data
            .iter()
            .flat_map(|p| [p.r, p.g, p.b, p.a])
            .collect();

        let dim = &self.dim;
        let out_img = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(dim.w, dim.h, raw_data)
            .ok_or(std::io::Error::other("Internal processing function error"))?;
        out_img.save(path)?;

        Ok(())
    }

    pub fn apply_per_pixel<F>(&mut self, oper: F) -> std::io::Result<()>
    where
        F: Fn(Pix) -> Pix,
    {
        self.data.iter_mut().for_each(|x| *x = oper(*x));
        Ok(())
    }
}

impl From<(u32, u32)> for Dimension {
    fn from((w, h): (u32, u32)) -> Self {
        Self { w, h }
    }
}

impl Pix {
    pub fn to_tuple(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
}
