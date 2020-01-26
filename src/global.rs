use std::collections::HashMap;
use serde::Deserialize;

use super::{Character};

#[derive(Debug, Deserialize, Clone)]
pub struct Global {
    pub potions: HashMap<String, PotionDef>,
    pub monsters: HashMap<String, MonsterDef>,
    pub spells: HashMap<String, Spell>,
    pub weapons: HashMap<String, WeaponDef>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Spell {
    pub name: String,
    pub description: String,
}


#[derive(Debug, Deserialize, Clone)]
pub struct PotionDef {
    pub name: String,
    pub effects: Vec<Effect>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Effect {
    pub characteristic: String,
    pub diff: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MonsterDef {
    pub name: String,
    pub character: Character,
    pub weapon: WeaponDef,
    #[serde(default)]
    pub attacks: Vec<String>,
    #[serde(default)]
    pub misses: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WeaponDef {
    pub name: String,
    pub damage: (u32, u32),
}