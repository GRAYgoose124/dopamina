use bevy::{app::PluginGroupBuilder, prelude::*};

use self::{
    manager::CharacterManager,
    prelude::{NPCController, PlayerController},
};

mod actor;
mod manager;
mod npc;
mod player;
mod stats;

pub mod prelude {
    pub use crate::character::npc::NPCController;
    pub use crate::character::player::PlayerController;
    pub use crate::character::stats::CharacterStats;
}

pub struct Characters;

impl PluginGroup for Characters {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(CharacterManager)
            .add(PlayerController)
            .add(NPCController);
    }
}
