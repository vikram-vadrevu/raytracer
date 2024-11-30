use image::{Pixel, RgbaImage};
use crate::raytracer::{MatVec, Color};

pub struct Texture2d {
    width: u32,
    height: u32,
    image: RgbaImage,
}

impl Texture2d {
    pub fn new(file: &String) -> Texture2d {
        println!("Loading texture from file: {}", file);
        let image = image::open(file).unwrap().to_rgba8();
        let (width, height) = image.dimensions();
        Texture2d {
            width,
            height,
            image,
        }
    }

    pub fn sample(&self, uv_coord: MatVec<2>) -> Color {
        let x = (uv_coord[0] * (self.width - 1) as f32) as u32;
        let y = (uv_coord[1] * (self.height - 1) as f32) as u32;
        let pixel = self.image.get_pixel(x, y);
        let channels = pixel.channels();
        
        // Convert sRGB to linear RGB; un gamma-correct
        let srgb_to_rgb = |c: u8| {
            let c = c as f32 / 255.0;
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };

        let r = srgb_to_rgb(channels[0]);
        let g = srgb_to_rgb(channels[1]);
        let b = srgb_to_rgb(channels[2]);

        MatVec::new(vec![r, g, b])
    }
}