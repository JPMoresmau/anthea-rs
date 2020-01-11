use super::{
    ItemMap, Map, Named, Player, Position, RunState, State, TileType, Viewshed, WantToDrop,
    WantToPickup, Interaction, InteractionType, Condition, Flags, Keyed, Equipped, Item, InteractionProvider, 
    Interact, WantToInteract, PlayerView, Journal, PlayerResource
};
use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    let mut new_pos=Option::None;

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx].tile_type != TileType::Wall {
            pos.x = min(map.width - 1, max(0, pos.x + delta_x));
            pos.y = min(map.height - 1, max(0, pos.y + delta_y));
            new_pos=Option::Some((pos.x,pos.y));
            viewshed.dirty = true;

        }
    }

    if let Some((x,y)) = new_pos {
        let ips = ecs.read_storage::<InteractionProvider>();
        let mut interacts = ecs.write_storage::<Interact>();
        let mut winteracts = ecs.write_storage::<WantToInteract>();
        let map = ecs.fetch::<Map>();
        for ent in map.tile(x,y).content.iter() {
            if let Some(ip) = ips.get(*ent) {
                if let Some(i) = get_interaction(ecs,ip) {
                    match i.interaction_type {
                        InteractionType::Question => {winteracts.insert(*ent, WantToInteract{interaction: i,}).expect("cannot add interaction");},
                        InteractionType::Automatic => {interacts.insert(*ent, Interact{interaction: i,}).expect("cannot add interaction");},
                    }
                }
                
            }
        }

    }
}

fn pickup_item(ecs: &mut World) {
    let positions = ecs.read_storage::<Position>();
    let players = ecs.read_storage::<Player>();

    let named_positions = ecs.read_storage::<Position>();
    let named = ecs.read_storage::<Named>();

    let entities = ecs.entities();
    let mut item = None;
    for (ppos, _player) in (&positions, &players).join() {
        for (npos, _named, ent) in (&named_positions, &named, &entities).join() {
            if ppos.x == npos.x && ppos.y == npos.y {
                item = Some(ent);
                break;
            }
        }
    }
    if let Some(ent) = item {
        let mut want = ecs.write_storage::<WantToPickup>();
        want.insert(ent, WantToPickup {})
            .expect("Unable to intent to equip item");
    }
}

fn drop_item(ecs: &mut World, ix: i32) {
    let mut itemmap = ecs.fetch_mut::<ItemMap>();
    if let Some(ent) = itemmap.map.get(&ix) {
        let mut want = ecs.write_storage::<WantToDrop>();
        want.insert(*ent, WantToDrop {})
            .expect("Unable to intent drop item");
    }
    itemmap.map.clear();
}

fn set_player_view(gs: &mut State, player_view: PlayerView){
    let mut pr=gs.ecs.fetch_mut::<PlayerResource>();
    pr.player_view=player_view;
}

enum JournalMove {
    Previous, Next, Last,
}

fn set_journal_entry(gs: &mut State, jmove: JournalMove){
    let mut j = gs.ecs.fetch_mut::<Journal>();
    match jmove {
        JournalMove::Previous => {
            if j.current>0 {
                j.current -= 1;
            }
        },
        JournalMove::Next => {
            if j.current< j.entries.len()-1 {
                j.current += 1;
            }
        },
        JournalMove::Last => {
            j.current = j.entries.len()-1;
        },
    }
}

fn interact(ecs: &mut World){
    let mut winteract = ecs.write_storage::<WantToInteract>();
    let mut interact = ecs.write_storage::<Interact>();
    let entities = ecs.entities();
    for (wi, ent) in (&winteract,&entities).join(){
        let i = Interaction {text: wi.interaction.after_text.clone(), ..wi.interaction.clone()};
        interact.insert(ent,Interact{interaction: i}).expect("Cannot insert interaction");
    }
    winteract.clear();
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        None => return waiting_state(gs.runstate), // Nothing happened
        Some(key) => match gs.runstate {
            RunState::Dropping => drop_item(&mut gs.ecs, rltk::letter_to_option(key)),
            _ => match key {
                VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
                VirtualKeyCode::Numpad4 => try_move_player(-1, 0, &mut gs.ecs),
                VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
                VirtualKeyCode::Numpad6 => try_move_player(1, 0, &mut gs.ecs),
                VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
                VirtualKeyCode::Numpad8 => try_move_player(0, -1, &mut gs.ecs),
                VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
                VirtualKeyCode::Numpad2 => try_move_player(0, 1, &mut gs.ecs),
                VirtualKeyCode::J => set_player_view(gs, PlayerView::Diary),
                VirtualKeyCode::C => set_player_view(gs, PlayerView::Characteristics),
                VirtualKeyCode::I => set_player_view(gs, PlayerView::Inventory),
                VirtualKeyCode::M => set_player_view(gs, PlayerView::Spells),
                VirtualKeyCode::H => set_player_view(gs, PlayerView::Help),
                VirtualKeyCode::F1 => set_player_view(gs, PlayerView::Help),
                VirtualKeyCode::G => pickup_item(&mut gs.ecs),
                VirtualKeyCode::D => {
                    set_player_view(gs, PlayerView::Inventory);
                    return RunState::Dropping;
                },
                VirtualKeyCode::P => set_journal_entry(gs, JournalMove::Previous),
                VirtualKeyCode::N => set_journal_entry(gs, JournalMove::Next),
                VirtualKeyCode::L => set_journal_entry(gs, JournalMove::Last),
                VirtualKeyCode::Y => interact(&mut gs.ecs),
                _ => return RunState::Paused,
            },
        },
    }
    RunState::Running
}

fn waiting_state(runstate: RunState) -> RunState {
    match runstate {
        RunState::Dropping => RunState::Dropping,
        _ => RunState::Paused,
    }
}

fn get_interaction(ecs: &World,ip: &InteractionProvider) -> Option<Interaction> {
    ip.interactions.iter()
        .filter(|i| valid_interaction(ecs, i))
        .last().cloned()
}

fn valid_interaction(ecs: &World,interaction: &Interaction) -> bool {
    for c in interaction.conditions.iter(){
        if !valid_condition(ecs, c){
            return false;
        }
    }
    return true;
}

fn valid_condition(ecs: &World, condition: &Condition) -> bool {
    match condition {
        Condition::IfFlag(q,f) =>{
            let fs=ecs.fetch::<Flags>();
            fs.set.contains(&(q.clone(),f.clone()))
        },
        Condition::IfQuestAchieved(q)=> {
            let fs=ecs.fetch::<Flags>();
            fs.set.contains(&(q.clone(),"DONE".to_owned()))
        }
        Condition::IfItem(item) => {
            let keys = ecs.read_storage::<Keyed>();
            let equippeds = ecs.read_storage::<Equipped>();
            let items = ecs.read_storage::<Item>();
            for (key,_equipped,_item) in (&keys,&equippeds,&items).join(){
                if key.key == *item {
                    return true;
                }
            };
            false
        }
        
    }

}
