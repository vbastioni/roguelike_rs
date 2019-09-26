use std::ops::{Deref,DerefMut};

use tcod::colors::Color;
use tcod::console::*;

use crate::constants as cst;

pub struct Objects(Vec<Object>);

impl Objects {
    pub fn new(player: Object) -> Self {
        Objects(vec![player])
    }

    pub fn player(&self) -> &Object {
        &self.0[cst::PLAYER_POS]
    }

    pub fn monsters(&self) -> &[Object] {
        &self.0[(cst::PLAYER_POS + 1)..]
    }

    pub fn player_mut(&mut self) -> &mut Object {
        &mut self.0[cst::PLAYER_POS]
    }

    pub fn set_player_pos(&mut self, pos: (i32, i32)) {
        self.player_mut().set_pos(pos)
    }
}

impl Deref for Objects {
    type Target = Vec<Object>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Objects {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
    _name: String,
    _blocks: bool,
    _alive: bool,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color, name: &str, blocks: bool) -> Self {
        Self { x, y, char, color, _name: name.into(), _blocks: blocks, _alive: true, }
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

    pub fn name(&self) -> &str {
        &self._name
    }
}
