use std::ops::Add;

use bevy::prelude::*;

pub const TIME_STEP: f64 = 1.0 / 1.0;
const GRAVITY: Vec3 = Vec3::new(0.0, -1.0, 0.0);
const DRAG: Vec3 = Vec3::new(-0.01, -0.01, -0.01);

#[derive(Component, Debug, Default)]
pub struct Velocity(Vec3);
#[derive(Component, Debug, Default)]
pub struct Acceleration(Vec3);

#[derive(Component, Bundle, Debug)]
pub struct PhysicsActor {
    transform: Transform,
    velocity: Velocity,
    acceleration: Acceleration,
    global_transform: GlobalTransform,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl PhysicsActor {
    pub fn new(
        transform: Transform,
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
    ) -> Self {
        Self {
            transform,
            velocity: Velocity::default(),
            acceleration: Acceleration::default(),
            global_transform: GlobalTransform::default(),
            mesh,
            material,
        }
    }
}

// TODO: maybe make this pub.
/// Simple Query which provides an iterator over all actors, letting you access them publically.
pub fn update_physics(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Acceleration, &mut Transform)>,
) {
    let delta = time.delta_seconds();
    for (mut velocity, mut acceleration, mut transform) in query.iter_mut() {
        transform.translation = transform.translation.add(velocity.0) * delta;

        velocity.0 = velocity.0.add(acceleration.0) * delta;
        velocity.0 = velocity.0.add(GRAVITY) * delta;

        acceleration.0 = acceleration.0.add(DRAG) * delta;
    }
}
