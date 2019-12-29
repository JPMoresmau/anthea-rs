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
pub struct Interaction {
    conditions: Vec<Condition>,
    text: String,
    actions: Vec<Action>,
}

#[derive(Debug, Deserialize)]
pub enum Condition {
    IfFlag(Flag),
    IfItem(String),
}

#[derive(Debug, Deserialize)]
pub enum Action {
    SetFlag(Flag),
    AddDiary(),
    UseItem(String),
}

#[derive(Debug, Deserialize)]
pub struct DiaryEntry {
    pub quest: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Flag {
    pub quest: String,
    pub flag: String,
}