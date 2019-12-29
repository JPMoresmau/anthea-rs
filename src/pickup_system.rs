extern crate specs;
use specs::prelude::*;
use super::{WantToPickup,Position,Equipped,WantToDrop,Player};

pub struct PickupSystem {}

impl<'a> System<'a> for PickupSystem {
    type SystemData = ( WriteStorage<'a, WantToPickup>,
                        WriteStorage<'a, WantToDrop>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, Equipped>,
                        ReadStorage<'a, Player>,
                        Entities<'a>,);

    fn run(&mut self, data : Self::SystemData) {
        let (mut pickups, mut drops, mut positions, mut equipped, players,entities) = data;
        for (_pickup,entity) in (&pickups,&entities).join(){
            positions.remove(entity);
            equipped.insert(entity, Equipped{}).expect("Cannot equip item");
        }
        pickups.clear();
        let mut ppos = Option::None;
        for (_player,pos) in (&players,&positions).join(){
            ppos=Option::Some(pos.clone());
        }

        for (_drop,entity) in (&drops,&entities).join(){
            equipped.remove(entity);
            positions.insert(entity, ppos.clone().expect("no player position")).expect("Cannot drop item");
        }
        drops.clear();
    }}