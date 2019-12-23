use super::{Rect};
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Stage {
    pub rooms: HashMap<String, Room>,
    pub start: String,
    pub doors: Vec<Door>,
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