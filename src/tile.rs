use tcod::colors::Color;

use crate::constants as cst;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    Wall,
    Floor,
    Passage,
}

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
    pub color: Color,
    pub inner: Type,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            color: cst::COLOR_DARK_GROUND,
            inner: Type::Floor,
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
            color: cst::COLOR_DARK_WALL,
            inner: Type::Wall,
        }
    }

    pub fn debug() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
            color: cst::COLOR_DEBUG,
            inner: Type::Passage,
        }
    }
}
