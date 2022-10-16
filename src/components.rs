use bracket_geometry::prelude::Point;
use specs::prelude::*;
use specs_derive::Component;

use super::TileGraphic;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub graphic: TileGraphic,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
}
