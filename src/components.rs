use specs::prelude::*;
use specs_derive::Component;

use super::{TileGraphic, TileLayer};

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub graphic: TileGraphic,
    pub layer: TileLayer,
}

#[derive(Component, Debug)]
pub struct Player {}
