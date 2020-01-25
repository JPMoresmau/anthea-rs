rltk::add_wasm_support!();
use rltk::{Console, GameState, Rltk, RGB};
use specs::prelude::*;
#[macro_use]
extern crate specs_derive;

use ron::de::from_str;
use std::collections::{BTreeSet, HashMap, HashSet};

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
mod interact_system;
pub use interact_system::*;
mod rpg;
pub use rpg::*;
mod combat_system;
pub use combat_system::*;
mod dead_system;
pub use dead_system::*;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Running,
    Dropping,
    Paused,
    Combat,
}

#[derive(PartialEq, Copy, Clone)]
pub enum PlayerView {
    Characteristics,
    Inventory,
    Spells,
    Diary,
    Help,
}

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.set_active_console(0);
        ctx.cls();

        {
            Map::draw_map(&self.ecs, ctx);

            let positions = self.ecs.read_storage::<Position>();
            let renderables = self.ecs.read_storage::<Renderable>();
            let map = self.ecs.fetch::<Map>();
            for (pos, render) in (&positions, &renderables).join() {
                if map.is_visible(pos.x, pos.y) {
                    ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
                }
            }
            gui::draw_ui(&self, ctx);
        }

        match self.runstate {
            RunState::Combat => {
                let mut csys = CombatSystem {};
                csys.run_now(&self.ecs);
                self.ecs.maintain();
                if let CombatResult::Stop = gui::draw_combat(&self, ctx) {
                    self.runstate = RunState::Running;
                    self.ecs.write_storage::<InFight>().clear();
                    self.ecs.write_storage::<Fled>().clear();
                    let mut dead = DeadSystem {};
                    dead.run_now(&self.ecs);
                    if is_dead(&self.ecs) {
                        ctx.quit();
                    }
                }
            }
            RunState::Running => {
                self.run_systems();
                self.runstate = RunState::Paused;
            }
            _ => self.runstate = player_input(self, ctx),
        }
    }
}

pub fn is_dead(ecs: &World) -> bool {
    let dead = ecs.read_storage::<Dead>();
    let player = ecs.fetch::<Entity>();
    dead.get(*player).is_some()

}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut pick = PickupSystem {};
        pick.run_now(&self.ecs);
        let mut int = InteractSystem {};
        int.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

pub struct ItemMap {
    pub map: HashMap<i32, Entity>,
}

pub struct MonsterMap {
    pub map: HashMap<String, Entity>,
}

pub struct Flags {
    pub set: HashSet<(String, String)>,
}

pub struct Journal {
    pub entries: Vec<(String, String)>,
    pub current: usize,
}

pub struct PlayerResource {
    pub player_view: PlayerView,
}

fn main() {
    let mut context = Rltk::init_simple8x8(80, 60, "Anthea's Quest", "resources");
    let font = context.register_font(rltk::Font::load("resources/vga8x16.png", (8, 16)));

    // Then we initialize it; notice 80x25 (half the height, since 8x16 is twice as tall).
    // This actually returns the console number, but it's always going to be 1.
    context.register_console(rltk::SparseConsole::init(80, 30, &context.backend), font);
    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Character>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<Equipped>();
    gs.ecs.register::<Named>();
    gs.ecs.register::<Keyed>();
    gs.ecs.register::<WantToPickup>();
    gs.ecs.register::<WantToDrop>();
    gs.ecs.register::<Weapon>();
    gs.ecs.register::<NPC>();
    gs.ecs.register::<Interact>();
    gs.ecs.register::<WantToInteract>();
    gs.ecs.register::<Wizard>();
    gs.ecs.register::<InteractionProvider>();
    gs.ecs.register::<Affordance>();
    gs.ecs.register::<Potion>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<WantsToFight>();
    gs.ecs.register::<WantsToFlee>();
    gs.ecs.register::<InFight>();
    gs.ecs.register::<Fled>();
    gs.ecs.register::<Damage>();
    gs.ecs.register::<Dead>();
    gs.ecs.register::<ActionHolder>();

    let ron1 = include_str!("stage1.ron");
    let stage = match from_str(ron1) {
        Ok(x) => x,
        Err(e) => panic!("Failed to load stage: {}", e),
    };
    let mut map = Map::new_map(&stage);

    let (player_x, player_y) = stage.rooms[&stage.start].dimensions.center();
    build_items(&mut gs.ecs, &stage, &mut map);
    build_npcs(&mut gs.ecs, &stage, &mut map);
    build_affordances(&mut gs.ecs, &stage, &mut map);
    build_potions(&mut gs.ecs, &stage);

    let mut mmap = MonsterMap {
        map: HashMap::new(),
    };
    build_monsters(&mut gs.ecs, &stage, &mut mmap);
    gs.ecs.insert(mmap);

    gs.ecs.insert(map);
    gs.ecs.insert(stage);

    gs.ecs.insert(PlayerResource {
        player_view: PlayerView::Characteristics,
    });
    gs.ecs.insert(ItemMap {
        map: HashMap::new(),
    });
    gs.ecs.insert(Flags {
        set: HashSet::new(),
    });
    let mut j = Journal {
        entries: Vec::new(),
        current: 0,
    };
    j.entries.push(("main".to_owned(),"I have decided it, and nothing will alter my resolve. I will set up in search for Father. Peleus cannot stop me.".to_owned()));
    gs.ecs.insert(j);

    let player_entity = gs
        .ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Wizard {
            spells: BTreeSet::new(),
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Character::default())
        .build();
    gs.ecs.insert(player_entity);

    rltk::main_loop(context, gs);
}

fn build_items(ecs: &mut World, stage: &Stage, map: &mut Map) {
    for (key, item) in stage.items.iter() {
        let ent = ecs
            .create_entity()
            .with(Position {
                x: item.position.0,
                y: item.position.1,
            })
            .with(Item {})
            .with(Named {
                name: item.name.clone(),
            })
            .with(Keyed { key: key.clone() })
            .with(Renderable {
                glyph: rltk::to_cp437('q'),
                fg: RGB::named(rltk::BLUE),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
        map.mut_tile(item.position.0, item.position.1)
            .content
            .insert(ent);
    }
    for item in stage.weapons.iter() {
        let ent = ecs
            .create_entity()
            .with(Position {
                x: item.position.0,
                y: item.position.1,
            })
            .with(Weapon {
                damage_min: item.damage.0,
                damage_max: item.damage.1,
            })
            .with(Named {
                name: format!("{} ({}-{})", item.name, item.damage.0, item.damage.1),
            })
            .with(Renderable {
                glyph: rltk::to_cp437('w'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
        map.mut_tile(item.position.0, item.position.1)
            .content
            .insert(ent);
    }
}

fn build_npcs(ecs: &mut World, stage: &Stage, map: &mut Map) {
    for (key, item) in stage.npcs.iter() {
        let ent = ecs
            .create_entity()
            .with(Position {
                x: item.position.0,
                y: item.position.1,
            })
            .with(Named {
                name: item.name.clone(),
            })
            .with(Keyed { key: key.clone() })
            .with(NPC {})
            .with(InteractionProvider {
                interactions: item.interactions.clone(),
            })
            .with(Renderable {
                glyph: rltk::to_cp437('c'),
                fg: RGB::named(rltk::GREEN),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
        map.mut_tile(item.position.0, item.position.1)
            .content
            .insert(ent);
    }
}

fn build_affordances(ecs: &mut World, stage: &Stage, map: &mut Map) {
    for (key, item) in stage.affordances.iter() {
        let ent = ecs
            .create_entity()
            .with(Position {
                x: item.position.0,
                y: item.position.1,
            })
            .with(Named {
                name: item.name.clone(),
            })
            .with(Keyed { key: key.clone() })
            .with(Affordance {})
            .with(InteractionProvider {
                interactions: item.interactions.clone(),
            })
            .with(Renderable {
                glyph: rltk::to_cp437('a'),
                fg: RGB::named(rltk::GREEN),
                bg: RGB::named(rltk::BLACK),
            })
            .build();
        map.mut_tile(item.position.0, item.position.1)
            .content
            .insert(ent);
    }
}

fn build_potions(ecs: &mut World, stage: &Stage) {
    for (key, potion) in stage.potions.iter() {
        ecs.create_entity()
            .with(Named {
                name: potion.name.clone(),
            })
            .with(Keyed { key: key.clone() })
            .with(Potion {
                effects: potion.effects.clone(),
            })
            .build();
    }
}

fn build_monsters(ecs: &mut World, stage: &Stage, mmap: &mut MonsterMap) {
    for (key, monster) in stage.monsters.iter() {
        for room in monster.rooms.iter() {
            let ent = ecs
                .create_entity()
                .with(Named {
                    name: monster.name.clone(),
                })
                .with(monster.character.clone())
                .with(Monster {})
                .with(Keyed { key: key.clone() })
                .with(Weapon {
                    damage_min: monster.weapon.damage.0,
                    damage_max: monster.weapon.damage.1,
                })
                .with(ActionHolder{actions:monster.actions.clone()})
                .build();
            mmap.map.insert(room.to_owned(), ent);
        }
    }
}
