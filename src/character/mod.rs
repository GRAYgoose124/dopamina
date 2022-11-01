use bevy::{ecs::query::WorldQuery, prelude::*, prelude::shape};
use bevy_rapier3d::prelude::*;

use self::stats::CharacterStats;

mod player;
mod stats;

pub mod prelude {
    pub use crate::character::player::PlayerController;
    pub use crate::character::CharacterManager;
    pub use crate::character::stats::CharacterStats;
}



#[derive(Component, Debug)]
pub struct Name(String);
#[derive(Component, Debug)]
pub struct NPC;

#[derive(Component, Bundle)]
pub struct CharacterActor {
    name: Name,

    // Character data
    #[bundle]
    stats: CharacterStats,

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
            name: Name(name.to_string()),
            stats: CharacterStats::new(),

            transform: spawn_location,
            mesh,
            material,

            rigidbody: RigidBody::Dynamic,
            collider,
        }
    }
}

#[derive(WorldQuery)]
pub struct ActorQuery<'a> {
    name: &'a Name,
    stats: &'a CharacterStats,
}

fn _iterate_characters(query: Query<ActorQuery>) {
    for character in query.iter() {
        let name = &character.name.0;

        println!("Character: {:?}", name);
    }
}

fn spawn_npcs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let mat = materials.add(Color::rgb(1.0, 0.0, 0.0).into());

    for i in 0..1000 {
        commands.spawn_bundle(CharacterActor::new(
            "Character",
            Transform::from_xyz(i as f32, 0.0, 0.0),
            mesh.clone(),
            mat.clone(),
            Collider::cuboid(0.5, 0.5, 0.5),
        )).insert(NPC);
    }
}

pub struct CharacterManager;

impl Plugin for CharacterManager {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_npcs);
        // .add_system_set(
        //     SystemSet::new()
        //         .with_run_criteria(FixedTimestep::step(physics_actor::TIME_STEP))
        //         .with_system(physics_actor::update_physics),
        // );
    }
}
