use rltk::{ RGB, Rltk, Console, Point, Algorithm2D, BaseMap };
use super::{Rect,Stage};
use std::cmp::{max, min};
use specs::prelude::*;
use std::collections::HashSet;

#[derive(PartialEq, Copy, Clone, Debug )]
pub enum TileType {
    Wall, Floor
}

const MAPWIDTH : i32 = 50;
const MAPHEIGHT : i32 = 50;
const MAPCOUNT : usize = (MAPHEIGHT * MAPWIDTH) as usize;

#[derive(Clone, Debug)]
pub struct Tile {
    pub tile_type: TileType,
    pub revealed: bool,
    pub visible: bool,
    pub room: Option<String>,
    pub content: HashSet<Entity>,
}

impl Tile {
    pub fn new(ttype: TileType)-> Self {
        Tile {
            tile_type: ttype,
            revealed: false,
            visible: false,
            room: None,
            content: HashSet::new(),
        }
    }
}

pub struct Map {
    pub tiles : Vec<Tile>,
    pub width : i32,
    pub height : i32,
}




impl Map {
    pub fn new_map(stage: &Stage) -> Map {
        let mut map = Map {tiles: vec![Tile::new(TileType::Wall); MAPCOUNT],
            width : MAPWIDTH,
            height: MAPHEIGHT,
        };
        
        for (new_code, new_room) in stage.rooms.iter() {
            map.apply_room_to_map(new_code, &new_room.dimensions);
        }
        for door in stage.doors.iter() {
            map.add_door(stage, &door.room1, &door.room2, &door.width);
        }
        map
    }

    pub fn add_door(&mut self, stage: &Stage, r1: &str, r2: &str, width: &usize){
        let room1 = stage.rooms.get(r1).expect("no room");
        let room2 = stage.rooms.get(r2).expect("no room");
        
        if room1.dimensions.is_lined_horizontal(&room2.dimensions){
            self.apply_horizontal_door(&room1.dimensions, &room2.dimensions, width);
        } else {
            self.apply_vertical_door(&room1.dimensions, &room2.dimensions, width);
        }
    }

    fn apply_room_to_map(&mut self, code: &String, room : &Rect) {
        for y in room.y1 +1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx].tile_type = TileType::Floor;
                self.tiles[idx].room = Some(code.clone());
            }
        }
    }

    pub fn tile(&self, x: i32, y: i32) -> &Tile {
        &self.tiles[self.xy_idx(x,y)]
    }

    pub fn mut_tile(&mut self, x: i32, y: i32) -> &mut Tile {
        let idx = self.xy_idx(x,y);
        &mut self.tiles[idx]
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn apply_horizontal_door(&mut self, r1: &Rect, r2: &Rect, width : &usize) {
        let mut ys = vec!();
        for y in r1.y1+1 ..= r1.y2 {
            if y>r2.y1 && y<=r2.y2 {
                ys.push(y);
            }
        }
        while ys.len()>*width {
            ys.remove(0);
            if ys.len()>*width {
                ys.remove(ys.len()-1);
            }
        }
        for y in ys {
            for x in min(r1.x1,r2.x1)+1 .. max(r1.x2,r2.x2) {
                let idx = self.xy_idx(x, y);
                if idx < self.tiles.len() {
                    self.tiles[idx as usize].tile_type = TileType::Floor;
                }
            }
        }
    }

    fn apply_vertical_door(&mut self, r1: &Rect, r2: &Rect, width : &usize) {
        let mut xs = vec!();
        for x in r1.x1+1 ..= r1.x2 {
            if x>r2.x1 && x<=r2.x2 {
                xs.push(x);
            }
        }
        while xs.len()>*width {
            xs.remove(0);
            if xs.len()>*width {
                xs.remove(xs.len()-1);
            }
        }
        for x in xs {
            for y in min(r1.y1,r2.y1)+1 .. max(r1.y2,r2.y2) {
                let idx = self.xy_idx(x, y);
                if idx < self.tiles.len() {
                    self.tiles[idx as usize].tile_type = TileType::Floor;
                }
            }
        }
    }

    pub fn is_visible(&self, x: i32, y: i32) -> bool {
        self.tiles[self.xy_idx(x,y)].visible
    }

    pub fn draw_map(ecs: &World, ctx : &mut Rltk) {
        ctx.set_active_console(0);
        let map = ecs.fetch::<Map>();
        let mut y = 0;
        let mut x = 0;
        for tile in map.tiles.iter() {
            // Render a tile depending upon the tile type
            if tile.revealed {
                let glyph;
                let mut fg;
                match tile.tile_type {
                    TileType::Floor => {
                        glyph = rltk::to_cp437('.');
                        fg = RGB::from_f32(0.0, 0.5, 0.5);
                    }
                    TileType::Wall => {
                        glyph = rltk::to_cp437('#');
                        fg = RGB::from_f32(0., 1.0, 0.);
                    }
                }
                if !tile.visible { 
                    fg = fg.to_greyscale()
                }
                ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
            }

            // Move the coordinates
            x += 1;
            if x > map.width-1 {
                x = 0;
                y += 1;
            }
        }
    }
}

impl Algorithm2D for Map {
    fn in_bounds(&self, pos : Point) -> bool {
        pos.x > 0 && pos.x < self.width-1 && pos.y > 0 && pos.y < self.height-1
    }

    fn point2d_to_index(&self, pt: Point) -> i32 {
        (pt.y * self.width) + pt.x
    }

    fn index_to_point2d(&self, idx:i32) -> Point {
        Point{ x: idx % self.width, y: idx / self.width }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx:i32) -> bool {
        self.tiles[idx as usize].tile_type == TileType::Wall
    }

    fn get_available_exits(&self, _idx:i32) -> Vec<(i32, f32)> {
        Vec::new()
    }

    fn get_pathing_distance(&self, idx1:i32, idx2:i32) -> f32 {
        let p1 = Point::new(idx1 % self.width, idx1 / self.width);
        let p2 = Point::new(idx2 % self.width, idx2 / self.width);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}