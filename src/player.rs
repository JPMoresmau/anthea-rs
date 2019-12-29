use rltk::{VirtualKeyCode, Rltk};
use specs::prelude::*;
use super::{WantToDrop,Position, Player, TileType, Map, State, Viewshed, Named, WantToPickup, RunState, ItemMap};
use std::cmp::{min, max};

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(map.width-1 , max(0, pos.x + delta_x));
            pos.y = min(map.height-1, max(0, pos.y + delta_y));

            viewshed.dirty = true;
        }
    }
}

fn pickup_item(ecs: &mut World){
    let positions = ecs.read_storage::<Position>();
    let players = ecs.read_storage::<Player>();

    let named_positions = ecs.read_storage::<Position>();
    let named = ecs.read_storage::<Named>();

    let entities = ecs.entities();
    let mut item = None;
    for (ppos, _player) in (&positions, &players).join() {
        for (npos, _named, ent) in (&named_positions, &named, &entities).join() {
            if ppos.x==npos.x && ppos.y == npos.y {
                item=Some(ent);
                break;
            }
        }
    }
    if let Some(ent) = item {
        let mut want = ecs.write_storage::<WantToPickup>();   
        want.insert(ent, WantToPickup{}).expect("Unable to intent to equip item");
        
    } 
}

fn drop_item(ecs: &mut World, ix: i32){
    let mut itemmap = ecs.fetch_mut::<ItemMap>();
    if let Some(ent) = itemmap.map.get(&ix) {
        let mut want = ecs.write_storage::<WantToDrop>();   
        want.insert(*ent, WantToDrop{}).expect("Unable to intent drop item");
    }
    itemmap.map.clear();
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        None => {return waiting_state(gs.runstate)} // Nothing happened
        Some(key) => match gs.runstate {
            RunState::Dropping => drop_item(&mut gs.ecs, rltk::letter_to_option(key)),
            _ => {
                match key {
                    VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
                    VirtualKeyCode::Numpad4 => try_move_player(-1, 0, &mut gs.ecs),
                    VirtualKeyCode::H => try_move_player(-1, 0, &mut gs.ecs),
                    VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
                    VirtualKeyCode::Numpad6 => try_move_player(1, 0, &mut gs.ecs),
                    VirtualKeyCode::L => try_move_player(1, 0, &mut gs.ecs),
                    VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
                    VirtualKeyCode::Numpad8 => try_move_player(0, -1, &mut gs.ecs),
                    VirtualKeyCode::K => try_move_player(0, -1, &mut gs.ecs),
                    VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
                    VirtualKeyCode::Numpad2 => try_move_player(0, 1, &mut gs.ecs),
                    VirtualKeyCode::J => try_move_player(0, 1, &mut gs.ecs),
                    VirtualKeyCode::G => pickup_item(&mut gs.ecs),
                    VirtualKeyCode::D => {return RunState::Dropping},
                    _ => {return RunState::Paused},
                }
            }
        },
    }
    RunState::Running
}

fn waiting_state (runstate: RunState) -> RunState {
    match runstate {
        RunState::Dropping => RunState::Dropping,
        _ => RunState::Paused,
    }
}