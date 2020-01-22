use std::ops::{Deref, DerefMut};

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fighter {
    max_hp: i32,
    hp: i32,
    defense: i32,
    power: i32,
}

impl Fighter {
    pub fn new(hp: i32, defense: i32, power: i32) -> Self {
        Fighter {
            max_hp: hp,
            hp,
            defense,
            power,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AI;

#[derive(Debug)]
pub struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
    _name: String,
    _blocks: bool,
    _alive: bool,
    _fighter: Option<Fighter>,
    _ai: Option<AI>,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color, name: &str, blocks: bool) -> Self {
        Self {
            x: x,
            y: y,
            char: char,
            color: color,
            _name: name.into(),
            _blocks: blocks,
            _alive: true,
            _fighter: None,
            _ai: None,
        }
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        // move by the given amount
        self.x += dx;
        self.y += dy;
    }

    pub fn distance_to(&self, other: &Object) -> (i32, i32, f32) {
        self.distance_to_pos(other.pos())
    }

    pub fn distance_to_pos(&self, pos: (i32, i32)) -> (i32, i32, f32) {
        let (x, y) = pos;
        let dx = x - self.x;
        let dy = y - self.y;
        let distance = ((dx.pow(2) + dy.pow(2)) as f32).sqrt();
        (dx, dy, distance)
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

    pub fn set_fighter(&mut self, fgh: Fighter) {
        self._fighter = Some(fgh)
    }

    pub fn set_ai(&mut self, ai: AI) {
        self._ai = Some(ai)
    }

    pub fn ai(&self) -> &Option<AI> {
        &self._ai
    }
}
