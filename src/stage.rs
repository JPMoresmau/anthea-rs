use super::{Rect};
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Stage {
    pub name: String,
    pub rooms: HashMap<String, Room>,
    pub start: String,
    pub doors: Vec<Door>,
    pub items: HashMap<String,StageItem>,
    pub weapons: Vec<StageWeapon>,
    pub npcs: HashMap<String,StageNPC>,
    pub affordances: HashMap<String,StageAffordance>,
    pub quests: HashMap<String,String>,
    pub spells: HashMap<String,Spell>,
    pub potions: HashMap<String,StagePotion>,
}

#[derive(Debug, Deserialize)]
pub struct Room {
    pub dimensions: Rect,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Door {
    pub room1: String,
    pub room2: String,
    pub width: usize,
}

#[derive(Debug, Deserialize)]
pub struct StageItem {
    pub name: String,
    pub position: (i32, i32),
}

#[derive(Debug, Deserialize)]
pub struct StageWeapon {
    pub name: String,
    pub position: (i32, i32),
    pub damage: (i32, i32),
}

#[derive(Debug, Deserialize)]
pub struct StageNPC {
    pub name: String,
    pub position: (i32, i32),
    pub interactions: Vec<Interaction>,
}

#[derive(Debug, Deserialize)]
pub struct StageAffordance {
    pub name: String,
    pub position: (i32, i32),
    pub interactions: Vec<Interaction>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum InteractionType {
    Automatic,
    Question,
}

impl Default for InteractionType {
    fn default() -> Self {
        InteractionType::Automatic
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Interaction {
    #[serde(default)]
    pub conditions: Vec<Condition>,
    pub text: String,
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(default)]
    pub interaction_type: InteractionType,
    #[serde(default)]
    pub after_text: String,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Condition {
    IfFlag(String,String),
    IfItem(String),
    IfQuestAchieved(String),
}

#[derive(Debug, Deserialize, Clone)]
pub enum Action {
    SetFlag(String,String),
    RemoveFlag(String,String),
    AddDiary(String,String),
    UseItem(String),
    RaiseXP(u32),
    UpdateCharacter(String, i32),
    StartQuest(String),
    CompleteQuest(String, u32),
    LearnSpell(String, u32),
    PickupPotion(String),
    AddDoor(String,String,usize),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Spell {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StagePotion {
    pub name: String,
    pub effects: Vec<Effect>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Effect {
    pub characteristic: String,
    pub diff: i32,
}