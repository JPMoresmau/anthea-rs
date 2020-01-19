use super::{Character,Weapon};

use rand::Rng;

pub fn initiative(player: &Character, monster: &Character) -> bool {
    against(player.dexterity *2 + player.willpower, monster.dexterity *2 + monster.willpower)
}

pub fn against(score1: u32, score2: u32) -> bool {
    let total = score1 + score2;
    let roll = rand::thread_rng().gen_range(1, total+1);
    roll<=score1
}

pub fn fight_round(attacker: &Character, weapon: &Weapon, defender: &Character) -> u32 {
    let sc1 = attacker.dexterity *2 + attacker.strength;
    let sc2 = defender.dexterity *2 + defender.strength;
    let total = sc1 + sc2;
    let roll = rand::thread_rng().gen_range(1, total+1);

    if roll<=sc1{
        let mut dmg = rand::thread_rng().gen_range(weapon.damage_min,weapon.damage_max+1);
        if attacker.strength>10 {
            dmg += (attacker.strength-10)/3;
        }
        if roll<total/10 {
            dmg *= 2;
        }
        return dmg;
    }
    0
}