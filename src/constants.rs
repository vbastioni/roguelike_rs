use tcod::colors::Color;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;

pub const FPS_LIMIT: i32 = 20;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 40;

pub const ROOM_MAX_SIZE: i32 = 10;
pub const ROOM_MIN_SIZE: i32 = 6;
pub const MAX_ROOMS: i32 = 30;

pub const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
pub const COLOR_DARK_GROUND: Color = Color {
    r: 50,
    g: 50,
    b: 150,
};

pub const COLOR_DEBUG: Color = Color { r: 255, g: 0, b: 255, };
