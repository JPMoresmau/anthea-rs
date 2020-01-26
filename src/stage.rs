use super::{Rect};
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Stage {
    pub name: String,
    pub rooms: HashMap<String, Room>,
    pub start: String,
    pub doors: Vec<Door>,
    pub items: HashMap<String,StageItem>,
    #[serde(default)]
    pub weapons: Vec<WeaponRef>,
    pub npcs: HashMap<String,StageNPC>,
    pub affordances: HashMap<String,StageAffordance>,
    pub quests: HashMap<String,String>,
    #[serde(default)]
    pub potions: Vec<PotionRef>,
    #[serde(default)]
    pub monsters: Vec<MonsterRef>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Room {
    pub dimensions: Rect,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Door {
    pub room1: String,
    pub room2: String,
    pub width: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StageItem {
    pub name: String,
    #[serde(default)]
    pub position: (i32, i32),
}

#[derive(Debug, Deserialize, Clone)]
pub struct WeaponRef {
    pub key: String,
    pub position: (i32, i32),
}

#[derive(Debug, Deserialize, Clone)]
pub struct PotionRef {
    pub key: String,
    pub position: (i32, i32),
}


#[derive(Debug, Deserialize, Clone)]
pub struct StageNPC {
    pub name: String,
    pub position: (i32, i32),
    pub interactions: Vec<Interaction>,
}

#[derive(Debug, Deserialize, Clone)]
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

impl Interaction {
    pub fn internal(action: Action) -> Interaction{
        Interaction{conditions:Vec::new(), text:String::new(),actions:vec!(action),interaction_type:InteractionType::Automatic, after_text:String::new()}
    }
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
    DrinkPotion(String),
}

#[derive(Debug, Deserialize, Clone)]
pub struct MonsterRef {
    pub key: String,
    #[serde(default)]
    pub items: Vec<StageItem>,
    #[serde(default)]
    pub actions: Vec<Action>,
    #[serde(default)]
    pub rooms: Vec<String>,
}
