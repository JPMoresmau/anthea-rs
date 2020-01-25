extern crate specs;
use super::{
    MonsterMap, Character, Dead, Map, Position, ActionHolder, Interaction, InteractionType, Interact,Action
};
use specs::prelude::*;

pub struct DeadSystem {}

impl<'a> System<'a> for DeadSystem {
    type SystemData = (
        ReadStorage<'a, Dead>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, ActionHolder>,
        ReadStorage<'a, Character>,
        WriteStorage<'a, Interact>,
        ReadExpect<'a, Map>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, MonsterMap>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (deads, positions, actionholders, characters,mut interact, map, player, mut mmap, entities) = data;
        let mut dead_entities = Vec::new();
        let mut interactions = Vec::new();
        for (entity, _dead, character, actionholder) in (&entities, &deads, &characters, &actionholders).join() {
            if entity != *player {
                dead_entities.push(entity);
                let mut acts= actionholder.actions.clone();
                acts.push(Action::RaiseXP(character.xp));
                interactions.push(Interaction{conditions:Vec::new(),text:String::new(),after_text:String::new(),interaction_type:InteractionType::Automatic,actions:acts});
                
            }
        }

        if dead_entities.len() > 0 {
            {
                let pos = positions.get(*player).expect("no position for player");
                if let Some(r) = &map.tile(pos.x, pos.y).room {
                    mmap.map.remove(r);
                }
            };

            for victim in dead_entities {
                entities.delete(victim).expect("Unable to delete");
            }
        }
        for interaction in interactions {
            interact.insert(*player,Interact{interaction:interaction}).expect("Could not add interaction");
        }
    }
}
