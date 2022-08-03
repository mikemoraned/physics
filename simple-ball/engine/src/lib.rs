use wasm_bindgen::prelude::*;
use rapier3d::prelude::*;

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
pub struct Simulation {
    state: RapierState
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

#[wasm_bindgen]
impl RapierState {
    fn new(ball_x: f32, ball_z: f32, ball_radius: f32) -> RapierState {
        console_log!("Creating RapierState");

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        /* Create the ground. */
        let collider = ColliderBuilder::cuboid(100.0, 0.1, 100.0).build();
        collider_set.insert(collider);

        /* Create the bouncing ball. */
        let rigid_body = RigidBodyBuilder::dynamic()
                .translation(vector![ball_x, 0.0, ball_z])
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
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(ball_x: f32, ball_y: f32, ball_radius: f32) -> Simulation {
        console_log!("Creating Simulation, with ball at {}, {} with radius {}", ball_x, ball_y, ball_radius);
        let state = RapierState::new(ball_x, ball_y, ball_radius);
        Simulation { state }
    }

    pub fn set_force(&mut self, x: f32, y: f32) { 
        self.state.set_ball_force(x, y);
    }

    pub fn update(&mut self, elapsed_since_last_update: u32, update_fn: &js_sys::Function) { 
        self.state.step(elapsed_since_last_update);
        let ball_position = self.state.ball_position();
        // console_log!("Ball position: {}", ball_position);

        let this = JsValue::null();
        let _ = update_fn.call2(&this, 
            &JsValue::from(ball_position.x), 
            &JsValue::from(ball_position.z));
    }   
}