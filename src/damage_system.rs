use super::components::{CombatStats, SufferDamage};
use specs::prelude::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats_store, mut damage_store) = data;
        for (mut stats, damage) in (&mut stats_store, &damage_store).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
        }

        damage_store.clear();
    }
}

pub fn delete_the_dead(ecs: &mut World) {
    let mut dead: Vec<Entity> = Vec::new();

    {
        let combat_stats_store = ecs.read_storage::<CombatStats>();
        let entities = ecs.entities();
        for (entity, stats) in (&entities, &combat_stats_store).join() {
            if stats.hp < 1 {
                dead.push(entity);
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim)
            .expect("Unable to delete dead entity");
    }
}
