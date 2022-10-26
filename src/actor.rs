use bevy::{ecs::query::WorldQuery, prelude::*};

#[derive(Component, Debug)]
struct Name(String);
#[derive(Component, Debug)]
struct IsPlayer(bool);

#[derive(Component, Debug)]
struct Health(f32);
#[derive(Component, Debug)]
struct Mana(f32);

#[derive(Component, Debug)]
struct Attack(f32);
#[derive(Component, Debug)]
struct Defense(f32);
#[derive(Component, Debug)]
struct Speed(f32);

#[derive(Component, Debug)]
struct Strength(f32);
#[derive(Component, Debug)]
struct Agility(f32);
#[derive(Component, Debug)]
struct Intelligence(f32);
#[derive(Component, Debug)]
struct Wisdom(f32);
#[derive(Component, Debug)]
struct Charisma(f32);

#[derive(Bundle, Debug)]
pub struct ActorStatBundle {
    name: Name,
    is_player: IsPlayer,

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

impl ActorStatBundle {
    fn new(name: String, is_player: bool) -> Self {
        Self {
            name: Name(name),
            is_player: IsPlayer(is_player),

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

impl Default for ActorStatBundle {
    fn default() -> Self {
        Self::new("".to_string(), false)
    }
}

#[derive(Bundle, Debug)]
pub struct ActorObjectBundle {
    transform: Transform,
    global_transform: GlobalTransform,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl ActorObjectBundle {
    pub fn new(
        transform: Transform,
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
    ) -> Self {
        Self {
            transform,
            global_transform: GlobalTransform::default(),
            mesh,
            material,
        }
    }
}

#[derive(Bundle)]
pub struct ActorBundle {
    // 3D object data
    #[bundle]
    object: ActorObjectBundle,

    // Character data
    #[bundle]
    stats: ActorStatBundle,
}

impl ActorBundle {
    pub fn new(
        transform: Transform,
        mesh: Handle<Mesh>,
        material: Handle<StandardMaterial>,
        name: String,
        is_player: bool,
    ) -> Self {
        Self {
            object: ActorObjectBundle::new(transform, mesh, material),
            stats: ActorStatBundle::new(name, is_player),
        }
    }
}

// TODO: maybe make this pub.
/// Simple Query which provides an iterator over all actors, letting you access them publically.
#[derive(WorldQuery)]
pub struct ActorQuery<'a> {
    name: &'a Name,
    transform: &'a Transform,
}

fn spawn_actors(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    let mat = materials.add(Color::rgb(0.8, 0.7, 0.6).into());

    for i in 0..100 {
        // spawn actor spheres
        commands.spawn_bundle(ActorBundle::new(
            Transform::from_translation(Vec3::new(i as f32, 0.0, 0.0)),
            mesh.clone(),
            mat.clone(),
            format!("Actor {}", i),
            false,
        ));
    }
}

fn iterate_actors(query: Query<ActorQuery>) {
    for _actor in query.iter() {}
}

pub struct ActorSystem;

impl Plugin for ActorSystem {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_actors)
            .add_system(iterate_actors);
    }
}
