rltk::add_wasm_support!();
use rltk::{Console, GameState, Rltk, RGB};
use specs::prelude::*;
#[macro_use]
extern crate specs_derive;

use ron::de::from_str;

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

pub struct State {
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        player_input(self, ctx);
        self.run_systems();

        Map::draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}





fn main() {
    let context = Rltk::init_simple8x8(80, 50, "Anthea's Quest", "resources");
    let mut gs = State {
        ecs: World::new()
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let ron1 = include_str!("stage1.ron");
    let stage = match from_str(ron1) {
        Ok(x) => x,
        Err(e) => panic!("Failed to load stage: {}",e),
    };
    let map = Map::new_map(&stage);
    gs.ecs.insert(map);
    let (player_x, player_y) = stage.rooms[&stage.start].dimensions.center();


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
        .build();

    rltk::main_loop(context, gs);
}



