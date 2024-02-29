pub use bracket_geometry::prelude::Point;
use specs::prelude::*;
use specs_derive::Component;
use units::Position2D;

use crate::map::TileGraphic;
use crate::world::units::WorldUnits;

#[derive(Component, Debug, Clone)]
pub struct Name {
    pub name: String,
}

#[derive(Component)]
pub struct Renderable {
    pub graphic: TileGraphic,
}

#[derive(Component, Debug, Clone)]
pub struct Player {}

#[derive(Component, Debug, Clone)]
pub struct Monster {}

#[derive(Component, Debug, Clone)]
pub struct Item {}

#[derive(Component, Debug, Clone)]
pub struct BlocksTile {}

#[derive(Component, Debug, Clone)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Debug, Clone)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

#[derive(Component, Debug, Clone)]
pub struct WantsToMelee {
    pub target: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct SufferDamage {
    pub amount: Vec<i32>,
}

// FIXME: restore ConvertSaveload
#[derive(Component, Debug, Clone)]
pub struct WantsToMove {
    // FIXME: add Map
    pub destination: WorldPosition2D,
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

#[derive(Component, Debug, Clone)]
pub struct HealthRestore {
    pub heal_amount: i32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct WorldPosition2D {
    pub x: i32,
    pub y: i32,
}

impl WorldPosition2D {
    pub fn to_world_units(&self) -> Position2D<WorldUnits> {
        WorldUnits::new_position2d(self.x, self.y)
    }

    pub fn from_world_units(position: Position2D<WorldUnits>) -> Self {
        Self {
            x: position.x.to_primitive(),
            y: position.y.to_primitive(),
        }
    }

    pub fn to_point(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    pub fn from_point(point: Point) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
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
    ecs.register::<WorldPosition2D>();
}
