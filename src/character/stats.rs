use bevy::prelude::{Component, Bundle};

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
pub struct Dexterity(f32);
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
    dexterity: Dexterity,
    intelligence: Intelligence,
    wisdom: Wisdom,
    charisma: Charisma,
}

impl CharacterStats {
    pub fn new() -> Self {
        Self {
            health: Health(100.0),
            mana: Mana(100.0),

            attack: Attack(1.0),
            defense: Defense(1.0),
            speed: Speed(1.0),

            strength: Strength(1.0),
            agility: Agility(1.0),
            dexterity: Dexterity(1.0),
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