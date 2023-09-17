use specs::prelude::*;

use damage::DamageSystem;
use inventory::InventorySystem;
use map_indexing::MapIndexingSystem;
use melee_combat::MeleeCombatSystem;
use monster_ai::MonsterAI;
use movement::MovementSystem;
use visibility::VisibilitySystem;

pub mod damage;
pub mod inventory;
pub mod map_indexing;
pub mod melee_combat;
pub mod monster_ai;
pub mod movement;
pub mod visibility;

pub fn run(ecs: &World) {
    let mut vis = VisibilitySystem {};
    vis.run_now(ecs);

    let mut mapindex = MapIndexingSystem {};
    mapindex.run_now(ecs);

    let mut mob = MonsterAI {};
    mob.run_now(ecs);

    let mut movement = MovementSystem {};
    movement.run_now(ecs);

    let mut melee = MeleeCombatSystem {};
    melee.run_now(ecs);

    let mut damage = DamageSystem {};
    damage.run_now(ecs);

    let mut inventory_system = InventorySystem {};
    inventory_system.run_now(ecs);

    vis.run_now(ecs);
    mapindex.run_now(ecs);
}
