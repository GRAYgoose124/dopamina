use crate::scene::TIME_STEP;

use super::actor::{CharacterActor, Identifier};
use bevy::{ecs::query::WorldQuery, prelude::*, time::FixedTimestep};
use bevy_rapier3d::prelude::Collider;

#[derive(Component, Debug)]
pub struct NPC;

fn spawn_npcs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let mat = materials.add(Color::rgb(1.0, 0.0, 0.0).into());

    for i in 0..1000 {
        commands
            .spawn_bundle(CharacterActor::new(
                &format!("NPC {}", i),
                Transform::from_xyz(i as f32, 0.0, 0.0),
                mesh.clone(),
                mat.clone(),
                Collider::cuboid(0.5, 0.5, 0.5),
            ))
            .insert(NPC);
    }
}

#[derive(WorldQuery)]
pub struct NPCQuery<'a> {
    entity: Entity, // CharacterActor instead?

    id: &'a Identifier,
    npc: With<NPC>,
}

fn iterate_npcs(query: Query<NPCQuery>) {
    for _character in query.iter() {
        //let name = &character.name.0;
    }
}

pub struct NPCController;

impl Plugin for NPCController {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_npcs).add_system_set(
            // add_system_to_stage
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP))
                .with_system(iterate_npcs),
        );
    }
}
