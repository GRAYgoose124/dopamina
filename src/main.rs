use bevy::prelude::*;

mod actor;

use crate::actor::prelude::*;




fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ActorPlugin)
        .add_startup_system(setup)
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    _asset_server: Res<AssetServer>,   
) {
    // ground plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(shape::Plane { size: 50. }.into()),
        material: materials.add(Color::SILVER.into()),
        ..default()
    });

    // Spawn a camera looking at the entities to show what's happening in this example.
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Add a light source for better 3d visibility.
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::splat(3.0)),
        ..default()
    });
}