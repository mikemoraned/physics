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
#[derive(Clone)]
pub struct Terrain {
    // elevations as stored in a matrix where
    // x = columns, y = rows, where x, y is in screen space
    // i.e. x goes from left->right and y goes from top->bottom
    elevations: DMatrix<Real>,
    pub width: usize,
    pub height: usize
}

impl Terrain {
    pub fn rows(&self) -> usize {
        self.height
    }

    pub fn columns(&self) -> usize {
        self.width
    }
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

        let rows = image.height() as usize;
        let columns = image.width() as usize;
        let elevations 
            = DMatrix::from_fn(rows, columns, |row, column| {
                let x = column as u32;
                let y = row as u32;
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
            elevations: DMatrix::from_fn(self.rows() / 2, self.columns() / 2, |row, column| {
                let stride = 2;
                let start = (row * stride, column * stride);
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

    pub fn shrink_to_fit(&self, dimension: usize) -> Terrain {
        let mut terrain = self.clone();
        while terrain.width > dimension || terrain.height > dimension {
            terrain = terrain.halfed().clone();
        }
        terrain
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
            let row = y as usize;
            let column = x as usize;
            let elevation = self.elevations.index((row as usize, column as usize));
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

        DMatrix::from_fn(self.rows(), self.columns(), |row, column| {
            let flipped_row = self.rows() - 1 - row;
            let elevation = self.elevations.index((flipped_row, column));
            (elevation - offset) * scale
        })
    }
}

#[cfg(test)]
mod terrain_tests {
    use image::{RgbaImage, Rgba};
    use rapier3d::na::dmatrix;
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
        nalgebra::dmatrix![
            A.e, A.e, B.e, B.e, C.e, C.e;
            A.e, A.e, B.e, B.e, C.e, C.e;
            B.e, B.e, B.e, B.e, B.e, B.e;
            B.e, B.e, B.e, B.e, B.e, B.e;
            A.e, A.e, B.e, B.e, D.e, D.e;
            A.e, A.e, B.e, B.e, D.e, D.e
        ];

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
            nalgebra::dmatrix![
                A.e, B.e, C.e;
                B.e, B.e, B.e;
                A.e, B.e, D.e;
            ];

        Terrain {
            elevations,
            width,
            height
        }
    }

    fn subpixels(pixels: &[Rgba<u8>]) -> Vec<u8> {
        pixels.iter()
        .flat_map(|p| vec![p[0], p[1], p[2], p[3]])
        .collect()
    }

    #[wasm_bindgen_test]
    fn test_from_png_terrain_image() {
        use examples::*;

        let width = 6u32;
        let height = 6u32;

        let image_buffer: RgbaImage = ImageBuffer::from_vec(width, height, subpixels(&[
            A.p, A.p, B.p, B.p, C.p, C.p,
            A.p, A.p, B.p, B.p, C.p, C.p,
            B.p, B.p, B.p, B.p, B.p, B.p,
            B.p, B.p, B.p, B.p, B.p, B.p,
            A.p, A.p, B.p, B.p, D.p, D.p,
            A.p, A.p, B.p, B.p, D.p, D.p
        ])).unwrap();

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

    #[wasm_bindgen_test]
    fn test_image_understanding() {
        use examples::*;

        let width = 2u32;
        let height = 2u32;

        let mut image_buffer1: RgbaImage 
            = ImageBuffer::new(width, height);

        image_buffer1.put_pixel(0, 0, A.p);
        image_buffer1.put_pixel(1, 0, B.p);
        image_buffer1.put_pixel(0, 1, C.p);
        image_buffer1.put_pixel(1, 1, D.p);

        let actual_subpixels1 = image_buffer1.to_vec();
        let expected_subpixels1 = subpixels(&[A.p, B.p, C.p, D.p]);

        assert_eq!(expected_subpixels1, actual_subpixels1);

        let image_buffer2 : RgbaImage
            = ImageBuffer::from_vec(width, height, subpixels(&[A.p, B.p, C.p, D.p])).unwrap();
        
        assert_eq!(A.p, *image_buffer2.get_pixel(0, 0));
        assert_eq!(B.p, *image_buffer2.get_pixel(1, 0));
        assert_eq!(C.p, *image_buffer2.get_pixel(0, 1));
        assert_eq!(D.p, *image_buffer2.get_pixel(1, 1));
    }

    #[wasm_bindgen_test]
    fn test_nalgebra_understanding() {
        let width = 3;        
        let columns = width;
        let height = 2;
        let rows = height;

        let input = vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
        ];
        let direct = dmatrix![
            0, 1, 2;
            3, 4, 5;
        ];
        assert_eq!(rows, direct.nrows());
        assert_eq!(columns, direct.ncols());

        let from_fn = DMatrix::from_fn(rows, columns, |row, column| {
            let x = column;
            let y = row;
            input[y][x]
        });

        assert_eq!(direct, from_fn);
    }
}