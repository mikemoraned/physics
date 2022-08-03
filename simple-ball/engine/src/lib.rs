use wasm_bindgen::prelude::*;
use rapier3d::prelude::*;
use nalgebra::Point2;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
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
    ball_body_handle: RigidBodyHandle,
}

#[derive(Debug)]
struct Scene {
    arena_side_length: f32
}

impl Scene {
    fn map_view_to_arena(&self, view: &View, point: Point2<Real>) -> Vector<Real> {
        // arena_side_length = 10, view.side_length = 100
        // 50, 50 -> 5, 5
        let scale = self.arena_side_length / view.side_length;
        vector![point.x * scale, 0.0, point.y * scale]
    }

    fn map_arena_to_view(&self, view: &View, vector: Vector<Real>) -> Point2<Real> {
        // arena_side_length = 10, view.side_length = 100
        // 5, 5 -> 50, 50
        let scale = view.side_length / self.arena_side_length;
        Point2::new(vector.x * scale, vector.z * scale)
    }
}

#[wasm_bindgen]
impl RapierState {
    fn new(ball_translation: Vector<Real>, scene: &Scene) -> RapierState {
        console_log!("Creating RapierState");

        let ball_radius = 0.05 * scene.arena_side_length;

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let side_length = scene.arena_side_length;
        let thickness = 0.1;
        /* ground. */
        let ground = ColliderBuilder::cuboid(side_length, thickness, side_length).build();
        collider_set.insert(ground);
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

        /* bouncing ball. */
        let rigid_body = RigidBodyBuilder::dynamic()
                .translation(ball_translation)
                .build();
        let collider = ColliderBuilder::ball(ball_radius).restitution(0.7).build();
        let ball_body_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

        /* Create other structures necessary for the simulation. */
        let gravity = vector![0.0, -9.81, 0.0];
        // let integration_parameters = IntegrationParameters::default();
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
            ball_body_handle
        }
    }

    pub fn set_ball_force(&mut self, x: f32, z: f32) { 
        let ball_body = self.rigid_body_set.get_mut(self.ball_body_handle).unwrap();

        ball_body.reset_forces(true);
        ball_body.add_force(vector![x, 0.0, z], true);
    }

    fn ball_position(&self) -> &Vector<Real> {
        let ball_body = &self.rigid_body_set[self.ball_body_handle];
        ball_body.translation()
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
    ball: Ball
}

#[wasm_bindgen]
#[derive(Debug)]
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
#[derive(Debug)]
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

    fn as_point2(&self) -> Point2<Real> {
        Point2::new(self.x, self.y)
    }
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(ball: Ball, view: View) -> Simulation {
        let scene = Scene {
            arena_side_length: 100.0
        };
        console_log!("Creating Simulation, with ball {:?}, using view {:?}, and scene: {:?}", ball, view, scene);
        let scene_ball = scene.map_view_to_arena(&view, ball.as_point2());
        let state = RapierState::new(scene_ball, &scene);
        Simulation { state, view, scene, ball }
    }

    pub fn set_force(&mut self, x: f32, y: f32) { 
        self.state.set_ball_force(x, y);
    }

    pub fn update(&mut self, elapsed_since_last_update: u32) { 
        self.state.step(elapsed_since_last_update);
        let ball_scene_position = self.state.ball_position();
        console_log!("Ball position: {}", ball_scene_position);
        let ball_position = self.scene.map_arena_to_view(&self.view, ball_scene_position.clone());
        self.ball.x = ball_position.x;
        self.ball.y = ball_position.y;
    }   
}