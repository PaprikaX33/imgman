use crate::image::Pix;

pub fn grayscale(pix: Pix) -> Pix {
    //0.299R + 0.587G + 0.114B
    //76R + 150G + 29B
    let (r, g, b, _a) = pix.to_tuple();
    let lum = (((r as u32) * 76 + (g as u32) * 150 + (b as u32) * 29) >> 8) as u8;
    Pix {
        r: lum,
        g: lum,
        b: lum,
        a: 0xff,
    }
}
