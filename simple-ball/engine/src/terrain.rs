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
    // i.e. x is left->right and y is top->bottom
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
    pub fn as_heightfield_heights(&self, subdivisions: usize, max_value: Real) -> DMatrix<Real> {
        let min = self.elevations.min();
        let max = self.elevations.max();
        let range = max - min;
        let scale = max_value / range;
        let offset = min;

        let index_x_stride = (self.width - 1) / subdivisions;
        let index_y_stride = (self.height - 1) / subdivisions;
        DMatrix::from_fn(subdivisions, subdivisions, |i, j| {
            let index_x = j * index_x_stride;
            // let index_x = self.width - 1 - (i * index_x_stride);
            let index_y = self.height - 1 - (i * index_y_stride);
            // let index_y = (j * index_y_stride);
            let elevation = self.elevations.index((index_x, index_y));
            (elevation - offset) * scale
        })
    }
}

#[cfg(test)]
mod terrain_tests {
    use image::{Rgba, RgbaImage};
    use std::io::Cursor;
    use wasm_bindgen_test::*;
    use super::*;

    struct ElevationMapping {
        e: f32, 
        p: image::Rgba<u8>
    }

    #[allow(non_snake_case)]
    struct ElevationMappings {
        A: ElevationMapping,
        B: ElevationMapping,
        C: ElevationMapping,
    }

    fn elevation_mappings() -> ElevationMappings {
        ElevationMappings {
            // elevation = -10000 + (({R} * 256 * 256 + {G} * 256 + {B}) * 0.1)
            // elevation = -10
            // invert:
            // (-10 + 10000) / 0.1 = 99,900
            // 99,900 / (256^2) = 1 remainder 34,364
            // 34,364 / (256^1) = 134 remainder 60
            // 60 / (256^0) = 60
            A: ElevationMapping{ e: -10.0, p: Rgba([1, 134, 60, u8::MAX]) },
            // elevation = 0
            // invert:
            // (0 + 10000) / 0.1 = 100,000
            // 100,000 / (256^2) = 1 remainder 34,464
            // 34,464 / (256^1) = 134 remainder 160
            // 160 / (256^0) = 160
            B: ElevationMapping{ e: 0.0, p: Rgba([1, 134, 160, u8::MAX]) },
            // elevation = 5
            // invert:
            // (5 + 10000) / 0.1 = 100,050
            // 100,050 / (256^2) = 1 remainder 34,514
            // 34,514 / (256^1) = 134 remainder 210
            // 210 / (256^0) = 210
            C: ElevationMapping{ e: 5.0, p: Rgba([1, 134, 210, u8::MAX]) },
        }
    }

    #[wasm_bindgen_test]
    fn test_to_elevation() {
        let m = elevation_mappings();
        let examples = vec![m.A, m.B, m.C];
        for example in examples {
            let expected = example.e;
            let input = example.p;
            let actual = input.to_elevation();
            assert_eq!(expected, actual);
        }
    }

    #[wasm_bindgen_test]
    fn test_from_png_terrain_image() {
        let width = 6u32;
        let height = 6u32;

        let num_rows = height as usize;
        let num_columns = width as usize;
        let m = elevation_mappings();
        let expected_elevations = 
            DMatrix::from_row_slice(num_rows, num_columns, &[
                m.A.e, m.A.e, m.B.e, m.B.e, m.C.e, m.C.e,
                m.A.e, m.A.e, m.B.e, m.B.e, m.C.e, m.C.e,
                m.B.e, m.B.e, m.B.e, m.B.e, m.B.e, m.B.e,
                m.B.e, m.B.e, m.B.e, m.B.e, m.B.e, m.B.e,
                m.A.e, m.A.e, m.B.e, m.B.e, m.C.e, m.C.e,
                m.A.e, m.A.e, m.B.e, m.B.e, m.C.e, m.C.e,
            ]);

        let mut image_buffer: RgbaImage 
            = ImageBuffer::new(width, height);
        
        image_buffer.put_pixel(0, 0, m.A.p);
        image_buffer.put_pixel(1, 0, m.A.p);
        image_buffer.put_pixel(2, 0, m.B.p);
        image_buffer.put_pixel(3, 0, m.B.p);
        image_buffer.put_pixel(4, 0, m.C.p);
        image_buffer.put_pixel(5, 0, m.C.p);

        image_buffer.put_pixel(0, 1, m.A.p);
        image_buffer.put_pixel(1, 1, m.A.p);
        image_buffer.put_pixel(2, 1, m.B.p);
        image_buffer.put_pixel(3, 1, m.B.p);
        image_buffer.put_pixel(4, 1, m.C.p);
        image_buffer.put_pixel(5, 1, m.C.p);

        image_buffer.put_pixel(0, 2, m.B.p);
        image_buffer.put_pixel(1, 2, m.B.p);
        image_buffer.put_pixel(2, 2, m.B.p);
        image_buffer.put_pixel(3, 2, m.B.p);
        image_buffer.put_pixel(4, 2, m.B.p);
        image_buffer.put_pixel(5, 2, m.B.p);

        image_buffer.put_pixel(0, 3, m.B.p);
        image_buffer.put_pixel(1, 3, m.B.p);
        image_buffer.put_pixel(2, 3, m.B.p);
        image_buffer.put_pixel(3, 3, m.B.p);
        image_buffer.put_pixel(4, 3, m.B.p);
        image_buffer.put_pixel(5, 3, m.B.p);

        image_buffer.put_pixel(0, 4, m.A.p);
        image_buffer.put_pixel(1, 4, m.A.p);
        image_buffer.put_pixel(2, 4, m.B.p);
        image_buffer.put_pixel(3, 4, m.B.p);
        image_buffer.put_pixel(4, 4, m.C.p);
        image_buffer.put_pixel(5, 4, m.C.p);

        image_buffer.put_pixel(0, 5, m.A.p);
        image_buffer.put_pixel(1, 5, m.A.p);
        image_buffer.put_pixel(2, 5, m.B.p);
        image_buffer.put_pixel(3, 5, m.B.p);
        image_buffer.put_pixel(4, 5, m.C.p);
        image_buffer.put_pixel(5, 5, m.C.p);

        let image = DynamicImage::ImageRgba8(image_buffer);
        let mut cursor = Cursor::new(Vec::new());
        image.write_to(&mut cursor, image::ImageFormat::Png).unwrap();
        let data : Vec<u8> = cursor.get_ref().to_owned();

        let terrain = Terrain::from_png_terrain_image(data);

        assert_eq!(width, terrain.width as u32);
        assert_eq!(height, terrain.height as u32);
        assert_eq!(expected_elevations, terrain.elevations);

    }

}