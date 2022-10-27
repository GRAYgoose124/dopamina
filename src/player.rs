use bevy::{prelude::*};
use leafwing_input_manager::prelude::*;

use crate::actor::CharacterActor;

pub mod prelude {
    pub use crate::player::PlayerController;
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,

    Run,
    Jump,

    Interact,
}

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq)]
enum CameraMotion {
    Look,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerCamera {
    pitch: f32,
    yaw: f32    
}

fn spawn_player(mut commands: Commands) {
    let spawn_location = Transform::from_xyz(50.0, 10.0, 50.0);

    commands
        .spawn()
        .insert(Player)
        .insert_bundle(CharacterActor::new(
            spawn_location,
            Handle::default(),
            Handle::default(),
            "Player".to_string(),
            true,
        ))
        .insert_bundle(Camera3dBundle {
            transform: spawn_location.looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert_bundle(InputManagerBundle::<Action> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (KeyCode::LShift, Action::Run),
                (KeyCode::Space, Action::Jump),
                (KeyCode::W, Action::MoveForward),
                (KeyCode::S, Action::MoveBackward),
                (KeyCode::A, Action::MoveLeft),
                (KeyCode::D, Action::MoveRight),
                (KeyCode::E, Action::Interact),
            ])

        })
        .insert_bundle(InputManagerBundle::<CameraMotion> {
            input_map: InputMap::default()
            .insert(DualAxis::mouse_motion(), CameraMotion::Look)
            .build(),
            ..default()
        });
}

fn player_camera_handler(mut query: Query<(&mut Transform, &ActionState<CameraMotion>), With<Camera3d>>) {
    let (mut transform, action_state) = query.single_mut();
    
    println!("{:?}", action_state);

    // Get the mouse motion
    let mouse_motion = if let Some(mouse_motion) = action_state.axis_pair(CameraMotion::Look) {
        eprintln!("Mouse motion: {:?}", mouse_motion);
        mouse_motion
    } else {
        return;
    };

    
    // Turn the 2d mouse motion into a 3d rotation
    let rotation = Quat::from_rotation_x(mouse_motion.y()) * Quat::from_rotation_z(mouse_motion.x());
    println!("{:?}", rotation);

    // Rotate the camera by the rotation we just calculated.
    transform.rotate(rotation);
}

fn player_input_handler(query: Query<&ActionState<Action>, With<Player>>) {
    let action_state = query.single();

    if action_state.just_pressed(Action::Jump) {
        println!("Jump!");
    } else if action_state.just_pressed(Action::MoveForward) {
        println!("Move forward!");
    } else if action_state.just_pressed(Action::MoveBackward) {
        println!("Move backward!");
    } else if action_state.just_pressed(Action::MoveLeft) {
        println!("Move left!");
    } else if action_state.just_pressed(Action::MoveRight) {
        println!("Move right!");
    } else if action_state.just_pressed(Action::Run) {
        println!("Run!");
    } else if action_state.just_pressed(Action::Interact) {
        println!("Interact!");
    }
}

pub struct PlayerController;

impl Plugin for PlayerController {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .add_startup_system(spawn_player)
            .add_system(player_input_handler)
            .add_system(player_camera_handler);
    }
}
