rltk::add_wasm_support!();
use rltk::{Console, GameState, Rltk, RGB};
use specs::prelude::*;
#[macro_use]
extern crate specs_derive;

use ron::de::from_str;

mod gui;
pub use gui::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
pub use rect::*;
mod stage;
pub use stage::*;
mod components;
pub use components::*;
mod visibility_system;
pub use visibility_system::*;
mod pickup_system;
pub use pickup_system::*;

pub struct State {
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {

        ctx.set_active_console(0);
        ctx.cls();
        player_input(self, ctx);
        self.run_systems();

        Map::draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        
        let map = self.ecs.fetch::<Map>();
        for (pos, render) in (&positions, &renderables).join() {
            if map.is_visible(pos.x, pos.y){
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
        gui::draw_ui(&self.ecs, ctx);
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        let mut pick = PickupSystem{};
        pick.run_now(&self.ecs);
        self.ecs.maintain();
    }
}





fn main() {
    let mut context = Rltk::init_simple8x8(80, 60, "Anthea's Quest", "resources");
    let font = context.register_font(rltk::Font::load("resources/vga8x16.png", (8, 16)));

    // Then we initialize it; notice 80x25 (half the height, since 8x16 is twice as tall).
    // This actually returns the console number, but it's always going to be 1.
    context.register_console(rltk::SparseConsole::init(80, 30, &context.backend), font);
    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Character>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<Equipped>();
    gs.ecs.register::<Named>();
    gs.ecs.register::<WantToPickup>();
    gs.ecs.register::<Weapon>();

    let ron1 = include_str!("stage1.ron");
    let stage = match from_str(ron1) {
        Ok(x) => x,
        Err(e) => panic!("Failed to load stage: {}",e),
    };
    let map = Map::new_map(&stage);

    let (player_x, player_y) = stage.rooms[&stage.start].dimensions.center();
    build_items(&mut gs.ecs,&stage);

    gs.ecs.insert(map);
    gs.ecs.insert(stage);

    gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true })
        .with(Character::default())
        .build();

    rltk::main_loop(context, gs);
}

fn build_items(ecs: &mut World,stage: &Stage){
    for item in stage.items.iter() {
        ecs
            .create_entity()
            .with(Position { x: item.position.0, y: item.position.1 })
            .with(Item {})
            .with(Named {name: item.name.clone()})
            .with(Renderable {
                glyph: rltk::to_cp437('q'),
                fg: RGB::named(rltk::BLUE),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
    }
    for item in stage.weapons.iter() {
        ecs
            .create_entity()
            .with(Position { x: item.position.0, y: item.position.1 })
            .with(Weapon {damage_min:item.damage.0, damage_max:item.damage.1})
            .with(Named {name: format!("{} ({}-{})",item.name,item.damage.0,item.damage.1)})
            .with(Renderable {
                glyph: rltk::to_cp437('w'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
    }
}

