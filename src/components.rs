use specs::prelude::*;
use rltk::{RGB,Point};

#[derive(Component, Debug, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles : Vec<Point>,
    pub range : i32,
    pub dirty : bool,
}

#[derive(Component, Debug)]
pub struct Character {
    pub strength: u32,
    pub dexterity: u32,
    pub willpower: u32,
    pub intelligence: u32,
    pub charisma: u32,
    pub level: u32,
    pub xp: u32,
    pub life : u32,
}

impl Default for Character {
    fn default() -> Self { 
        Character {
            strength: 8,
            dexterity: 8,
            willpower: 8,
            intelligence: 8,
            charisma: 8,
            level: 1,
            xp: 0,
            life: 10
        }
    }
}

#[derive(Component, Debug)]
pub struct Item {
}

#[derive(Component, Debug)]
pub struct Weapon {
    pub damage_min: i32,
    pub damage_max: i32,
}

#[derive(Component, Debug)]
pub struct Named {
    pub name: String,
}


#[derive(Component, Debug)]
pub struct Keyed {
    pub key: String,
}

#[derive(Component, Debug)]
pub struct Equipped {}

#[derive(Component, Debug)]
pub struct WantToPickup {

}

#[derive(Component, Debug)]
pub struct WantToDrop {
}
