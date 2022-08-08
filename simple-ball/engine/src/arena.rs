
use rapier3d::prelude::*;
use web_sys::Performance;

use crate::log::*;
use crate::terrain::*;
use crate::dimension::*;


pub struct RapierState {
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


pub struct Arena {
    pub dimension: Dimension,
    pub physics: RapierState
}

impl Arena {
    pub fn new(side_length: f32, num_balls: u8, terrain: &Terrain) -> Arena {
        let default_y = 100.0;
        let balls = Self::random_balls(num_balls, side_length, default_y);
        let ball_radius = 0.01 * side_length;
        let physics = RapierState::new(balls, ball_radius, side_length, terrain);
        Arena {
            dimension: Dimension { side_length },
            physics
        }
    }

    fn random_balls(num_balls: u8, side_length: f32, y: Real) -> Vec<Vector<Real>> {
        use js_sys::Math::random;
        let mut balls = Vec::new();
        for _ in 0 .. num_balls {
            balls.push(vector![
                side_length * (random() as f32),
                y,
                side_length * (random() as f32)
            ]);
        }
        balls
    }
}

impl RapierState {
    fn new(ball_translations: Vec<Vector<Real>>, ball_radius: Real, side_length: f32, terrain: &Terrain) -> RapierState {

        console_log!("Creating RapierState");

        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let thickness = 0.1;

        /* heightfield as ground */
        let height_y_extent = ball_radius * 2.0 * 2.0;
        let ground_size 
            = Vector::new(side_length, height_y_extent, side_length);
        let heights 
            = terrain.as_xz_heightfield(1.0);
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
                    .ccd_enabled(true)
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
        let browser_refreshes_per_second = 60.0;
        let integration_parameters = IntegrationParameters { 
            dt: 1.0 / browser_refreshes_per_second, 
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
        let default_y = (x.abs() + z.abs()) / 2.0;
        for ball_body_handle in &self.ball_body_handles {
            let ball_body = self.rigid_body_set.get_mut(ball_body_handle.clone()).unwrap();

            ball_body.reset_forces(true);
            // ball_body.enable_ccd(true);
            // console_log!("{}", ball_body.is_ccd_active());
            ball_body.add_force(vector![x, default_y, z], true);
        }
    }

    pub fn ball_translations(&self) -> Vec<Vector<Real>> {
        let mut ball_translations = Vec::new();
        for ball_body_handle in &self.ball_body_handles {
            let ball_body = &self.rigid_body_set[ball_body_handle.clone()];
            ball_translations.push(ball_body.translation().clone());
        }
        ball_translations
    }

    pub fn ball_radius(&self) -> f32 {
        self.ball_radius
    }

    pub fn step(&mut self, _steps: u32, _performance: &Performance, _max_milliseconds: u32) {
        let physics_hooks = ();
        let event_handler = ();

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
}
