use std::rc::Rc;
use std::sync::Mutex;

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
use object::Object;
use rect::Rect;
use tile::Tile;

use crate::constants as cst;

fn main() {
    let (map, (start_x, start_y)) = Map::new();
    let player = Object::new(start_x, start_y, '@', colors::WHITE);
    let npc = Object::new(
        cst::MAP_WIDTH / 2 - 5,
        cst::MAP_HEIGHT / 2,
        '@',
        colors::YELLOW,
    );
    // the list of objects with those two
    let mut objects = [player, npc];

    let fov_map = FovMap::new(cst::MAP_WIDTH, cst::MAP_HEIGHT);
    let fov_map_lock = Rc::new(Mutex::new(fov_map));
    Cb::new_2d(cst::MAP_WIDTH as usize, cst::MAP_HEIGHT as usize).for_each(|t| {
        let (x, y) = t;
        let tx = *x as i32;
        let ty = *y as i32;
        let tile = map.get(tx, ty);
        let mut data = fov_map_lock.lock().unwrap();
        data.set(tx, ty, !tile.block_sight, !tile.blocked);
    });

    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(cst::SCREEN_WIDTH, cst::SCREEN_HEIGHT)
        .title("Rust/libctod tutorial")
        .init();

    let mut con = Offscreen::new(cst::MAP_WIDTH, cst::MAP_HEIGHT);

    tcod::system::set_fps(cst::FPS_LIMIT);

    let mut previous_player_position = (-1, -1);
    while !root.window_closed() {
        // con.set_default_foreground(WHITE);
        con.clear();

        let fov_recompute = previous_player_position != objects[0].pos();
        render_all(
            &mut root,
            &mut con,
            &mut objects,
            &map,
            &fov_map_lock,
            fov_recompute,
        );

        root.flush();
        let player = &mut objects[0];
        previous_player_position = player.pos();
        let exit = handle_keys(&mut root, player, &map);
        if exit {
            break;
        }
    }
}

fn handle_keys(root: &mut Root, player: &mut Object, map: &Map) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Up, .. }
        | Key {
            code: Char,
            printable: 'w',
            ..
        } => player.move_by(0, -1, map),
        Key { code: Down, .. }
        | Key {
            code: Char,
            printable: 's',
            ..
        } => player.move_by(0, 1, map),
        Key { code: Left, .. }
        | Key {
            code: Char,
            printable: 'a',
            ..
        } => player.move_by(-1, 0, map),
        Key { code: Right, .. }
        | Key {
            code: Char,
            printable: 'd',
            ..
        } => player.move_by(1, 0, map),
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,
        _ => {},
    }

    false
}

fn render_all(
    root: &mut Root,
    con: &mut Offscreen,
    objects: &mut [Object],
    map: &Map,
    fov_map: &std::rc::Rc<std::sync::Mutex<tcod::map::Map>>,
    fov_recompute: bool,
) {
    let mut data = fov_map.lock().unwrap();
    if fov_recompute {
        // recompute FOV if needed (the player moved or something)
        let (x, y) = objects[0].pos();
        data.compute_fov(x, y, cst::TORCH_RADIUS, cst::FOV_LIGHT_WALLS, cst::FOV_ALGO);
    }

    objects
        .iter()
        .filter(|o| {
            let (x, y) = o.pos();
            data.is_in_fov(x, y)
        })
        .for_each(|o| o.draw(con));

    for (i, t) in map.map().iter().enumerate() {
        let (x, y) = Map::index_to_pos(i as i32);
        let visible = data.is_in_fov(x, y);
        con.set_char_background(x, y, t.colors.get(visible), BackgroundFlag::Set);
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
