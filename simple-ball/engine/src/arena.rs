
use rapier3d::prelude::*;

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
        let ball_radius = 0.01 * side_length;
        let balls 
            = Self::random_balls(num_balls, ball_radius, side_length, terrain, default_y);
        let physics = RapierState::new(balls, ball_radius, side_length, terrain);
        Arena {
            dimension: Dimension { side_length },
            physics
        }
    }

    fn random_balls(num_balls: u8, ball_radius: f32, side_length: f32, terrain: &Terrain, y: Real) -> Vec<Vector<Real>> {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        let containing_box_side_length = ball_radius * 2.0;
        let possible_grid_positions_per_axis = (side_length / containing_box_side_length).floor() as u32;
        console_log!("possible_grid_positions_per_axis: {}", possible_grid_positions_per_axis);
        let sized_terrain = terrain.shrink_to_fit(possible_grid_positions_per_axis as usize);
        console_log!("Sized terrain: {}x{}", sized_terrain.width, sized_terrain.height);
        let max_bucket_value = 20.0;
        let heightfield = sized_terrain.as_xz_heightfield(max_bucket_value);
        console_log!("Converted to heightfield, shape: {:?}", heightfield.shape());
        let possible_grid_positions : Vec<(u32, u32)>
            = (0..sized_terrain.height).into_iter().flat_map(|z| {
                let row : Vec<(u32, u32)> 
                    = (0..sized_terrain.width).into_iter().map(|x| {
                        (x as u32, z as u32)
                    }).collect();
                row
            }).collect();
        console_log!("Created possible grid positions");
        let probababilities : Vec<((u32, u32), f64)> 
            = possible_grid_positions.iter().map(|(x, z)| {
                let row = *z as usize;
                let column = *x as usize;
                let index = (row, column);
                let bucketed_height = *heightfield.index(index) as f64;
                let probability = 2.0f64.powf((max_bucket_value as f64) - bucketed_height);
                console_log!("{:?} -> {:?}", bucketed_height, probability);
                ((*x, *z), probability)
            }).collect();
        console_log!("Created probabilities: {:?}", probababilities);
        let mut rng = thread_rng();
        let selected
            = probababilities.choose_multiple_weighted(
                &mut rng, 
                num_balls as usize, 
                |(_point, probability)| *probability).unwrap();
        let x_scale_up = side_length / (sized_terrain.width as f32);
        let z_scale_up = side_length / (sized_terrain.height as f32);
        selected.map(|((x, z), _probability)| {
            vector![
                ((*x as f32) * x_scale_up) + ball_radius, 
                y, 
                ((*z as f32) * z_scale_up) + ball_radius]
        }).collect()
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

    pub fn step(&mut self) {
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
