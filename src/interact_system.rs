extern crate specs;
use specs::prelude::*;
use super::{Interact, Flags, Action, Position, Player};

pub struct InteractSystem {}


impl<'a> System<'a> for InteractSystem {
    type SystemData = ( WriteStorage<'a, Interact>,
                        WriteExpect<'a, Flags>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, Player>,
                        Entities<'a>,
                    );

    fn run(&mut self, data : Self::SystemData) {
        let (mut interacts,mut flags, positions, players,entities) = data;
        let mut oplayer_pos=None;
        for (pos,_player) in (&positions,&players).join(){
            oplayer_pos=Some(pos);
        }
        if let Some(player_pos) = oplayer_pos {
            let mut ents = vec!();
            for (pos,i,ent) in (&positions,&interacts,&entities).join(){
                if pos.x!=player_pos.x || pos.y!=player_pos.y {
                    for act in i.interaction.actions.iter(){
                        match act {
                            Action::SetFlag(quest, flag) => {flags.set.insert((quest.to_owned(), flag.to_owned()));},
                            _ => (),
                        };
                    }
                    ents.push(ent);
                }
            }
            ents.iter().for_each(|e| {interacts.remove(*e);});
        }
       
    }

}
