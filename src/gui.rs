extern crate rltk;
use rltk::{ RGB, Rltk, Console, };
extern crate specs;
use specs::prelude::*;
use super::{Position, Player, Map, Stage, Character, Named, Item, Equipped, Weapon};

pub fn draw_ui(ecs: &World, ctx : &mut Rltk) {
    ctx.set_active_console(1);
    ctx.cls();
    ctx.draw_box(50, 0, 29, 24, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));

    ctx.draw_box(0, 25, 79, 4, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));

    let y = draw_position(ecs, ctx);
    draw_player(ecs,ctx);
    draw_item(ecs,ctx, y);
}

fn draw_position(ecs: &World, ctx : &mut Rltk) -> i32{
    let positions = ecs.read_storage::<Position>();
    let players = ecs.read_storage::<Player>();
    let map = ecs.fetch::<Map>();
    let stage = ecs.fetch::<Stage>();

    let name_length = stage.name.len() + 2;
    let x_pos = (22 - (name_length / 2)) as i32;
    let black = RGB::named(rltk::BLACK);
    let white = RGB::named(rltk::WHITE);

    ctx.print_color(x_pos+1, 0, white, black, &stage.name);

    let mut y=25;
    for (pos, _player) in (&positions, &players).join() {
        
        if let Some(r) = map.rooms.get(&(pos.x,pos.y)){
            
            let room = stage.rooms.get(r).expect(&format!("no room for code {}!",r));
            y=print_multiline(ctx,1,y,vec!(&room.name, &room.description));
            
        }
        //format!("x: {}, y: {}", pos.x, pos.y);
        //ctx.print_color(1, 51, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), &pos_name);
        
    }
    y
}

fn print_multiline(ctx : &mut Rltk, x: i32, y: i32, text: Vec<&String>) -> i32 {
    let mut ry=y;
    for s in text {
        for l in s.lines() {
            ctx.print(x, ry, l);
            ry+=1;
        }
    }
    ry
}

fn draw_player(ecs: &World, ctx : &mut Rltk){
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
    
    y+=1;

    let nameds = ecs.read_storage::<Named>();
    let equippeds = ecs.read_storage::<Equipped>();
    let items = ecs.read_storage::<Item>();
    let weapons = ecs.read_storage::<Weapon>();
    
    for (named,_equipped,_weapong) in (&nameds,&equippeds,&weapons).join(){
        ctx.print_color(51, y,RGB::named(rltk::RED), RGB::named(rltk::BLACK), &named.name);
        y+=1;
    }

    for (named,_equipped,_item) in (&nameds,&equippeds,&items).join(){
        ctx.print_color(51, y,RGB::named(rltk::BLUE), RGB::named(rltk::BLACK), &named.name);
        y+=1;
    }
}

fn draw_attribute(ctx : &mut Rltk, name: &str, value: u32, y: i32) -> i32{
    ctx.print(51, y, name);
    ctx.print(71, y, &format!("{:>4}",value));
    y+1
}

fn draw_item(ecs: &World, ctx : &mut Rltk, y: i32) {
    let positions = ecs.read_storage::<Position>();
    let players = ecs.read_storage::<Player>();

    let named_positions = ecs.read_storage::<Position>();
    let named = ecs.read_storage::<Named>();
    let items = ecs.read_storage::<Item>();
    let weapons = ecs.read_storage::<Weapon>();
    
    for (ppos, _player) in (&positions, &players).join() {
        for (npos, named, _item) in (&named_positions, &named, &items).join() {
            if ppos.x==npos.x && ppos.y == npos.y {
                ctx.print_color(1, y,RGB::named(rltk::BLUE), RGB::named(rltk::BLACK), &named.name);
            }
        }
        for (npos, named, _weapons) in (&named_positions, &named, &weapons).join() {
            if ppos.x==npos.x && ppos.y == npos.y {
                ctx.print_color(1, y,RGB::named(rltk::RED), RGB::named(rltk::BLACK), &named.name);
            }
        }
    }
}