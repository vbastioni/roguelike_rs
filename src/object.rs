use tcod::colors::Color;
use tcod::console::*;

use crate::map::Map;

#[derive(Debug)]
pub struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Self { x, y, char, color }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32, map: &Map) {
        // if !map[(self.x + dx) as usize][(self.y + dy) as usize].blocked {
        if !map.is_blocked(self.x + dx, self.y + dy) {
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
