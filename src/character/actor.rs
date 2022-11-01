use bevy::{prelude::*, reflect::Uuid};

use super::prelude::CharacterStats;

#[derive(Component, Debug)]
pub struct CharName(pub String);

#[derive(Component, Debug)]
pub struct Identifier(pub Uuid);

#[derive(Component, Bundle)]
pub struct CharacterActor {
    // Unique ID
    pub id: Identifier,

    // Character data
    pub name: CharName,
    #[bundle]
    pub stats: CharacterStats,
}

impl CharacterActor {
    pub fn new(
        name: &str,
    ) -> Self {
        Self {
            id: Identifier(Uuid::new_v4()),

            name: CharName(name.to_string()),
            stats: CharacterStats::new(),
        }
    }
}
