use bevy::prelude::*;
use bevy::app::PluginGroupBuilder;

mod actor;
mod scene;
mod player;

use bevy::prelude::PluginGroup;

use crate::actor::ActorSystem;
use crate::scene::Scene;
use crate::player::PlayerController;

pub struct Dopamina;

impl PluginGroup for Dopamina {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(Scene)
            .add(ActorSystem)
            .add(PlayerController);
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
) {}