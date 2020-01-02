extern crate specs;
use specs::prelude::*;
use super::{Interact, Flags, Action, Position, Player, Journal, Keyed, Equipped, Character,
     Wizard, WantToInteract, PlayerResource, PlayerView};

pub struct InteractSystem {}


impl<'a> System<'a> for InteractSystem {
    type SystemData = ( WriteStorage<'a, Interact>,
                        WriteStorage<'a, WantToInteract>,
                        WriteExpect<'a, Flags>,
                        WriteExpect<'a, Journal>,
                        WriteExpect<'a, PlayerResource>,
                        ReadStorage<'a, Position>,
                        ReadStorage<'a, Player>,
                        ReadStorage<'a, Keyed>,
                        WriteStorage<'a, Equipped>,
                        WriteStorage<'a, Character>,
                        WriteStorage<'a, Wizard>,
                        Entities<'a>,
                    );

    fn run(&mut self, data : Self::SystemData) {
        let (mut interacts,mut winteracts,mut flags, mut journal, mut pr, positions, players, keyeds, mut equipped, mut characters, mut wizards, entities) = data;
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
                                for (key,entity) in (&keyeds,&entities).join(){
                                    if *item == key.key{
                                        equipped.remove(entity);
                                    }
                                }
                            },
                            Action::RaiseXP(amount) => {
                                for (ch,_player) in (&mut characters,&players).join(){
                                    ch.xp = ch.xp+ amount;
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
                                    ch.xp = ch.xp+ xp;
                                }
                            },
                            Action::LearnSpell(spell,xp) => {
                                for (ch,wizard, _player) in (&mut characters,&mut wizards,&players).join(){
                                    ch.xp = ch.xp+ xp;
                                    wizard.spells.insert(spell.to_owned());
                                    pr.player_view=PlayerView::Spells;
                                }
                            }
                        };
                    }
                    ents.push(ent);
                }
            }
            ents.iter().for_each(|e| {interacts.remove(*e);});
            ents.clear();
            for (pos,_wi,ent) in (&positions,&winteracts,&entities).join(){
                if pos.x!=player_pos.x || pos.y!=player_pos.y {
                    ents.push(ent);
                }
            }
            ents.iter().for_each(|e| {winteracts.remove(*e);});
        }
       
    }

}

