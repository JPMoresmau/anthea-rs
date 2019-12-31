extern crate rltk;
use rltk::{Console, Rltk, RGB};
extern crate specs;
use super::{
    Character, Equipped, Item, ItemMap, Map, Named, Player, Position, RunState, Stage, State,
    Weapon, Interact
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
    draw_npc(state, ctx, &mut y);
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
        if let Some(r) = map.rooms.get(&(pos.x, pos.y)) {
            let room = stage
                .rooms
                .get(r)
                .expect(&format!("no room for code {}!", r));
            print_multiline(ctx, 1, y, vec![&room.name, &room.description]);
        }
        //format!("x: {}, y: {}", pos.x, pos.y);
        //ctx.print_color(1, 51, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &pos_name);
    }
}

fn print_multiline(ctx: &mut Rltk, x: i32, y: &mut i32, text: Vec<&String>) {
    for s in text {
        for l in s.lines() {
            ctx.print(x, *y, l);
            *y += 1;
        }
    }
}

fn draw_player(state: &State, ctx: &mut Rltk) {
    let ecs = &state.ecs;
    let characters = ecs.read_storage::<Character>();
    let players = ecs.read_storage::<Player>();
    let mut y = 1;
    for (ch, _player) in (&characters, &players).join() {
        y = draw_attribute(ctx, "Strength:", ch.strength, y);
        y = draw_attribute(ctx, "Dexterity:", ch.dexterity, y);
        y = draw_attribute(ctx, "Willpower:", ch.willpower, y);
        y = draw_attribute(ctx, "Intelligence:", ch.intelligence, y);
        y = draw_attribute(ctx, "Charisma:", ch.charisma, y);
        y = draw_attribute(ctx, "Level:", ch.level, y);
        y = draw_attribute(ctx, "XP:", ch.xp, y);
        y = draw_attribute(ctx, "Life:", ch.life, y);
    }

    y += 1;
    let nameds = ecs.read_storage::<Named>();
    let equippeds = ecs.read_storage::<Equipped>();
    let items = ecs.read_storage::<Item>();
    let weapons = ecs.read_storage::<Weapon>();

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
    }
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

    let named_positions = ecs.read_storage::<Position>();
    let named = ecs.read_storage::<Named>();
    let items = ecs.read_storage::<Item>();
    let weapons = ecs.read_storage::<Weapon>();
    
    for (ppos, _player) in (&positions, &players).join() {
        for (npos, named, _item) in (&named_positions, &named, &items).join() {
            if ppos.x == npos.x && ppos.y == npos.y {
                ctx.print_color(
                    1,
                    *y,
                    RGB::named(rltk::BLUE),
                    RGB::named(rltk::BLACK),
                    &named.name,
                );
                *y+=1;
            }
        }
        for (npos, named, _weapons) in (&named_positions, &named, &weapons).join() {
            if ppos.x == npos.x && ppos.y == npos.y {
                ctx.print_color(
                    1,
                    *y,
                    RGB::named(rltk::RED),
                    RGB::named(rltk::BLACK),
                    &named.name,
                );
                *y+=1;
            }
        }

    }
}

fn draw_npc(state: &State, ctx: &mut Rltk, y: &mut i32) {
    let ecs = &state.ecs;
    let positions = ecs.read_storage::<Position>();
    let players = ecs.read_storage::<Player>();

    let named_positions = ecs.read_storage::<Position>();
    let named = ecs.read_storage::<Named>();
    let interacts = ecs.read_storage::<Interact>();

    for (ppos, _player) in (&positions, &players).join() {
        for (npos, named, i) in (&named_positions, &named, &interacts).join() {
            if ppos.x == npos.x && ppos.y == npos.y {
                ctx.print_color(
                    1,
                    *y,
                    RGB::named(rltk::GREEN),
                    RGB::named(rltk::BLACK),
                    &named.name,
                );
                *y += 1;
                for l in i.interaction.text.lines() {
                    ctx.print_color(
                        1,
                        *y,
                        RGB::named(rltk::GREEN),
                        RGB::named(rltk::BLACK),
                        &l,
                    );
                    *y += 1;
                }
                
            }
        }
    }
}