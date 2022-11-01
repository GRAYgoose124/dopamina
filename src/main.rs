use bevy::prelude::*;
use bevy_rapier3d::plugin::{RapierPhysicsPlugin, NoUserData};

mod character;
mod scene;

use crate::scene::Scene;
use bevy_rapier_cam::MovementSettings;
use character::Characters;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup)
        .add_plugin(Scene)
        .add_plugins(Characters)
        .run();
}

fn setup(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    commands.insert_resource(MovementSettings {
        speed: 20.0,
        sensitivity: 0.0015,
        ..Default::default()
    });
}
