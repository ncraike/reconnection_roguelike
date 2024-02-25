use specs::prelude::*;

use super::super::components::{
    BlocksTile, CombatStats, HealthRestore, Item, Monster, Name, Player, Renderable, Viewshed,
    WorldPosition2D,
};
use super::super::map::TileGraphic;

pub fn create_player(ecs: &mut World, at: WorldPosition2D) -> Entity {
    ecs.create_entity()
        .with(Player {})
        .with(Name {
            name: String::from("Player"),
        })
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .with(at.clone())
        .with(Renderable {
            graphic: TileGraphic::PlayerCharacter,
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build()
}

pub fn create_enemy_hound(ecs: &mut World, at: WorldPosition2D) -> Entity {
    ecs.create_entity()
        .with(Monster {})
        .with(Name {
            name: String::from("H-32"),
        })
        .with(CombatStats {
            max_hp: 16,
            hp: 16,
            defense: 1,
            power: 4,
        })
        .with(BlocksTile {})
        .with(at.clone())
        .with(Renderable {
            graphic: TileGraphic::EnemyHound,
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build()
}

pub fn create_enemy_big_stalker(ecs: &mut World, at: WorldPosition2D) -> Entity {
    ecs.create_entity()
        .with(Monster {})
        .with(Name {
            name: String::from("S-07"),
        })
        .with(CombatStats {
            max_hp: 16,
            hp: 16,
            defense: 1,
            power: 4,
        })
        .with(BlocksTile {})
        .with(at.clone())
        .with(Renderable {
            graphic: TileGraphic::EnemyBigStalker,
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build()
}

pub fn create_bandage(ecs: &mut World, at: WorldPosition2D) -> Entity {
    ecs.create_entity()
        .with(Item {})
        .with(Name {
            name: String::from("bandage"),
        })
        .with(Renderable {
            graphic: TileGraphic::ItemBandage,
        })
        .with(HealthRestore { heal_amount: 8 })
        .with(at.clone())
        .build()
}

pub fn create_first_aid_kit(ecs: &mut World, at: WorldPosition2D) -> Entity {
    ecs.create_entity()
        .with(Item {})
        .with(Name {
            name: String::from("first aid kit"),
        })
        .with(Renderable {
            graphic: TileGraphic::ItemFirstAidKit,
        })
        .with(HealthRestore { heal_amount: 20 })
        .with(at.clone())
        .build()
}
