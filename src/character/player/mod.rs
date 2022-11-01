use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier_cam::{FlyCam, NoCameraPlayerPlugin};

use crate::character::actor::CharacterActor;

#[derive(Component)]
struct Player;

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let spawn_location = Transform::from_xyz(0.0, 10.0, 0.0);

    let mesh = Mesh::from(shape::Capsule {
        radius: 0.5,
        ..Default::default()
    });
    let mat = Color::rgb(0.8, 0.7, 0.6).into();

    commands
        .spawn_bundle(SpatialBundle {
            transform: spawn_location,
            visibility: Visibility {
                is_visible: true,
            },
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(mat),
                ..Default::default()
            });
          
            parent.spawn_bundle(CharacterActor::new(
                "Player",
            )).insert(Player);
            parent.spawn_bundle(Camera3dBundle {
                ..default()
            })
            // TODO: Flycam doesn't interact with rapier.
            .insert(FlyCam);            
        })   
        .insert(RigidBody::Dynamic)
        .insert(Collider::capsule_y(0.5, 0.5));
       
}

pub struct PlayerController;

impl Plugin for PlayerController {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_plugin(NoCameraPlayerPlugin);
    }
}
