use bevy::{prelude::*, reflect::Uuid};
use bevy_rapier3d::prelude::*;

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

    // Object data
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    transform: Transform,

    collider: Collider,
    rigidbody: RigidBody,
}

impl CharacterActor {
    pub fn new(
        name: &str,
        spawn_location: Transform,
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
        collider: Collider,
    ) -> Self {
        Self {
            id: Identifier(Uuid::new_v4()),

            name: CharName(name.to_string()),
            stats: CharacterStats::new(),

            transform: spawn_location,
            mesh,
            material,

            rigidbody: RigidBody::Dynamic,
            collider,
        }
    }
}
