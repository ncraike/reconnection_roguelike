pub use bracket_geometry::prelude::Point;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};
use specs_derive::{Component, ConvertSaveload};
use std::convert::Infallible as NoError;

use super::map::TileGraphic;

#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Component)]
pub struct Renderable {
    pub graphic: TileGraphic,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component, Debug)] 
pub struct Monster {}

#[derive(Component, Debug)]
pub struct Item {}

#[derive(Component, Debug)]
pub struct BlocksTile {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, Debug)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct WantsToMove {
    // FIXME: add Map
    pub destination: Point,
}

impl SufferDamage {
    pub fn new_damage(store: &mut WriteStorage<SufferDamage>, victim: Entity, amount: i32) {
        if let Some(suffering) = store.get_mut(victim) {
            suffering.amount.push(amount);
        } else {
            let dmg = SufferDamage {
                amount: vec![amount],
            };
            store.insert(victim, dmg).expect("Unable to insert damage");
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct InInventory {
    pub owner: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToPickupItem {
    pub collected_by: Entity,
    pub item: Entity,
}

#[derive(Component, Debug)]
pub struct HealthRestore {
    pub heal_amount: i32,
}

pub fn register_components(ecs: &mut World) {
    ecs.register::<BlocksTile>();
    ecs.register::<CombatStats>();
    ecs.register::<InInventory>();
    ecs.register::<Item>();
    ecs.register::<HealthRestore>();
    ecs.register::<Monster>();
    ecs.register::<Name>();
    ecs.register::<Player>();
    ecs.register::<Point>();
    ecs.register::<Renderable>();
    ecs.register::<SufferDamage>();
    ecs.register::<Viewshed>();
    ecs.register::<WantsToMelee>();
    ecs.register::<WantsToMove>();
    ecs.register::<WantsToPickupItem>();
}
