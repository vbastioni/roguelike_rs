use std::cmp;

use tcod::colors::{self, Color};
use tcod::console::*;

mod constants;
mod map;
mod rect;
mod tile;
mod object;

use map::Map;
use rect::Rect;
use tile::Tile;
use object::Object;

use crate::constants as cst;

fn main() {
    let (map, (start_x, start_y)) = Map::new();
    let player = Object::new(start_x, start_y, '@', colors::WHITE);
    let npc = Object::new(cst::MAP_WIDTH / 2 - 5, cst::MAP_HEIGHT / 2, '@', colors::YELLOW);
    
    // the list of objects with those two
    let mut objects = [player, npc];

    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(cst::SCREEN_WIDTH, cst::SCREEN_HEIGHT)
        .title("Rust/libctod tutorial")
        .init();

    let mut con = Offscreen::new(cst::MAP_WIDTH, cst::MAP_HEIGHT);

    tcod::system::set_fps(cst::FPS_LIMIT);

    while !root.window_closed() {
        // con.set_default_foreground(WHITE);
        con.clear();

        render_all(&mut root, &mut con, &mut objects, &map);

        root.flush();
        let player = &mut objects[0];
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
        Key { code: Up, .. } => player.move_by(0, -1, map),
        Key { code: Down, .. } => player.move_by(0, 1, map),
        Key { code: Left, .. } => player.move_by(-1, 0, map),
        Key { code: Right, .. } => player.move_by(1, 0, map),
        Key {
            code: Enter,
            alt: true,
            ..
        } => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,
        _ => {}
    }

    false
}

fn render_all(root: &mut Root, con: &mut Offscreen, objects: &mut [Object], map: &Map) {
    // draw all objects in the list
    for object in objects.iter_mut() {
        object.draw(con);
    }

    for (i, t) in map.map().iter().enumerate() {
        let (x, y) = Map::index_to_pos(i as i32);
        con.set_char_background(x, y, t.color, BackgroundFlag::Set);
    }

    blit(con, (0, 0), (cst::MAP_WIDTH, cst::MAP_HEIGHT), root, (0, 0), 1.0, 1.0);
}
