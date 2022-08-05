use wasm_bindgen::prelude::*;
use rapier3d::prelude::*;
use nalgebra::Point2;

mod log;
mod terrain;

use log::*;
use terrain::*;

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
struct Arena {
    side_length: f32
}

impl Arena {
    fn map_screen_to_arena(&self, screen: &Screen, point: Point2<Real>, default_y: Real) -> Vector<Real> {
        let scale = self.side_length / screen.side_length;
        let x = point.x * scale;
        let z = self.side_length - (point.y * scale);
        vector![x, default_y, z]
    }

    fn map_arena_to_screen(&self, screen: &Screen, vector: Vector<Real>) -> Point2<Real> {
        let scale = screen.side_length / self.side_length;
        let x = vector.x * scale;
        let y = screen.side_length - (vector.z * scale);
        Point2::new(x, y)
    }
}

#[wasm_bindgen]
impl RapierState {
    fn new(ball_translations: Vec<Vector<Real>>, terrain: &Terrain, arena: &Arena) -> RapierState {

        console_log!("Creating RapierState");

        let ball_radius = 0.01 * arena.side_length;

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let side_length = arena.side_length;
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
    screen: Screen,
    arena: Arena,
    balls: Vec<Ball>
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Screen {
    side_length: f32
}

#[wasm_bindgen]
impl Screen {
    #[wasm_bindgen(constructor)]
    pub fn new(side_length: f32) -> Screen {
        Screen { side_length }
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
    pub fn new(num_balls: u8, terrain: &Terrain, screen: &Screen) -> Simulation {
        let arena = Arena {
            side_length: 50.0
        };
        console_log!("Creating Simulation, with num_balls {:?}, using screen {:?}, terrain of {}x{}, and arena: {:?}", 
            num_balls, screen, terrain.width, terrain.height, arena);
        let default_y = 10.0;
        let balls = Self::random_balls(num_balls, &screen);
        let arena_balls : Vec<Vector<Real>> = balls
            .iter()
            .map(|ball| arena.map_screen_to_arena(&screen, ball.as_point2(), default_y))
            .collect();
        let state = RapierState::new(arena_balls, terrain, &arena);
        Simulation { state, screen: screen.clone(), arena, balls }
    }

    fn random_balls(num_balls: u8, screen: &Screen) -> Vec<Ball> {
        use js_sys::Math::random;
        let mut balls = Vec::new();
        for _ in 0 .. num_balls {
            balls.push(Ball {
                x: screen.side_length * (random() as f32),
                y: screen.side_length * (random() as f32)
            });
        }
        balls
    }

    pub fn set_force(&mut self, x: f32, y: f32) { 
        self.state.set_ball_force(x, y);
    }

    pub fn iter_ball_positions(&self, iter_fn: &js_sys::Function) {
        let arena_ball_radius = self.state.ball_radius();
        let p = self.arena.map_arena_to_screen(&self.screen, vector![arena_ball_radius, arena_ball_radius, arena_ball_radius]);
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
        let ball_arena_translations = self.state.ball_translations();
        for i in 0 .. ball_arena_translations.len() {
            let ball_arena_translation = ball_arena_translations[i];
            // console_log!("Ball position: {}", ball_arena_translation);
            let ball_position = self.arena.map_arena_to_screen(&self.screen, ball_arena_translation.clone());
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
        arena: Arena,
        screen: Screen,
        mappings: Vec<(Point2<Real>, Vector<Real>)>,
        default_y: Real
    }

    fn context() -> Context {
        let arena = Arena {
            side_length: 10.0
        };
        let screen = Screen {
            side_length: 100.0
        };
        let default_y = 0.123;
        let mappings = vec![
            (Point2::new(20.0, 20.0), vector![2.0, default_y, 8.0]),
            (Point2::new(50.0, 50.0), vector![5.0, default_y, 5.0]),
            (Point2::new(80.0, 80.0), vector![8.0, default_y, 2.0])
        ];
        Context {
            arena, screen, mappings, default_y
        }
    }

    #[wasm_bindgen_test]
    fn test_map_screen_to_arena() {
        let context = context();
        for mapping in &context.mappings {
            let (input, expected) = mapping;
            let actual 
                = context.arena.map_screen_to_arena(&context.screen, *input, context.default_y);
            assert_eq!(*expected, actual);
        }
    }

    #[wasm_bindgen_test]
    fn test_map_arena_to_screen() {
        let context = context();
        for mapping in &context.mappings {
            let (expected, input) = mapping;
            let actual = context.arena.map_arena_to_screen(&context.screen, *input);
            assert_eq!(*expected, actual);
        }
    }
}

