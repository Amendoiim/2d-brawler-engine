//! 2D Physics system

use anyhow::Result;
use rapier2d::prelude::*;

/// 2D Physics world
pub struct PhysicsWorld {
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    integration_parameters: IntegrationParameters,
    gravity: Vector<f32>,
}

impl PhysicsWorld {
    /// Create a new physics world
    pub fn new() -> Result<Self> {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        
        // Create ground
        let ground_handle = rigid_body_set.insert(RigidBodyBuilder::fixed());
        let ground_collider = ColliderBuilder::cuboid(100.0, 1.0);
        collider_set.insert_with_parent(ground_collider, ground_handle, &mut rigid_body_set);

        Ok(Self {
            rigid_body_set,
            collider_set,
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            integration_parameters: IntegrationParameters::default(),
            gravity: vector![0.0, -9.81],
        })
    }

    /// Step the physics simulation
    pub fn step(&mut self, dt: f32) {
        let mut context = PhysicsHooks::default();
        let mut event_handler = ();
        
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.ccd_solver,
            &mut self.query_pipeline,
            &mut context,
            &mut event_handler,
        );
    }

    /// Add a rigid body to the world
    pub fn add_rigid_body(&mut self, body: RigidBody) -> RigidBodyHandle {
        self.rigid_body_set.insert(body)
    }

    /// Add a collider to the world
    pub fn add_collider(&mut self, collider: Collider, parent: RigidBodyHandle) -> ColliderHandle {
        self.collider_set.insert_with_parent(collider, parent, &mut self.rigid_body_set)
    }

    /// Get reference to rigid body set
    pub fn rigid_bodies(&self) -> &RigidBodySet {
        &self.rigid_body_set
    }

    /// Get mutable reference to rigid body set
    pub fn rigid_bodies_mut(&mut self) -> &mut RigidBodySet {
        &mut self.rigid_body_set
    }

    /// Get reference to collider set
    pub fn colliders(&self) -> &ColliderSet {
        &self.collider_set
    }

    /// Get mutable reference to collider set
    pub fn colliders_mut(&mut self) -> &mut ColliderSet {
        &mut self.collider_set
    }
}
