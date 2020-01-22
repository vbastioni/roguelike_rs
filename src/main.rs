use std::rc::Rc;
use std::sync::Mutex;

use rand::Rng;
use tcod::colors;
use tcod::console::*;
use tcod::map::Map as FovMap;

mod combinations;
mod constants;
mod map;
mod object;
mod rect;
mod tile;

use combinations::Combination as Cb;
use map::Map;
use object::{Object, Objects, Fighter, AI};
use rect::Rect;

use crate::constants as cst;

static mut DISABLED_FOV: bool = false;

#[derive(Copy, Clone, Debug, PartialEq)]
enum PlayerAction {
    TookTurn,
    DidntTakeTurn,
    Exit,
}

fn init() -> (Rc<Mutex<Objects>>, Map, Rc<Mutex<FovMap>>, Root, Offscreen) {
    let mut player = Object::new(-1, -1, '@', colors::WHITE, "player", true);
    player.set_fighter(Fighter::new(30, 2, 5));

    let objects = Objects::new(player);
    let objects_lock = Rc::new(Mutex::new(objects));

    let (map, (start_x, start_y)) = Map::new(|r| place_objects(r, &objects_lock));
    {
        let mut objects = objects_lock.lock().unwrap();
        objects.set_player_pos((start_x, start_y));
    }

    let fov_map = FovMap::new(cst::MAP_WIDTH, cst::MAP_HEIGHT);
    let fov_map_lock = Rc::new(Mutex::new(fov_map));
    Cb::new_2d(cst::MAP_WIDTH as usize, cst::MAP_HEIGHT as usize).for_each(|t| {
        let (x, y) = t;
        let tx = *x as i32;
        let ty = *y as i32;
        let tile = map.get(tx, ty);
        fov_map_lock
            .lock()
            .unwrap()
            .set(tx, ty, !tile.block_sight, !tile.blocked);
    });

    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(cst::SCREEN_WIDTH, cst::SCREEN_HEIGHT)
        .title("Rust/libctod tutorial")
        .init();

    let con = Offscreen::new(cst::MAP_WIDTH, cst::MAP_HEIGHT);

    tcod::system::set_fps(cst::FPS_LIMIT);

    (objects_lock, map, fov_map_lock, root, con)
}

fn main() {
    let (objects_lock, mut map, fov_map_lock, mut root, mut con) = init();

    let mut previous_player_position = (-1, -1);
    while !root.window_closed() {
        let player_pos = objects_lock.lock().unwrap().player().pos();
        let fov_recompute = previous_player_position != player_pos;
        con.clear();

        render_all(
            &mut root,
            &mut con,
            &objects_lock,
            &mut map,
            &fov_map_lock,
            fov_recompute,
        );

        root.flush();
        previous_player_position = player_pos;
        let player_action = handle_keys(&mut root, &map, &objects_lock);
        if player_action == PlayerAction::Exit {
            break;
        }
        let (explored, total) = map.explored_count();
        println!(
            "explored tiles: {} / {} ({:.3}%)",
            explored,
            total,
            (explored as f32 / total as f32) * 100f32
        );

        {
            let objects = objects_lock.lock().unwrap();
            if objects.player().alive() && player_action != PlayerAction::DidntTakeTurn {
                for (i, _) in objects.monsters().iter().enumerate().filter(|(_, o)| o.ai().is_some()) {
                    ai_take_turn(i, &map, &objects_lock, &fov_map_lock);
                    // only if object is not player
                    // println!("The {} growls!", object.name());
                }
            }
        }
    }
}

enum EnemyAction {
    Attack(usize, usize),
    Idle,
    Move(i32, i32),
}

fn ai_take_turn(monster_id: usize, map: &Map, objects_lock: &Rc<Mutex<Objects>>, fov_map_lock: &Rc<Mutex<FovMap>>) {
    match {
        let objects = objects_lock.lock().unwrap();
        let monster = &objects[monster_id];
        let player = &objects[cst::PLAYER_POS];
        let (x, y) = monster.pos();
        let fov_map = fov_map_lock.lock().unwrap();
        if !fov_map.is_in_fov(x, y) { EnemyAction::Idle }
        else {
            let distance = monster.distance_to(player).2;
            if distance >= 2.0f32 {
                let (px, py) = player.pos();
                EnemyAction::Move(px, py)
            } else if true {
                EnemyAction::Attack(monster_id, cst::PLAYER_POS)
            } else {
                EnemyAction::Idle
            }
        }
    } {
        EnemyAction::Attack(monster_id, player_id) => {
            let objects = objects_lock.lock().unwrap();
            let monster = &objects[monster_id];
            let player = &objects[player_id];

            println!("The attack of {} bounces on {}'s armor!", monster.name(), player.name())
        },
        EnemyAction::Move(x, y) => {
            let mut objects = objects_lock.lock().unwrap();
            objects[monster_id].move_by(x, y);
        },
        EnemyAction::Idle => {},
    }
}

fn handle_keys(root: &mut Root, map: &Map, objects_lock: &Rc<Mutex<Objects>>) -> PlayerAction {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;
    use PlayerAction::*;
    let player_alive = {
        let objects = objects_lock.lock().unwrap();
        objects[cst::PLAYER_POS].alive()
    };
    let key = root.wait_for_keypress(true);
    match (key, player_alive) {
        (Key { code: Up, .. }, true) | (Key { code: Char, printable: 'w', .. }, true) => {
            map.move_or_attack_object(objects_lock, cst::PLAYER_POS, (0, -1));
            TookTurn
        },
        (Key { code: Down, .. }, true) | (Key { code: Char, printable: 's', .. }, true) => {
            map.move_or_attack_object(objects_lock, cst::PLAYER_POS, (0, 1));
            TookTurn
        },
        (Key { code: Left, .. }, true) | (Key { code: Char, printable: 'a', .. }, true) => {
            map.move_or_attack_object(objects_lock, cst::PLAYER_POS, (-1, 0));
            TookTurn
        },
        (Key { code: Right, .. }, true) | (Key { code: Char, printable: 'd', .. }, true) => {
            map.move_or_attack_object(objects_lock, cst::PLAYER_POS, (1, 0));
            TookTurn
        },
        (Key { code: Enter, alt: true, .. }, _) => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
            DidntTakeTurn
        },
        (Key { code: Escape, .. }, _) => Exit,
        (Key { code: Char, printable: 'p', .. }, _) => {
            unsafe { DISABLED_FOV = !DISABLED_FOV };
            DidntTakeTurn
        },
        _ => DidntTakeTurn,
    }
}

fn render_all(
    root: &mut Root,
    con: &mut Offscreen,
    objects_lock: &std::rc::Rc<std::sync::Mutex<Objects>>,
    map: &mut Map,
    fov_map: &std::rc::Rc<std::sync::Mutex<tcod::map::Map>>,
    fov_recompute: bool,
) {
    let mut data = fov_map.lock().unwrap();
    let objects = objects_lock.lock().unwrap();

    if fov_recompute {
        // recompute FOV if needed (the player moved or something)
        let (x, y) = objects[cst::PLAYER_POS].pos();
        data.compute_fov(x, y, cst::TORCH_RADIUS, cst::FOV_LIGHT_WALLS, cst::FOV_ALGO);
    }

    // draw each other object
    objects
        .iter()
        .filter(|o| {
            let (x, y) = o.pos();
            data.is_in_fov(x, y)
        })
        .for_each(|o| o.draw(con));

    // draw player
    objects[cst::PLAYER_POS].draw(con);

    for (i, t) in map.iter_mut().enumerate() {
        let (x, y) = Map::index_to_pos(i as i32);
        let visible = data.is_in_fov(x, y);
        if visible {
            t.explored = true;
        }
        // if t.explored || disabled_fov {
        if t.explored || unsafe { DISABLED_FOV } {
            con.set_char_background(x, y, t.get_color(visible), BackgroundFlag::Set);
        }
    }

    blit(
        con,
        (0, 0),
        (cst::MAP_WIDTH, cst::MAP_HEIGHT),
        root,
        (0, 0),
        1.0,
        1.0,
    );
}

fn place_objects(room: &Rect, objects_lock: &std::rc::Rc<std::sync::Mutex<Objects>>) {
    // choose a random number of monsters
    let num_monsters = rand::thread_rng().gen_range(0, cst::MAX_ROOM_MONSTERS + 1);
    for _ in 0..num_monsters {
        let mut objects = objects_lock.lock().unwrap();
        let x = rand::thread_rng().gen_range(room.x1 + 1, room.x2);
        let y = rand::thread_rng().gen_range(room.y1 + 1, room.y2);

        objects.push(if rand::random::<f32>() < 0.8 {
            let mut orc = Object::new(x, y, 'o', colors::DESATURATED_GREEN, "orc", true);
            orc.set_fighter(Fighter::new(10, 0, 3));
            orc.set_ai(AI{});
            orc
        } else {
            let mut troll = Object::new(x, y, 'T', colors::DARKER_GREEN, "troll", true);
            troll.set_fighter(Fighter::new(16, 1, 4));
            troll.set_ai(AI{});
            troll
        });
    }
}
