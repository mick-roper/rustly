use specs::prelude::*;
use specs_derive::*;
use rltk::{RGB};

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component)]
pub struct Monster {}

#[derive(Component)]
pub struct Named {
    pub name: String
}

#[derive(Component, Debug)]
pub struct BlocksTile {}

#[derive(Component)]
pub struct CombatStats {
    pub max_hp: i32,
    pub current_hp: i32,
    pub defence: i32,
    pub power: i32,
}