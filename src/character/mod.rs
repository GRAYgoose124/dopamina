use bevy::{ecs::query::WorldQuery, prelude::*, time::FixedTimestep};

use self::physics_actor::PhysicsActor;

mod physics_actor;
mod player;

pub mod prelude {
    pub use crate::character::player::PlayerController;
    pub use crate::character::CharacterManager;
}

#[derive(Component, Debug)]
pub struct Health(f32);
#[derive(Component, Debug)]
pub struct Mana(f32);

#[derive(Component, Debug)]
pub struct Attack(f32);
#[derive(Component, Debug)]
pub struct Defense(f32);
#[derive(Component, Debug)]
pub struct Speed(f32);

#[derive(Component, Debug)]
pub struct Strength(f32);
#[derive(Component, Debug)]
pub struct Agility(f32);
#[derive(Component, Debug)]
pub struct Intelligence(f32);
#[derive(Component, Debug)]
pub struct Wisdom(f32);
#[derive(Component, Debug)]
pub struct Charisma(f32);

#[derive(Component, Bundle, Debug)]
pub struct CharacterStats {
    health: Health,
    mana: Mana,

    attack: Attack,
    defense: Defense,
    speed: Speed,

    strength: Strength,
    agility: Agility,
    intelligence: Intelligence,
    wisdom: Wisdom,
    charisma: Charisma,
}

impl CharacterStats {
    fn new() -> Self {
        Self {
            health: Health(100.0),
            mana: Mana(100.0),

            attack: Attack(1.0),
            defense: Defense(1.0),
            speed: Speed(1.0),

            strength: Strength(1.0),
            agility: Agility(1.0),
            intelligence: Intelligence(1.0),
            wisdom: Wisdom(1.0),
            charisma: Charisma(1.0),
        }
    }
}

impl Default for CharacterStats {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Component, Debug)]
pub struct Name(String);
#[derive(Component, Debug)]
pub struct IsPlayer(bool);

#[derive(Component, Bundle)]
pub struct CharacterActor {
    name: Name,
    is_player: IsPlayer,

    // 3D object data
    #[bundle]
    object: PhysicsActor,

    // Character data
    #[bundle]
    stats: CharacterStats,
}

impl CharacterActor {
    pub fn new(
        name: &str,
        is_player: bool,
        spawn_location: Transform,
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
    ) -> Self {
        Self {
            name: Name(name.to_string()),
            is_player: IsPlayer(is_player),
            object: PhysicsActor::new(spawn_location, mesh, material),
            stats: CharacterStats::new(),
        }
    }
}

#[derive(WorldQuery)]
pub struct ActorQuery<'a> {
    name: &'a Name,
    stats: &'a CharacterStats,
    actor: &'a PhysicsActor,
}

fn _iterate_characters(query: Query<ActorQuery>) {
    for character in query.iter() {
        let name = &character.name.0;

        println!("Character: {:?}", name);
    }
}

fn spawn_actors(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 10.0 }));
    let mat = materials.add(Color::rgb(1.0, 0.0, 0.0).into());

    for i in 0..1000 {
        commands.spawn_bundle(CharacterActor {
            name: Name(format!("Character {}", i)),
            is_player: IsPlayer(false),
            object: PhysicsActor::new(
                Transform::from_translation(Vec3::new((i % 10) as f32, (i + 1 % 10) as f32, (i + 2 % 10) as f32)),
                mesh.clone(),
                mat.clone(),
            ),
            stats: CharacterStats::new(),
        });
    }
}

pub struct CharacterManager;

impl Plugin for CharacterManager {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_actors)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(physics_actor::TIME_STEP))
                .with_system(physics_actor::update_physics),
        );
    }
}
