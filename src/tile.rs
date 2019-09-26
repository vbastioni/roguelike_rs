use tcod::colors::{self, Color};

use crate::constants as cst;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    Wall,
    Floor,
    Passage,
}

#[derive(Debug, Copy, Clone)]
pub struct Colors {
    light: Color,
    dark: Color,
}

impl Colors {
    pub fn get(self, visible: bool) -> Color {
        if visible {
            self.light
        } else {
            self.dark
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
    pub explored: bool,
    pub colors: Colors,
    pub inner: Type,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            explored: false,
            blocked: false,
            block_sight: false,
            colors: Colors {
                dark: cst::COLOR_DARK_GROUND,
                light: cst::COLOR_LIGHT_GROUND,
            },
            inner: Type::Floor,
        }
    }

    pub fn wall() -> Self {
        Tile {
            explored: false,
            blocked: true,
            block_sight: true,
            colors: Colors {
                dark: cst::COLOR_DARK_WALL,
                light: cst::COLOR_LIGHT_WALL,
            },
            inner: Type::Wall,
        }
    }

    pub fn debug() -> Self {
        Tile {
            explored: false,
            blocked: false,
            block_sight: false,
            colors: Colors {
                dark: cst::COLOR_DEBUG,
                light: colors::LIGHTEST_FLAME,
            },
            inner: Type::Passage,
        }
    }

    pub fn get_color(&self, visible: bool) -> Color {
        self.colors.get(visible)
    }
}
