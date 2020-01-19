extern crate specs;
use specs::prelude::*;
use super::{Position, Player, Keyed, Equipped, Weapon, Character,Dead,
     Wizard, Stage, InFight, Monster, WantsToFight, WantsToFlee,Map, Fled,fight_round,Damage};
use rand::Rng;

pub struct CombatSystem {}

impl<'a> System<'a> for CombatSystem {
    type SystemData = ( ReadExpect<'a, Stage>,
                        ReadExpect<'a, Map>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Player>,
                        ReadStorage<'a, Keyed>,
                        ReadStorage<'a, Equipped>,
                        ReadStorage<'a, Weapon>,
                        WriteStorage<'a, Character>,
                        ReadStorage<'a, Wizard>,
                        ReadStorage<'a, InFight>,
                        ReadStorage<'a, Monster>,
                        WriteStorage<'a, WantsToFight>,
                        WriteStorage<'a, WantsToFlee>,
                        WriteStorage<'a, Fled>,
                        WriteStorage<'a, Damage>,
                        WriteStorage<'a, Dead>,
                        Entities<'a>,
                    );

    fn run(&mut self, data : Self::SystemData) {
        let (stage, map, mut positions, players, keyeds, equipped, weapons, mut characters, mut wizards,infight,monsters,mut wantstofight, mut wantstoflee, mut fled, mut damages, mut dead, entities) = data;

        let mut dmgs = vec!();

        for (_player,_wantstofight,chr) in (&players,&wantstofight,&characters).join(){
            for (_monster,_infight,mchr, ent) in (&monsters,&infight,&characters,&entities).join(){
                let mut cnt = 0;
                let mut dmg = 0;
                for (_e,weapon) in (&equipped,&weapons).join(){
                    dmg = fight_round(chr, weapon, mchr);
                    cnt +=1;
                }
                // no weapon: bare hands...
                if cnt == 0 {
                    dmg = fight_round(chr, &Weapon{damage_min:1,damage_max:3}, mchr);
                }
                if dmg>0 {
                    dmgs.push((ent,dmg));
                }
            }
        }

        for (_monster,_wantstofight,chr,weapon) in (&monsters,&wantstofight,&characters,&weapons).join(){
            for (_player,pchr, ent) in (&players,&characters,&entities).join(){
                let dmg = fight_round(chr, weapon, pchr);
                println!("monster attacked player: {}",dmg);
                if dmg>0 {
                    dmgs.push((ent,dmg));
                }
            }
        }

        if dmgs.len()>0 {
            damages.clear();
            for (ent,dmg) in dmgs.iter(){
                damages.insert(*ent,Damage{damage:*dmg}).expect("Cannot add damage");
            }
            for(damage,character,entity) in (&damages,&mut characters,&entities).join(){
                character.life=0.max(character.life as i32 - damage.damage as i32) as u32;
                if character.life==0 {
                    dead.insert(entity,Dead{}).expect("Cannot add Dead");
                }
            }
        }

        for (_player,_wantstoflee,ent) in (&players,&wantstoflee,&entities).join() {
            let vs: Vec<usize>=map.tiles.iter().enumerate().filter(|(_i,t)| t.visited).map(|(i,_t)| i).collect();
            let idx = rand::thread_rng().gen_range(0, vs.len());
            let (x,y) = map.idx_xy(vs[idx]);
            let mut p=positions.get_mut(ent).expect("no position for character");
            p.x=x;
            p.y=y;
            fled.insert(ent,Fled{}).expect("cannot add Fled on player");
        }


        wantstoflee.clear();
        wantstofight.clear();

    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum CombatResult { Stop, Continue}