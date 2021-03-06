extern crate rltk;
use rltk::{Console, Rltk, RGB, VirtualKeyCode};
extern crate specs;
use super::{
    Character, Equipped, Item, ItemMap, Map, Named, Player, Position, RunState, Stage, State, Damage,Global,Potion,
    Weapon, Interact, WantToInteract, PlayerView, Journal, Wizard, PlayerResource, WantsToFight, Monster, WantsToFlee, InFight, Fled, CombatResult, Dead
};
use specs::prelude::*;

pub fn draw_ui(state: &State, ctx: &mut Rltk) {
    ctx.set_active_console(1);
    ctx.cls();
    ctx.draw_box(
        50,
        0,
        29,
        24,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    ctx.draw_box(
        0,
        25,
        79,
        4,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );
    let mut y = 25;
    draw_interact(state, ctx, &mut y);
    if y==25{
        draw_position(state, ctx, &mut y);
    }
    draw_player(state, ctx);
    draw_item(state, ctx, &mut y);
}

fn draw_position(state: &State, ctx: &mut Rltk, y: &mut i32)  {
    let ecs = &state.ecs;
    let positions = ecs.read_storage::<Position>();
    let players = ecs.read_storage::<Player>();
    let map = ecs.fetch::<Map>();
    let stage = ecs.fetch::<Stage>();

    let name_length = stage.name.len() + 2;
    let x_pos = (22 - (name_length / 2)) as i32;
    let black = RGB::named(rltk::BLACK);
    let white = RGB::named(rltk::WHITE);

    ctx.print_color(x_pos + 1, 0, white, black, &stage.name);

    
    for (pos, _player) in (&positions, &players).join() {
        if let Some(r) = &map.tile(pos.x, pos.y).room {
            let room = stage
                .rooms
                .get(r)
                .unwrap_or_else(|| panic!("no room for code {}!", r));
            print_multiline(ctx, 1, y, 78, vec![&room.name, &room.description]);
        }
        //format!("x: {}, y: {}", pos.x, pos.y);
        //ctx.print_color(1, 51, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &pos_name);
    }
}

fn print_multiline(ctx: &mut Rltk, x: i32, y: &mut i32, width: usize, text: Vec<&str>) {
    for s in text {
        for l in s.lines() {
            for c in split(&l, width){
                ctx.print(x, *y, &c);
                *y += 1;
            }
        }
    }
}

fn print_multiline_color(ctx: &mut Rltk, x: i32, y: &mut i32, fg: RGB, width: usize, text: Vec<&str>) {
    for s in text {
        for l in s.lines() {
            for c in split(&l, width){
                ctx.print_color(x, *y, fg, RGB::named(rltk::BLACK), &c);
                *y += 1;
            }
        }
    }
}

fn draw_player(state: &State, ctx: &mut Rltk) {
    let ecs = &state.ecs;
    let characters = ecs.read_storage::<Character>();
    let players = ecs.read_storage::<Player>();
    let global = ecs.fetch::<Global>();
    let stage = ecs.fetch::<Stage>();
    let mut y = 0;
    let pr = ecs.fetch::<PlayerResource>();
    match pr.player_view {
        PlayerView::Characteristics => {
            ctx.print(51, y, "Character");
            y+=1;
            for (ch, _player) in (&characters, &players).join() {
                y = draw_attribute(ctx, "Strength:", ch.strength, y);
                y = draw_attribute(ctx, "Dexterity:", ch.dexterity, y);
                y = draw_attribute(ctx, "Willpower:", ch.willpower, y);
                y = draw_attribute(ctx, "Intelligence:", ch.intelligence, y);
                y = draw_attribute(ctx, "Charisma:", ch.charisma, y);
                y = draw_attribute(ctx, "Level:", ch.level, y);
                y = draw_attribute(ctx, "Experience:", ch.xp, y);
                y = draw_attribute(ctx, "Life:", ch.life, y);
            }
        },
        PlayerView::Inventory => {
            ctx.print(51, y, "Inventory");
            y+=1;
            let nameds = ecs.read_storage::<Named>();
            let equippeds = ecs.read_storage::<Equipped>();
            let items = ecs.read_storage::<Item>();
            let weapons = ecs.read_storage::<Weapon>();
            let potions = ecs.read_storage::<Potion>();

            if state.runstate == RunState::Dropping {
                let entities = ecs.entities();
                let mut itemmap = ecs.fetch_mut::<ItemMap>();
                itemmap.map.clear();
                let mut j = 0;
        
                for (entity, named, _equipped, _weapon) in (&entities, &nameds, &equippeds, &weapons).join()
                {
                    ctx.set(
                        51,
                        y,
                        RGB::named(rltk::YELLOW),
                        RGB::named(rltk::BLACK),
                        97 + j as u8,
                    );
                    ctx.print_color(
                        53,
                        y,
                        RGB::named(rltk::RED),
                        RGB::named(rltk::BLACK),
                        &named.name,
                    );
                    itemmap.map.insert(j, entity);
                    j += 1;
                    y += 1;
                }
        
                for (entity, named, _equipped, _item) in (&entities, &nameds, &equippeds, &items).join() {
                    ctx.set(
                        51,
                        y,
                        RGB::named(rltk::YELLOW),
                        RGB::named(rltk::BLACK),
                        97 + j as u8,
                    );
                    ctx.print_color(
                        53,
                        y,
                        RGB::named(rltk::BLUE),
                        RGB::named(rltk::BLACK),
                        &named.name,
                    );
                    itemmap.map.insert(j, entity);
                    j += 1;
                    y += 1;
                }

                for (entity, named, _equipped, _potion) in (&entities, &nameds, &equippeds, &potions).join() {
                    ctx.set(
                        51,
                        y,
                        RGB::named(rltk::YELLOW),
                        RGB::named(rltk::BLACK),
                        97 + j as u8,
                    );
                    ctx.print_color(
                        53,
                        y,
                        RGB::named(rltk::GRAY),
                        RGB::named(rltk::BLACK),
                        &named.name,
                    );
                    itemmap.map.insert(j, entity);
                    j += 1;
                    y += 1;
                }
                 
            } else if state.runstate == RunState::Using {
                    let entities = ecs.entities();
                    let mut itemmap = ecs.fetch_mut::<ItemMap>();
                    itemmap.map.clear();
                    let mut j = 0;
            
                    for (entity, named, _equipped, _weapon) in (&entities, &nameds, &equippeds, &potions).join()
                    {
                        ctx.set(
                            51,
                            y,
                            RGB::named(rltk::YELLOW),
                            RGB::named(rltk::BLACK),
                            97 + j as u8,
                        );
                        ctx.print_color(
                            53,
                            y,
                            RGB::named(rltk::GRAY),
                            RGB::named(rltk::BLACK),
                            &named.name,
                        );
                        itemmap.map.insert(j, entity);
                        j += 1;
                        y += 1;
                    }
            
                    
            } else {
                for (named, _equipped, _weapon) in (&nameds, &equippeds, &weapons).join() {
                    ctx.print_color(
                        51,
                        y,
                        RGB::named(rltk::RED),
                        RGB::named(rltk::BLACK),
                        &named.name,
                    );
                    y += 1;
                }
        
                for (named, _equipped, _item) in (&nameds, &equippeds, &items).join() {
                    ctx.print_color(
                        51,
                        y,
                        RGB::named(rltk::BLUE),
                        RGB::named(rltk::BLACK),
                        &named.name,
                    );
                    y += 1;
                }

                for (named, _equipped, _item) in (&nameds, &equippeds, &potions).join() {
                    ctx.print_color(
                        51,
                        y,
                        RGB::named(rltk::GRAY),
                        RGB::named(rltk::BLACK),
                        &named.name,
                    );
                    y += 1;
                }
            }
        },
        PlayerView::Diary => {
            ctx.print(51, y, "Journal");
            y+=1;
            let j = ecs.fetch::<Journal>();
            let idx = j.current;
            let (quest,entry)=&j.entries[idx];
            let quest_name = stage.quests.get(quest).unwrap_or_else(|| panic!("Cannot get quest name for {}", quest));
            if idx>0{
                ctx.print(51,y,"(P)revious");
                y+=1;
            }
            print_multiline_color(ctx, 51, &mut y, RGB::named(rltk::BLUE), 28, vec!(&quest_name));
            print_multiline(ctx, 51, &mut y, 28, vec!(entry,&format!("{}/{}",idx+1,j.entries.len())));
            if idx<j.entries.len()-1 {
                ctx.print(51,y,"(N)ext");
            }
            
        },
        PlayerView::Spells => {
            ctx.print(51, y, "Magic Spells");
            y+=1;
            let wizards = ecs.read_storage::<Wizard>();
            for (_player,wizard) in (&players,&wizards).join(){
                for spell in wizard.spells.iter() {
                    let spell_struct=global.spells.get(spell).unwrap_or_else(|| panic!("no spell {}", spell));
                    let spell_desc=format!("{} ({})",spell_struct.name,spell_struct.description);
                    print_multiline(ctx, 51, &mut y, 28, vec!(&spell_desc));
                }
            }
        },
        PlayerView::Help => {
            print_multiline(ctx, 51, &mut y, 28, vec!(
                "Help",
                "h/F1   Help",
                "arrows Move",
                "g      Get (pick up) item",
                "d      Drop item",
                "c      Show Character",
                "i      Show Inventory",
                "m      Show Magic Spells",
                "j      Show Journal",
                "p      Prev. Journal Entry",
                "n      Next Journal Entry",
                "l      Last Journal Entry",
            ));
        },
    };
   
}

fn draw_attribute(ctx: &mut Rltk, name: &str, value: u32, y: i32) -> i32 {
    ctx.print(51, y, name);
    ctx.print(71, y, &format!("{:>4}", value));
    y + 1
}

fn draw_item(state: &State, ctx: &mut Rltk, y: &mut i32) {
    let ecs = &state.ecs;
    let positions = ecs.read_storage::<Position>();
    let players = ecs.read_storage::<Player>();

    let named = ecs.read_storage::<Named>();
    let items = ecs.read_storage::<Item>();
    let weapons = ecs.read_storage::<Weapon>();
    let potions = ecs.read_storage::<Potion>();
    let map = ecs.fetch::<Map>();

    for (ppos, _player) in (&positions, &players).join() {
        for ent in map.tile(ppos.x,ppos.y).content.iter() {
            if let Some(name) = named.get(*ent) {
                let full_text=format!("{} (g to pick up)",name.name);
                if items.contains(*ent) { 
                    ctx.print_color(
                        1,
                        *y,
                        RGB::named(rltk::BLUE),
                        RGB::named(rltk::BLACK),
                        &full_text,
                    );
                    *y+=1;
                } else if weapons.contains(*ent){
                    ctx.print_color(
                        1,
                        *y,
                        RGB::named(rltk::RED),
                        RGB::named(rltk::BLACK),
                        &full_text,
                    );
                    *y+=1;
                } else if potions.contains(*ent){
                    ctx.print_color(
                        1,
                        *y,
                        RGB::named(rltk::GRAY),
                        RGB::named(rltk::BLACK),
                        &full_text,
                    );
                    *y+=1;
                }
            }
        }

    }
}

fn draw_interact(state: &State, ctx: &mut Rltk, y: &mut i32) {
    let ecs = &state.ecs;
    let positions = ecs.read_storage::<Position>();
    let players = ecs.read_storage::<Player>();

    let named = ecs.read_storage::<Named>();
    let interacts = ecs.read_storage::<Interact>();
    let winteracts = ecs.read_storage::<WantToInteract>();
    let map = ecs.fetch::<Map>();

    for (ppos, _player) in (&positions, &players).join() {
        for ent in map.tile(ppos.x,ppos.y).content.iter() {
            if let Some(name) = named.get(*ent) {
                if let Some(i) = interacts.get(*ent) { 
                    ctx.print_color(
                        1,
                        *y,
                        RGB::named(rltk::GREEN),
                        RGB::named(rltk::BLACK),
                        &name.name,
                    );
                    *y += 1;
                    print_multiline_color(ctx, 1, y, RGB::named(rltk::GREEN), 78, vec!(&i.interaction.text));
                } else if let Some(i) = winteracts.get(*ent) {
                    let full_text=format!("{} (y to accept)",i.interaction.text);
                    ctx.print_color(
                        1,
                        *y,
                        RGB::named(rltk::GREEN),
                        RGB::named(rltk::BLACK),
                        &name.name,
                    );
                    *y += 1;
                    print_multiline_color(ctx, 1, y, RGB::named(rltk::GREEN), 78, vec!(&full_text));

                }
            }
        }
        
    }
}

pub fn draw_combat(gs : &State, ctx : &mut Rltk) -> CombatResult {
    let wizards = gs.ecs.read_storage::<Wizard>();
    let player_entity = gs.ecs.fetch::<Entity>();
    let global = gs.ecs.fetch::<Global>();
    
    let owizard =  wizards.get(*player_entity)  ;

    let fights = gs.ecs.read_storage::<InFight>();
    let fleds = gs.ecs.read_storage::<Fled>();
    let ofled = fleds.get(*player_entity);

    let characters = gs.ecs.read_storage::<Character>();
    let damages = gs.ecs.read_storage::<Damage>();
    let entities = gs.ecs.entities();
    let mut count = 3;

    let names= gs.ecs.read_storage::<Named>();
    let monsters= gs.ecs.read_storage::<Monster>();
    let dead= gs.ecs.read_storage::<Dead>();

    if ofled.is_none(){
        if let Some(wizard) = owizard { 
            count += wizard.spells.len();
        }
    }

    let mut msgs = vec!();
    for(damage,character,entity) in (&damages,&characters, &entities).join(){
        if entity==*player_entity {
            if damage.damage>0{
                msgs.push(format!("You get hit for {} damage! You have {} life points left.",damage.damage,character.life));
            } else {
                msgs.push("Pfew, no damage!".to_owned());
            }
        } else {
            let name = &names.get(entity).expect("No name for monster").name;
            if damage.damage>0{
                msgs.push(format!("You hit {} for {} damage!",name,damage.damage));
            } else {
                msgs.push("You miss!".to_owned());
            }
        }
    }
    let mut player_dead = false;
    let mut monster_dead = false;
    for(_dead,entity) in (&dead, &entities).join(){
        
        if entity==*player_entity {
            player_dead = true;
            msgs.push("You are dead!".to_string());
        } else {
            monster_dead = true;
            let name = &names.get(entity).expect("No name for monster").name;
            msgs.push(format!("You killed {}",name));
        }
    }
    count+=msgs.len();
    let x = 10;
    let textx = x+3;

    let mut y = (10 - (count / 2)) as i32;
    ctx.draw_box(x, y-2, 60, (count+3) as i32, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
    let mut monster_ents = vec!();

    for (name, _wtf, _ms, ent) in (&names,&fights,&monsters, &entities).join(){
        ctx.print_color(textx, y-2, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &name.name);
        monster_ents.push(ent);
    }

    for s in msgs {
        ctx.print_color(textx, y, RGB::named(rltk::RED), RGB::named(rltk::BLACK), &s);
        y+=1;
    }


    let mut rs = CombatResult::Continue;
    if player_dead || monster_dead {
        rs=stop_combat(ctx, textx, y);
    } else if ofled.is_some(){
        ctx.print_color(textx, y, RGB::named(rltk::GREEN), RGB::named(rltk::BLACK), "You manage to escape!");
        y+=1;
        rs=stop_combat(ctx, textx, y);
    } else {
        let mut option_idx=0;

        write_option(ctx, textx, y, option_idx, "Attack");
        y+=1;
        option_idx+=1;
        write_option(ctx, textx, y, option_idx, "Flee");
        if let Some(wizard) = owizard { 
            for spell in wizard.spells.iter() {
                let spell_struct=global.spells.get(spell).unwrap_or_else(|| panic!("no spell {}", spell));
                write_option(ctx, textx, y, option_idx, &spell_struct.name );
                y+=1;
                option_idx+=1;
            }
        }

        if let Some(key) = ctx.key {
            let mut wantstofight = gs.ecs.write_storage::<WantsToFight>();
            let selection = rltk::letter_to_option(key);
            for m in monster_ents {
                wantstofight.insert(m,WantsToFight{}).expect("Cannot add fight");
            }
            match selection {
                0 => {
                    wantstofight.insert(*player_entity,WantsToFight{}).expect("Cannot add fight");
                },
                1 => {
                    let mut flee=gs.ecs.write_storage::<WantsToFlee>();
                    flee.insert(*player_entity,WantsToFlee{}).expect("Cannot add flee");
                },
                _ => {

                }
            }
            
        }
    }
    rs

}

fn stop_combat(ctx : &mut Rltk, textx: i32, y: i32) -> CombatResult{
    ctx.print_color(textx, y as i32, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "SPACE to continue");
    match ctx.key {
        None => CombatResult::Continue,
        Some(key) => {
            match key {
                VirtualKeyCode::Space => CombatResult::Stop,
                _ => CombatResult::Continue,
            }   
        },
    }
}

fn write_option(ctx : &mut Rltk, textx: i32, y: i32, j: i32, option: &str){
    ctx.set(textx, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437('('));
    ctx.set(textx +1, y, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), 97+j as u8);
    ctx.set(textx +2, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), rltk::to_cp437(')'));
    ctx.print(textx +4, y, option);
}

fn split(s: &str, width: usize) -> Vec<String> {
    let mut r = vec!();
    if s.len()<width{
        r.push(s.to_owned());
        return r;
    }
    let mut line = String::new();
    let mut cur = String::new();
    let mut sep = String::new();
    for c in s.chars(){
        if c.is_whitespace(){
            if line.len() + sep.len() + cur.len()<=width {
                line.push_str(&sep);
                sep.clear();
            } else if !line.is_empty(){
                r.push(line);
                line = String::new();
                sep.clear();
                
            }
            line.push_str(&cur);
            cur.clear();
            sep.push(c);
        } else {
            cur.push(c);
        }
    }
    if !cur.is_empty(){
        if line.len() + sep.len() + cur.len()<=width {
            line.push_str(&sep);
        } else if !line.is_empty(){
            r.push(line);
            line = String::new();
            
        }
        line.push_str(&cur);
    }
    if !line.is_empty(){
        r.push(line);
    }

    r
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_split() {
        assert_eq!(vec!("hello"), split("hello",10));
        assert_eq!(vec!("hello"), split("hello",3));
        assert_eq!(vec!("hello world"), split("hello world",20));
        assert_eq!(vec!("hello","world"), split("hello world",10));
        assert_eq!(vec!("hello crual","world"), split("hello crual world",15));
        assert_eq!(vec!("ab cd","ef gh","ij"), split("ab cd ef gh ij",5));
        assert_eq!(vec!("ab cd","ef gh","ij"), split("ab cd ef gh ij",6));
    }
}
