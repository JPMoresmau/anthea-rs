extern crate specs;

use specs::prelude::*;
use super::{Interact, Flags, Action, Player, Journal, Keyed, Equipped, Character, Global, Named,Renderable,
     Wizard, PlayerResource, PlayerView, Potion, Stage, Map};
use rltk::{RGB};

pub struct InteractSystem {}


impl<'a> System<'a> for InteractSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( ReadStorage<'a, Interact>,
                        WriteExpect<'a, Flags>,
                        WriteExpect<'a, Journal>,
                        WriteExpect<'a, PlayerResource>,
                        ReadExpect<'a, Stage>,
                        ReadExpect<'a, Global>,
                        WriteExpect<'a, Map>,
                        ReadStorage<'a, Player>,
                        WriteStorage<'a, Keyed>,
                        WriteStorage<'a, Named>,
                        WriteStorage<'a, Potion>,
                        WriteStorage<'a, Equipped>,
                        WriteStorage<'a, Character>,
                        WriteStorage<'a, Wizard>,
                        WriteStorage<'a, Renderable>,
                        
                        Entities<'a>,
                    );

    fn run(&mut self, data : Self::SystemData) {
        let (interacts,mut flags, mut journal, mut pr, stage,global, mut map, players, mut keyeds,mut nameds, mut potions, mut equipped, mut characters, mut wizards, mut renderables, entities) = data;
        
        //let mut ents = vec!();
        for i in interacts.join() {
            //if pos.x!=player_pos.x || pos.y!=player_pos.y {
            let mut actions = vec!();
            i.interaction.actions.iter().for_each(|a| actions.push(a.clone()));
            while !actions.is_empty(){
                let mut new_actions = vec!();
                for act in actions.iter(){
                    match act {
                        Action::SetFlag(quest, flag) => {
                            flags.set.insert((quest.to_owned(), flag.to_owned()));
                        },
                        Action::RemoveFlag(quest, flag) => {
                            flags.set.remove(&(quest.to_owned(), flag.to_owned()));
                        },
                        Action::AddDiary(quest,text) => {
                            journal.entries.push((quest.clone(),text.clone()));
                            journal.current=journal.entries.len()-1;
                            pr.player_view=PlayerView::Diary;
                        },
                        Action::UseItem(item) => {
                            let mut used_entities = Vec::new();
                            for (key,entity,_equip) in (&keyeds,&entities,&equipped).join(){
                                if *item == key.key{
                                    used_entities.push(entity);
                                }
                            }
                            for used in used_entities {
                                entities.delete(used).expect("Unable to delete ued item");
                            }
                        },
                        Action::RaiseXP(amount) => {
                            for (ch,_player) in (&mut characters,&players).join(){
                                ch.xp += amount;
                            }
                        },
                        Action::UpdateCharacter(chr, amount) => {
                            for (ch,_player) in (&mut characters,&players).join(){
                                match chr.as_ref() {
                                    "charisma" => ch.charisma= (ch.charisma as i32 + amount).max(0) as u32,
                                    "strength" => ch.strength= (ch.strength as i32 + amount).max(0) as u32,
                                    "dexterity" => ch.dexterity= (ch.dexterity as i32 + amount).max(0) as u32,
                                    "willpower" => ch.willpower= (ch.willpower as i32 + amount).max(0) as u32,
                                    "intelligence" => ch.intelligence= (ch.intelligence as i32 + amount).max(0) as u32,
                                    "life" => ch.life= (ch.life as i32 + amount).max(0) as u32,
                                    "xp" => ch.xp= (ch.xp as i32 + amount).max(0) as u32,
                                    _ => panic!(format!("unknow characteristic {}",chr.to_owned())),
                                }
                            }
                        },
                        Action::StartQuest(quest) => {
                            flags.set.insert((quest.to_owned(), "STARTED".to_owned()));
                        },
                        Action::CompleteQuest(quest,xp) => {
                            flags.set.insert((quest.to_owned(), "DONE".to_owned()));
                            for (ch,_player) in (&mut characters,&players).join(){
                                ch.xp += xp;
                            }
                        },
                        Action::LearnSpell(spell,xp) => {
                            for (ch,wizard, _player) in (&mut characters,&mut wizards,&players).join(){
                                ch.xp += xp;
                                wizard.spells.insert(spell.to_owned());
                                pr.player_view=PlayerView::Spells;
                            }
                        },
                        Action::PickupPotion(potion) => {
                            let p = global.potions.get(potion).unwrap_or_else(|| panic!("No potion for key {}",potion));
                            let e = entities.create();
                            nameds.insert(e,Named {
                                name: p.name.clone(),
                            }).expect("Cannot name potion");
                            keyeds.insert(e,Keyed { key: potion.to_string() }).expect("Cannot key potion");
                            potions.insert(e,Potion {
                                effects: p.effects.clone(),
                            }).expect("Cannot add potion");
                            equipped.insert(e,Equipped{}).expect("Cannot equip potion");
                            renderables.insert(e,
                            Renderable {
                                glyph: rltk::to_cp437('p'),
                                fg: RGB::named(rltk::GRAY),
                                bg: RGB::named(rltk::BLACK),
                            }).expect("Cannot add renderable");
                        },
                        Action::DrinkPotion(potion) => {
                            println!("drink potion");
                            let p = global.potions.get(potion).unwrap_or_else(|| panic!("No potion for key {}",potion));
                            new_actions.push(Action::UseItem(potion.to_string()));
                            for e in p.effects.iter() {
                                new_actions.push(Action::UpdateCharacter(e.characteristic.to_string(),e.diff));
                            }
                        }
                        Action::AddDoor(room1,room2,width)=>{
                            map.add_door(&stage,room1,room2,*width);
                        },
                    };
                }
                actions.clear();
                actions.append(&mut new_actions);
                println!("actions: {:?}", actions);
            }
               
            
            //ents.push(ent);
                //}
            
           /* ents.iter().for_each(|e| {interacts.remove(*e);});
            ents.clear();
            for (pos,_wi,ent) in (&positions,&winteracts,&entities).join(){
                if pos.x!=player_pos.x || pos.y!=player_pos.y {
                    ents.push(ent);
                }
            }
            ents.iter().for_each(|e| {winteracts.remove(*e);});*/
        }
       
    }

    
}

