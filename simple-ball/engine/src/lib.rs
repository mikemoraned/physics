use wasm_bindgen::prelude::*;
use js_sys::Math::random;
use std::f64::consts::PI;
use rapier2d::prelude::*;

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
pub struct Engine {
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
}

#[wasm_bindgen]
impl RapierState {
    fn new() -> RapierState {
        console_log!("Creating RapierState");

        let rigid_body_set = RigidBodySet::new();
        let collider_set = ColliderSet::new();

        let gravity = vector![0.0, -9.81];
        let integration_parameters = IntegrationParameters::default();
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
            ccd_solver
        }
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

        console_log!("completed {} steps", steps);
    }
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Engine {
        console_log!("Creating Engine");
        let state = RapierState::new();
        Engine { state }
    }

    pub fn update(&mut self, elapsed_since_last_update: u32, x: u32, y: u32, update_fn: &js_sys::Function) { 
        self.state.step(elapsed_since_last_update);

        let speed = 0.3f64; // pixels per millisecond
        let distance = speed * (elapsed_since_last_update as f64);
        console_log!("e: {}, speed: {}, distance: {}", elapsed_since_last_update, speed, distance);
        let angle = 2.0 * PI * random();
        let x_change = (angle.cos() * distance) as i32;
        let y_change = (angle.sin() * distance) as i32;

        console_log!("e: {}, angle: {}, x_change: {}, y_change: {}", 
            elapsed_since_last_update,
            angle,
            x_change,
            y_change);

        let new_x = (x as i32 + x_change) as u32;
        let new_y = (y as i32 + y_change) as u32;
        console_log!("x: {} + {} = {}", x, x_change, new_x);
        console_log!("y: {} + {} = {}", y, y_change, new_y);

        let this = JsValue::null();
        let _ = update_fn.call2(&this, 
            &JsValue::from(new_x), 
            &JsValue::from(new_y));
    }   
}