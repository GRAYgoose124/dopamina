use bevy::prelude::*;

mod character;
mod scene;

use crate::scene::Scene;
use bevy_flycam::MovementSettings;
use character::Characters;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
        sensitivity: 0.1,
        ..Default::default()
    });
}
