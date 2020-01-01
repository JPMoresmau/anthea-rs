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
    pub quests: HashMap<String,String>,
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

#[derive(Debug, Deserialize, Clone)]
pub struct Interaction {
    pub conditions: Vec<Condition>,
    pub text: String,
    pub actions: Vec<Action>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Condition {
    IfFlag((String,String)),
    IfItem(String),
}

#[derive(Debug, Deserialize, Clone)]
pub enum Action {
    SetFlag(String,String),
    AddDiary(String,String),
    UseItem(String),
}

