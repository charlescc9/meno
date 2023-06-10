use super::particle;
use rapier2d::prelude::*;

pub struct Simulation {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub rigid_body_handles: Vec<RigidBodyHandle>,
    pub physics_pipeline: PhysicsPipeline,
    pub gravity: f32,
    pub integration_parameters: IntegrationParameters,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
}

impl Simulation {
    pub fn new(particles: &Vec<particle::Particle>, particle_radius: f32) -> Self {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let mut rigid_body_handles = Vec::new();

        // Make sides of window colliders
        let top = ColliderBuilder::cuboid(1.0, 1.0)
            .translation(vector![0.0, 2.0])
            .build();
        collider_set.insert(top);
        let right = ColliderBuilder::cuboid(1.0, 1.0)
            .translation(vector![2.0, 0.0])
            .build();
        collider_set.insert(right);
        let bottom = ColliderBuilder::cuboid(1.0, 1.0)
            .translation(vector![0.0, -2.0])
            .build();
        collider_set.insert(bottom);
        let left = ColliderBuilder::cuboid(1.0, 1.0)
            .translation(vector![-2.0, 0.0])
            .build();
        collider_set.insert(left);

        for particle in particles {
            let rigid_body = RigidBodyBuilder::dynamic()
                .translation(vector![particle.position[0], particle.position[1]])
                .linvel(vector![particle.velocity[0], particle.velocity[1]])
                .build();
            let rigid_body_handle = rigid_body_set.insert(rigid_body);
            let collider = ColliderBuilder::ball(particle_radius)
                .restitution(1.0)
                .build();
            collider_set.insert_with_parent(collider, rigid_body_handle, &mut rigid_body_set);
            rigid_body_handles.push(rigid_body_handle);
        }

        Self {
            rigid_body_set,
            collider_set,
            rigid_body_handles,
            physics_pipeline: PhysicsPipeline::new(),
            gravity: 0.0,
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
        }
    }
}