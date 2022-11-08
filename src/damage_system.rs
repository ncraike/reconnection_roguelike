use super::components::{CombatStats, Player, SufferDamage};
use bracket_terminal::prelude::console;
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
        let players_store = ecs.read_storage::<Player>();
        let entities = ecs.entities();
        for (entity, stats) in (&entities, &combat_stats_store).join() {
            if stats.hp < 1 {
                let maybe_player = players_store.get(entity);
                match maybe_player {
                    None => dead.push(entity),
                    Some(_) => console::log("You are dead"),
                }
            }
        }
    }

    for victim in dead {
        ecs.delete_entity(victim)
            .expect("Unable to delete dead entity");
    }
}
