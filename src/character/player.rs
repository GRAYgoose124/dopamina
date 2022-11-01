use bevy::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};
use bevy_rapier3d::prelude::*;

use crate::character::actor::CharacterActor;

#[derive(Component)]
struct Player;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let spawn_location = Transform::from_xyz(-2.0, 10.0, 5.0);

    let mesh = Mesh::from(shape::Icosphere {
        radius: 1.0,
        subdivisions: 4,
    });
    let mat = Color::rgb(0.8, 0.7, 0.6).into();

    commands
        .spawn_bundle(CharacterActor::new(
            "Player",
            spawn_location,
            meshes.add(mesh),
            materials.add(mat),
            Collider::capsule_y(0.5, 0.5),
        ))
        .insert_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(FlyCam)
        .insert(Player);
}

pub struct PlayerController;

impl Plugin for PlayerController {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_plugin(NoCameraPlayerPlugin);
    }
}
