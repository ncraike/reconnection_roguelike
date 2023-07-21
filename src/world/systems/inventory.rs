use bracket_geometry::prelude::Point;
use specs::prelude::*;

use super::super::super::components::{InInventory, Name, WantsToPickupItem};
use super::super::super::message_log::MessageLog;

pub struct InventorySystem {}

impl<'a> System<'a> for InventorySystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, MessageLog>,
        WriteStorage<'a, WantsToPickupItem>,
        WriteStorage<'a, Point>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, InInventory>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (player_entity, mut messages, mut wants_to_pickup, mut points, names, mut inventories) =
            data;

        for pickup in wants_to_pickup.join() {
            points.remove(pickup.item);
            inventories
                .insert(
                    pickup.item,
                    InInventory {
                        owner: pickup.collected_by,
                    },
                )
                .expect("Unable to add item to inventory");

            if pickup.collected_by == *player_entity {
                messages.entries.push(format!(
                    "You pickup the {}.",
                    names.get(pickup.item).unwrap().name
                ));
            }
        }
        wants_to_pickup.clear();
    }
}
