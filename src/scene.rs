use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};

pub const TIME_STEP: f64 = 1.0 / 60.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    // ground plane
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            mesh: meshes.add(shape::Plane { size: 200. }.into()),
            material: materials.add(Color::SEA_GREEN.into()),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(100.0, 0.0, 100.0));

    // light source
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
        ..default()
    });
}

pub struct Scene;

impl Plugin for Scene {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}
