use tcod::colors::{self, Color};
use tcod::console::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const FPS_LIMIT: i32 = 20;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 40;

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_GROUND: Color = Color { r: 50, g: 50, b: 150 };

#[derive(Debug, Copy, Clone)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile { blocked: false, block_sight: false, }
    }

    pub fn wall() -> Self {
        Tile { blocked: true, block_sight: true, }
    }
}

#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Self {
            x, y, char, color,
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        if !map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
            // move by the given amount
            self.x += dx;
            self.y += dy;
        }
    }

    /// set the color and then draw the character that represents this object
    /// at its position
    pub fn draw(&mut self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
}

type Map = Vec<Vec<Tile>>;

// TODO: Tuple struct to define cusom funcs
// impl Map {
//     pub fn is_blocked(&self, x: i32, y: i32) -> bool {
//         self[x as usize][y as usize].blocked
//     }
// }

fn make_map() -> Map {
    let mut map = vec![vec![Tile::empty(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

    map[30][22] = Tile::wall();
    map[50][22] = Tile::wall();

    map
}

fn render_all(root: &mut Root, con: &mut Offscreen, objects: &mut [Object], map: &Map) {
    // draw all objects in the list
    for object in objects.iter_mut() {
        object.draw(con);
    }

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let wall = map[x as usize][y as usize].block_sight;
            con.set_char_background(x, y, match wall {
                true => COLOR_DARK_WALL,
                false => COLOR_DARK_GROUND,
            }, BackgroundFlag::Set);
        }
    }

    blit(con, (0, 0), (MAP_WIDTH, MAP_HEIGHT), root, (0, 0), 1.0, 1.0);
}

fn main() {
    let player = Object::new(MAP_WIDTH / 2, MAP_HEIGHT / 2, '@', colors::WHITE);
    let npc = Object::new(MAP_WIDTH / 2 - 5, MAP_HEIGHT / 2, '@', colors::YELLOW);
    
    // the list of objects with those two
    let mut objects = [player, npc];

    let map = make_map();

    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libctod tutorial")
        .init();

    let mut con = Offscreen::new(MAP_WIDTH, MAP_HEIGHT);

    tcod::system::set_fps(FPS_LIMIT);

    while !root.window_closed() {
        // con.set_default_foreground(WHITE);
        con.clear();

        render_all(&mut root, &mut con, &mut objects, &map);

        root.flush();
        let player = &mut objects[0];
        let exit = handle_keys(&mut root, player, &map);
        if exit {
            break
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
        Key { code: Enter, alt: true, .. } => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        },
        Key { code: Escape, .. } => return true,
        _ => {},
    }

    false
}