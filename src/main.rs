use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

mod character;
mod scene;

use bevy::prelude::PluginGroup;

use crate::character::prelude::*;
use crate::scene::Scene;

pub struct Dopamina;

impl PluginGroup for Dopamina {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(Scene).add(CharacterManager).add(PlayerController);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Dopamina)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut _commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
    _asset_server: Res<AssetServer>,
) {
}
