
use nalgebra::Point2;
use rapier3d::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Dimension {
    pub side_length: f32
}

pub fn map_screen_to_arena(screen: &Dimension, arena: &Dimension, point: Point2<Real>, default_y: Real) -> Vector<Real> {
    let scale = arena.side_length / screen.side_length;
    let x = point.x * scale;
    let z = arena.side_length - (point.y * scale);
    vector![x, default_y, z]
}

pub fn map_arena_to_screen(screen: &Dimension, arena: &Dimension, vector: Vector<Real>) -> Point2<Real> {
    let scale = screen.side_length / arena.side_length;
    let x = vector.x * scale;
    let y = screen.side_length - (vector.z * scale);
    Point2::new(x, y)
}


#[cfg(test)]
mod mapping_tests {
    use wasm_bindgen_test::*;
    use super::*;

    struct Context {
        arena_dimension: Dimension,
        screen_dimension: Dimension,
        mappings: Vec<(Point2<Real>, Vector<Real>)>,
        default_y: Real
    }

    fn context() -> Context {
        let arena_dimension = Dimension {
            side_length: 10.0
        };
        let screen_dimension = Dimension {
            side_length: 100.0
        };
        let default_y = 0.123;
        let mappings = vec![
            (Point2::new(20.0, 20.0), vector![2.0, default_y, 8.0]),
            (Point2::new(50.0, 50.0), vector![5.0, default_y, 5.0]),
            (Point2::new(80.0, 80.0), vector![8.0, default_y, 2.0])
        ];
        Context {
            arena_dimension, screen_dimension, mappings, default_y
        }
    }

    #[wasm_bindgen_test]
    fn test_map_screen_to_arena() {
        let context = context();
        for mapping in &context.mappings {
            let (input, expected) = mapping;
            let actual 
                = map_screen_to_arena(&context.screen_dimension, &context.arena_dimension, *input, context.default_y);
            assert_eq!(*expected, actual);
        }
    }

    #[wasm_bindgen_test]
    fn test_map_arena_to_screen() {
        let context = context();
        for mapping in &context.mappings {
            let (expected, input) = mapping;
            let actual = map_arena_to_screen(&context.screen_dimension, &context.arena_dimension, *input);
            assert_eq!(*expected, actual);
        }
    }
}

