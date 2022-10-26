use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;

mod actor;
mod player;
mod scene;

use bevy::prelude::PluginGroup;

use crate::actor::ActorSystem;
use crate::player::PlayerController;
use crate::scene::Scene;

pub struct Dopamina;

impl PluginGroup for Dopamina {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(Scene).add(ActorSystem).add(PlayerController);
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
