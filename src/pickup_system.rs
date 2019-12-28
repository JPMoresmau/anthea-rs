extern crate specs;
use specs::prelude::*;
use super::{WantToPickup,Position,Equipped};

pub struct PickupSystem {}

impl<'a> System<'a> for PickupSystem {
    type SystemData = ( WriteStorage<'a, WantToPickup>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, Equipped>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut pickups, mut positions, mut equipped) = data;
        for pickup in pickups.join(){
            positions.remove(pickup.item);
            equipped.insert(pickup.item, Equipped{}).expect("Cannot equip item");
        }
        pickups.clear();
    }}