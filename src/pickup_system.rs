extern crate specs;
use specs::prelude::*;
use super::{WantToPickup,Position,Equipped,WantToDrop,Player,Map, PlayerResource, PlayerView};

pub struct PickupSystem {}

impl<'a> System<'a> for PickupSystem {
    type SystemData = ( WriteStorage<'a, WantToPickup>,
                        WriteStorage<'a, WantToDrop>,
                        WriteStorage<'a, Position>,
                        WriteStorage<'a, Equipped>,
                        ReadStorage<'a, Player>,
                        WriteExpect<'a, Map>,
                        WriteExpect<'a, PlayerResource>,
                        Entities<'a>,);

    
    fn run(&mut self, data : Self::SystemData) {
        let (mut pickups, mut drops, mut positions, mut equipped, players,mut map, mut pr, entities) = data;
        let mut opos=None;
        for (_player,pos) in (&players,&positions).join(){
            opos=Some((pos.x,pos.y));
        }
        if let Some((x,y)) = opos {
            let mut changed=false;
            for (_pickup,entity) in (&pickups,&entities).join(){
                positions.remove(entity);
                equipped.insert(entity, Equipped{}).expect("Cannot equip item");
                map.mut_tile(x,y).content.remove(&entity);
                changed=true;
            }
            pickups.clear();

            for (_drop,entity) in (&drops,&entities).join(){
                equipped.remove(entity);
                positions.insert(entity, Position{x:x,y:y}).expect("Cannot drop item");
                map.mut_tile(x,y).content.insert(entity);
                changed=true;
            }
            drops.clear();
            if changed {
                pr.player_view=PlayerView::Inventory;
            }
        }
    }}