use std::cmp;
use std::ops;
use std::rc::Rc;
use std::sync::Mutex;

use rand::Rng;

use crate::cst;
use crate::object::Object;
use crate::tile::{Tile, Type};
use crate::Rect;

// pub struct Map(Vec<Vec<Tile>>);
pub struct Map(Vec<Tile>);

impl ops::Deref for Map {
    type Target = Vec<Tile>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Map {
    pub fn new(place_objects: impl Fn(&Rect)) -> (Self, (i32, i32)) {
        // fill the map with "blocked" tiles
        let mut map = Map(vec![
            Tile::wall();
            (cst::MAP_HEIGHT * cst::MAP_WIDTH) as usize
        ]);

        let mut starting_pos = None as Option<(i32, i32)>;
        let mut prev_room: Rect = Rect::new(0, 0, 0, 0);
        let mut rooms = Vec::new();

        for _ in 0..cst::MAX_ROOMS {
            let w = rand::thread_rng().gen_range(cst::ROOM_MIN_SIZE, cst::ROOM_MAX_SIZE + 1);
            let h = rand::thread_rng().gen_range(cst::ROOM_MIN_SIZE, cst::ROOM_MAX_SIZE + 1);

            let x = rand::thread_rng().gen_range(0, cst::MAP_WIDTH - w);
            let y = rand::thread_rng().gen_range(0, cst::MAP_HEIGHT - h);

            let new_room = Rect::new(x, y, w, h);
            let (new_x, new_y) = new_room.center();
            let failed = rooms
                .iter()
                .any(|other_room| new_room.intersects_with(other_room));

            if !failed {
                map.create_room(&new_room);
                place_objects(&new_room);

                if starting_pos.is_none() {
                    starting_pos = Some((new_x, new_y));
                } else {
                    // connect to the previous room
                    let (prev_x, prev_y) = prev_room.center();
                    if rand::random() {
                        map.create_h_tunnel(prev_x, new_x, prev_y);
                        map.create_v_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.create_v_tunnel(prev_y, new_y, prev_x);
                        map.create_h_tunnel(prev_x, new_x, new_y);
                    }
                }
                rooms.push(new_room);
                prev_room = new_room;
            }
        }

        (map, starting_pos.expect("should be set"))
    }

    pub fn move_object(
        &self,
        objects_lock: &Rc<Mutex<Vec<Object>>>,
        index: usize,
        delta: (i32, i32),
    ) {
        let (x, y) = {
            let objects = objects_lock.lock().unwrap();
            objects[index].pos()
        };
        let (dx, dy) = delta;
        let (nx, ny) = (dx + x, dy + y);
        if !self.is_blocked(nx, ny, objects_lock) {
            let mut objects = objects_lock.lock().unwrap();
            objects[index].move_by(dx, dy)
        }
    }

    pub fn set(&mut self, x: i32, y: i32, t: Tile) {
        *self.get_mut(x, y) = t;
    }

    pub fn get(&self, x: i32, y: i32) -> Tile {
        self.0[(x + y * cst::MAP_WIDTH) as usize]
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> &mut Tile {
        &mut self.0[(x + y * cst::MAP_WIDTH) as usize]
    }

    pub fn is_blocked(&self, x: i32, y: i32, objects_lock: &Rc<Mutex<Vec<Object>>>) -> bool {
        if self.get(x, y).blocked {
            true
        } else {
            let objects = objects_lock.lock().unwrap();
            objects.iter().any(|o| o.blocks() && o.pos() == (x, y))
        }
    }

    fn create_room(&mut self, room: &Rect) {
        for x in (room.x1 + 1)..(room.x2) {
            for y in (room.y1 + 1)..(room.y2) {
                self.set(x, y, Tile::empty());
            }
        }
    }

    fn create_h_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in cmp::min(x1, x2)..(cmp::max(x1, x2) + 1) {
            if self.get(x, y).inner == crate::tile::Type::Wall {
                *self.get_mut(x, y) = Tile::debug();
            }
        }
    }

    fn create_v_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in cmp::min(y1, y2)..(cmp::max(y1, y2) + 1) {
            if self.get(x, y).inner == crate::tile::Type::Wall {
                *self.get_mut(x, y) = Tile::debug();
            }
        }
    }

    pub fn index_to_pos(i: i32) -> (i32, i32) {
        (i % cst::MAP_WIDTH, i / cst::MAP_WIDTH)
    }

    pub fn explored_count(&self) -> (i32, i32) {
        let explorables = self
            .0
            .iter()
            .filter(|t| match t.inner {
                Type::Floor | Type::Passage => true,
                _ => false,
            })
            .collect::<Vec<_>>();
        let explored = explorables.iter().filter(|t| t.explored).count() as i32;
        let total = explorables.len() as i32;
        (explored, total)
    }
}
