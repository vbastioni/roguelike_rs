use std::ops;
use std::rc::Rc;
use std::sync::Mutex;

use tcod::colors::Color;
use tcod::console::*;

use crate::map::Map;

#[derive(Debug)]
pub struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
    name: String,
    _blocks: bool,
    _alive: bool,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color, name: &str, blocks: bool) -> Self {
        Self { x, y, char, color, name: name.into(), _blocks: blocks, _alive: true, }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        // move by the given amount
        self.x += dx;
        self.y += dy;
    }

    /// set the color and then draw the character that represents this object
    /// at its position
    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_pos(&mut self, pos: (i32, i32)) {
        let (x, y) = pos;
        self.x = x;
        self.y = y;
    }

    pub fn blocks(&self) -> bool {
        self._blocks
    }

    pub fn alive(&self) -> bool {
        self._alive
    }
}
