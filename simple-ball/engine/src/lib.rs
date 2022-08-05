use image::{GenericImageView, DynamicImage, ImageBuffer};
use wasm_bindgen::prelude::*;
use rapier3d::prelude::*;
use nalgebra::Point2;

mod log;

use log::*;

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
    width: usize,
    height: usize
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

#[wasm_bindgen]
struct RapierState {
    rigid_body_set:  RigidBodySet,
    collider_set:  ColliderSet,
    gravity: Vector<Real>,
    integration_parameters:  IntegrationParameters,
    physics_pipeline:  PhysicsPipeline,
    island_manager:  IslandManager,
    broad_phase:  BroadPhase,
    narrow_phase:  NarrowPhase,
    impulse_joint_set:  ImpulseJointSet,
    multibody_joint_set:  MultibodyJointSet,
    ccd_solver:  CCDSolver,
    ball_body_handles: Vec<RigidBodyHandle>,
    ball_radius: f32
}

#[derive(Debug)]
struct Scene {
    arena_side_length: f32
}

impl Scene {
    fn map_view_to_arena(&self, view: &View, point: Point2<Real>, default_y: Real) -> Vector<Real> {
        let scale = self.arena_side_length / view.side_length;
        let x = point.x * scale;
        let z = self.arena_side_length - (point.y * scale);
        vector![x, default_y, z]
    }

    fn map_arena_to_view(&self, view: &View, vector: Vector<Real>) -> Point2<Real> {
        let scale = view.side_length / self.arena_side_length;
        let x = vector.x * scale;
        let y = view.side_length - (vector.z * scale);
        Point2::new(x, y)
    }
}

#[wasm_bindgen]
impl RapierState {
    fn new(ball_translations: Vec<Vector<Real>>, terrain: &Terrain, scene: &Scene) -> RapierState {

        console_log!("Creating RapierState");

        let ball_radius = 0.01 * scene.arena_side_length;

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let side_length = scene.arena_side_length;
        let thickness = 0.1;
        /* ground. */
        let ground = ColliderBuilder::cuboid(side_length, thickness, side_length)
            .translation(vector![0.0, -thickness, 0.0])
            .build();
        collider_set.insert(ground);

        /* heightfield */
        let height_y_extent = 100.0;
        let ground_size 
            = Vector::new(side_length, height_y_extent, side_length);
        let subdivisions : usize = 100;
        let max_heightfield = ball_radius;
        let heights 
            = terrain.as_heightfield_heights(subdivisions, max_heightfield);
        let heightfield = ColliderBuilder::heightfield(heights, ground_size)
            .translation(vector![0.5 * side_length, 0.0, 0.5 * side_length])
            .build();
        collider_set.insert(heightfield);

        /* walls */
        let wall_y_extent = 100.0;
        let wall1 = ColliderBuilder::cuboid(thickness, wall_y_extent, side_length)
            .translation(vector![-thickness, 0.0, 0.0])
            .build();
        let wall2 = ColliderBuilder::cuboid(thickness, wall_y_extent, side_length)
            .translation(vector![side_length, 0.0, 0.0])
            .build();
        let wall3 = ColliderBuilder::cuboid(side_length, wall_y_extent, thickness)
            .translation(vector![0.0, 0.0, -thickness])
            .build();
        let wall4 = ColliderBuilder::cuboid(side_length, wall_y_extent, thickness)
            .translation(vector![0.0, 0.0, side_length])
            .build();
        collider_set.insert(wall1);
        collider_set.insert(wall2);
        collider_set.insert(wall3);
        collider_set.insert(wall4);

        /* bouncing balls. */
        let mut ball_body_handles : Vec<RigidBodyHandle> = Vec::new();
        for ball_translation in ball_translations {
            let rigid_body = RigidBodyBuilder::dynamic()
                    .translation(ball_translation)
                    .build();
            let collider = ColliderBuilder::ball(ball_radius)
                .restitution(0.8)
                .build();
            let ball_body_handle = rigid_body_set.insert(rigid_body);
            collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);
            ball_body_handles.push(ball_body_handle);
        }

        /* Create other structures necessary for the simulation. */
        let gravity = vector![0.0, -9.81, 0.0];
        let integration_parameters = IntegrationParameters { 
            dt: 1.0 / 1000.0, 
            ..Default::default()
        };
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();

        RapierState {
            rigid_body_set,
            collider_set,
            gravity,
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            ball_body_handles,
            ball_radius
        }
    }

    pub fn set_ball_force(&mut self, x: f32, z: f32) { 
        for ball_body_handle in &self.ball_body_handles {
            let ball_body = self.rigid_body_set.get_mut(ball_body_handle.clone()).unwrap();

            ball_body.reset_forces(true);
            ball_body.add_force(vector![x, 0.0, z], true);
        }
    }

    fn ball_translations(&self) -> Vec<Vector<Real>> {
        let mut ball_translations = Vec::new();
        for ball_body_handle in &self.ball_body_handles {
            let ball_body = &self.rigid_body_set[ball_body_handle.clone()];
            ball_translations.push(ball_body.translation().clone());
        }
        ball_translations
    }

    fn ball_radius(&self) -> f32 {
        self.ball_radius
    }

    fn step(&mut self, steps: u32) {
        let physics_hooks = ();
        let event_handler = ();

        for _ in 0..steps {
            self.physics_pipeline.step(
                &self.gravity,
                &self.integration_parameters,
                &mut self.island_manager,
                &mut self.broad_phase,
                &mut self.narrow_phase,
                &mut self.rigid_body_set,
                &mut self.collider_set,
                &mut self.impulse_joint_set,
                &mut self.multibody_joint_set,
                &mut self.ccd_solver,
                &physics_hooks,
                &event_handler,
            );
        }

        // console_log!("completed {} steps", steps);
    }
}

#[wasm_bindgen]
pub struct Simulation {
    state: RapierState,
    view: View,
    scene: Scene,
    balls: Vec<Ball>
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct View {
    side_length: f32
}

#[wasm_bindgen]
impl View {
    #[wasm_bindgen(constructor)]
    pub fn new(side_length: f32) -> View {
        View { side_length }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Ball {
    x: f32,
    y: f32
}

#[wasm_bindgen]
impl Ball {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Ball {
        Ball { x, y }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f32 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f32 {
        self.y
    }

    fn as_point2(&self) -> Point2<Real> {
        Point2::new(self.x, self.y)
    }
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(num_balls: u8, terrain: &Terrain, view: &View) -> Simulation {
        let scene = Scene {
            arena_side_length: 50.0
        };
        console_log!("Creating Simulation, with num_balls {:?}, using view {:?}, terrain of {}x{}, and scene: {:?}", 
            num_balls, view, terrain.width, terrain.height, scene);
        let default_y = 10.0;
        let balls = Self::random_balls(num_balls, &view);
        let scene_balls : Vec<Vector<Real>> = balls
            .iter()
            .map(|ball| scene.map_view_to_arena(&view, ball.as_point2(), default_y))
            .collect();
        let state = RapierState::new(scene_balls, terrain, &scene);
        Simulation { state, view: view.clone(), scene, balls }
    }

    fn random_balls(num_balls: u8, view: &View) -> Vec<Ball> {
        use js_sys::Math::random;
        let mut balls = Vec::new();
        for _ in 0 .. num_balls {
            balls.push(Ball {
                x: view.side_length * (random() as f32),
                y: view.side_length * (random() as f32)
            });
        }
        balls
    }

    pub fn set_force(&mut self, x: f32, y: f32) { 
        self.state.set_ball_force(x, y);
    }

    pub fn iter_ball_positions(&self, iter_fn: &js_sys::Function) {
        let scene_ball_radius = self.state.ball_radius();
        let p = self.scene.map_arena_to_view(&self.view, vector![scene_ball_radius, scene_ball_radius, scene_ball_radius]);
        let ball_radius = p.x;
        for ball in &self.balls {
            let this = JsValue::null();
            let _ = iter_fn.call3(&this, 
                &JsValue::from(ball.x), 
                &JsValue::from(ball.y), 
                &JsValue::from(ball_radius));
        }
    }

    pub fn update(&mut self, elapsed_since_last_update: u32) { 
        self.state.step(elapsed_since_last_update);
        let ball_scene_translations = self.state.ball_translations();
        for i in 0 .. ball_scene_translations.len() {
            let ball_scene_translation = ball_scene_translations[i];
            // console_log!("Ball position: {}", ball_scene_translation);
            let ball_position = self.scene.map_arena_to_view(&self.view, ball_scene_translation.clone());
            let mut ball = &mut self.balls[i];
            ball.x = ball_position.x;
            ball.y = ball_position.y;
        }
    }   
}

#[cfg(test)]
mod mapping_tests {
    use wasm_bindgen_test::*;
    use super::*;

    struct Context {
        scene: Scene,
        view: View,
        mappings: Vec<(Point2<Real>, Vector<Real>)>,
        default_y: Real
    }

    fn context() -> Context {
        let scene = Scene {
            arena_side_length: 10.0
        };
        let view = View {
            side_length: 100.0
        };
        let default_y = 0.123;
        let mappings = vec![
            (Point2::new(20.0, 20.0), vector![2.0, default_y, 8.0]),
            (Point2::new(50.0, 50.0), vector![5.0, default_y, 5.0]),
            (Point2::new(80.0, 80.0), vector![8.0, default_y, 2.0])
        ];
        Context {
            scene, view, mappings, default_y
        }
    }

    #[wasm_bindgen_test]
    fn test_map_view_to_arena() {
        let context = context();
        for mapping in &context.mappings {
            let (input, expected) = mapping;
            let actual 
                = context.scene.map_view_to_arena(&context.view, *input, context.default_y);
            assert_eq!(*expected, actual);
        }
    }

    #[wasm_bindgen_test]
    fn test_map_arena_to_view() {
        let context = context();
        for mapping in &context.mappings {
            let (expected, input) = mapping;
            let actual = context.scene.map_arena_to_view(&context.view, *input);
            assert_eq!(*expected, actual);
        }
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