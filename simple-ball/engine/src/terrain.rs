use rapier3d::prelude::*;
use wasm_bindgen::prelude::*;
use image::{GenericImageView, DynamicImage, ImageBuffer};

use crate::log::*;

trait Elevation {
    fn to_elevation(&self) -> Real;
}

impl Elevation for image::Rgba<u8> {
    fn to_elevation(&self) -> Real {
        let (r, g, b) = (self[0] as f32, self[1] as f32, self[2] as f32);
        let elevation = -10000.0 + ((r * 256.0 * 256.0 + g * 256.0 + b) * 0.1);
        elevation
    }
}

#[wasm_bindgen]
pub struct Terrain {
    // elevations as stored in a matrix where
    // x = columns, y = rows, where x, y is in screen space
    // i.e. x goes from left->right and y goes from top->bottom
    elevations: DMatrix<Real>,
    pub width: usize,
    pub height: usize
}

#[wasm_bindgen]
impl Terrain {
    pub fn from_png_terrain_image(data: Vec<u8>) -> Terrain {
        console_log!("reading image");
        let result = 
            image::load_from_memory_with_format(&data, 
                image::ImageFormat::Png);
        let image = result.unwrap();
        console_log!("read image");

        // let elevations 
        //     = DMatrix::from_fn(image.width() as usize, image.height() as usize, |x, y| {
        //         image.get_pixel(x as u32, y as u32).to_elevation()
        // });
        let elevations 
            = DMatrix::from_fn(image.height() as usize, image.width() as usize, |y, x| {
                image.get_pixel(x as u32, y as u32).to_elevation()
        });

        Terrain { 
            elevations, 
            width: image.width() as usize, 
            height: image.height() as usize
        }
    }

    pub fn halfed(&self) -> Terrain {
        Terrain { 
            elevations: DMatrix::from_fn(self.height / 2, self.width / 2, |i, j| {
                let stride = 2;
                let start = (i * stride, j * stride);
                let shape = (stride, stride);
                let slice 
                    = self.elevations.slice(start, shape);
                let avg = slice.sum() / (slice.len() as Real);
                avg
            }),
            width: self.width / 2,
            height: self.height / 2
        }
    }

    pub fn as_grayscale_height_image(&self) -> Vec<u8> {
        use std::io::Cursor;

        let min = self.elevations.min();
        let max = self.elevations.max();
        let range = max - min;
        let max_luma = u16::MAX as f32;
        let scale = max_luma / range;
        let offset = min;

        let image_buffer 
            = ImageBuffer::from_fn(self.width as u32, self.height as u32, |x, y| {
            let elevation = self.elevations.index((x as usize, y as usize));
            let luma = ((elevation - offset) * scale) as u16;
            image::Luma([luma])
        });

        let image = DynamicImage::ImageLuma16(image_buffer);
        
        console_log!("writing image");
        let mut cursor = Cursor::new(Vec::new());
        image::write_buffer_with_format(
            &mut cursor, 
            image.as_bytes(), 
            image.width(), 
            image.height(),
            image.color(),
            image::ImageFormat::Png
        ).unwrap();
        console_log!("wrote image");
        cursor.get_ref().clone()
    }
}

impl Terrain {
    pub fn as_xz_heightfield(&self, max_value: Real) -> DMatrix<Real> {
        let min = self.elevations.min();
        let max = self.elevations.max();
        let range = max - min;
        let scale = max_value / range;
        let offset = min;

        DMatrix::from_fn(self.height, self.width, |z, x| {
            let j = self.height - 1 - z;
            let i = x;
            let elevation = self.elevations.index((i, j));
            (elevation - offset) * scale
        })
    }
}

#[cfg(test)]
mod terrain_tests {
    use image::RgbaImage;
    use std::io::Cursor;
    use wasm_bindgen_test::*;
    use super::*;

    pub struct ElevationMapping {
        e: f32, 
        p: image::Rgba<u8>
    }

    mod examples {
        use image::Rgba;
        use super::ElevationMapping;

        // elevation = -10000 + (({R} * 256 * 256 + {G} * 256 + {B}) * 0.1)
        // elevation = -10
        // invert:
        // (-10 + 10000) / 0.1 = 99,900
        // 99,900 / (256^2) = 1 remainder 34,364
        // 34,364 / (256^1) = 134 remainder 60
        // 60 / (256^0) = 60
        pub const A : ElevationMapping = ElevationMapping{ e: -10.0, p: Rgba([1, 134, 60, u8::MAX]) };
        // elevation = 0
        // invert:
        // (0 + 10000) / 0.1 = 100,000
        // 100,000 / (256^2) = 1 remainder 34,464
        // 34,464 / (256^1) = 134 remainder 160
        // 160 / (256^0) = 160
        pub const B: ElevationMapping = ElevationMapping{ e: 0.0, p: Rgba([1, 134, 160, u8::MAX]) };
        // elevation = 5
        // invert:
        // (5 + 10000) / 0.1 = 100,050
        // 100,050 / (256^2) = 1 remainder 34,514
        // 34,514 / (256^1) = 134 remainder 210
        // 210 / (256^0) = 210
        pub const C: ElevationMapping = ElevationMapping{ e: 5.0, p: Rgba([1, 134, 210, u8::MAX]) };
        // elevation = 50
        // invert:
        // (50 + 10000) / 0.1 = 100,500
        // 100,500 / (256^2) = 1 remainder 34,964
        // 34,964 / (256^1) = 136 remainder 148
        // 148 / (256^0) = 148
        pub const D: ElevationMapping = ElevationMapping{ e: 50.0, p: Rgba([1, 136, 148, u8::MAX]) };
    }

    #[wasm_bindgen_test]
    fn test_to_elevation() {
        use examples::*;
        
        let examples = vec![A, B, C, D];
        for example in examples {
            let expected = example.e;
            let input = example.p;
            let actual = input.to_elevation();
            assert_eq!(expected, actual);
        }
    }

    fn example_terrain() -> Terrain {
        use examples::*;

        let height = 6usize;
        let width = 6usize;
            let elevations = 
            DMatrix::from_row_slice(width, height, &[
                A.e, A.e, B.e, B.e, C.e, C.e,
                A.e, A.e, B.e, B.e, C.e, C.e,
                B.e, B.e, B.e, B.e, B.e, B.e,
                B.e, B.e, B.e, B.e, B.e, B.e,
                A.e, A.e, B.e, B.e, D.e, D.e,
                A.e, A.e, B.e, B.e, D.e, D.e,
            ]);

        Terrain {
            elevations,
            width,
            height
        }
    }

    fn halfed_terrain() -> Terrain {
        use examples::*;

        let height = 3usize;
        let width = 3usize;
        let elevations = 
            DMatrix::from_row_slice(width, height, &[
                A.e, B.e, C.e,
                B.e, B.e, B.e,
                A.e, B.e, D.e,
            ]);

        Terrain {
            elevations,
            width,
            height
        }
    }

    #[wasm_bindgen_test]
    fn test_from_png_terrain_image() {
        use examples::*;

        let width = 6u32;
        let height = 6u32;

        let mut image_buffer: RgbaImage 
            = ImageBuffer::new(width, height);
        
        image_buffer.put_pixel(0, 0, A.p);
        image_buffer.put_pixel(1, 0, A.p);
        image_buffer.put_pixel(2, 0, B.p);
        image_buffer.put_pixel(3, 0, B.p);
        image_buffer.put_pixel(4, 0, C.p);
        image_buffer.put_pixel(5, 0, C.p);

        image_buffer.put_pixel(0, 1, A.p);
        image_buffer.put_pixel(1, 1, A.p);
        image_buffer.put_pixel(2, 1, B.p);
        image_buffer.put_pixel(3, 1, B.p);
        image_buffer.put_pixel(4, 1, C.p);
        image_buffer.put_pixel(5, 1, C.p);

        image_buffer.put_pixel(0, 2, B.p);
        image_buffer.put_pixel(1, 2, B.p);
        image_buffer.put_pixel(2, 2, B.p);
        image_buffer.put_pixel(3, 2, B.p);
        image_buffer.put_pixel(4, 2, B.p);
        image_buffer.put_pixel(5, 2, B.p);

        image_buffer.put_pixel(0, 3, B.p);
        image_buffer.put_pixel(1, 3, B.p);
        image_buffer.put_pixel(2, 3, B.p);
        image_buffer.put_pixel(3, 3, B.p);
        image_buffer.put_pixel(4, 3, B.p);
        image_buffer.put_pixel(5, 3, B.p);

        image_buffer.put_pixel(0, 4, A.p);
        image_buffer.put_pixel(1, 4, A.p);
        image_buffer.put_pixel(2, 4, B.p);
        image_buffer.put_pixel(3, 4, B.p);
        image_buffer.put_pixel(4, 4, D.p);
        image_buffer.put_pixel(5, 4, D.p);

        image_buffer.put_pixel(0, 5, A.p);
        image_buffer.put_pixel(1, 5, A.p);
        image_buffer.put_pixel(2, 5, B.p);
        image_buffer.put_pixel(3, 5, B.p);
        image_buffer.put_pixel(4, 5, D.p);
        image_buffer.put_pixel(5, 5, D.p);

        let image = DynamicImage::ImageRgba8(image_buffer);
        let mut cursor = Cursor::new(Vec::new());
        image.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
        let data : Vec<u8> = cursor.get_ref().to_owned();

        let expected_terrain = example_terrain();
        let terrain = Terrain::from_png_terrain_image(data);

        assert_eq!(width, terrain.width as u32);
        assert_eq!(height, terrain.height as u32);
        assert_eq!(expected_terrain.elevations, terrain.elevations);

    }

    #[wasm_bindgen_test]
    fn test_halfed() {
        let initial = example_terrain();
        let expected = halfed_terrain();
        let actual = initial.halfed();

        assert_eq!(expected.width, actual.width);
        assert_eq!(expected.height, actual.height);
        assert_eq!(expected.elevations, actual.elevations);
    }

}