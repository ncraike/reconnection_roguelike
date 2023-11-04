use bracket_geometry::prelude::Point;
use specs::prelude::*;

use super::components::{Item, WantsToPickupItem};
use super::message_log::MessageLog;

fn get_item(ecs: &mut World) {
    let entities = ecs.entities();
    let player_entity = ecs.fetch::<Entity>();
    let mut messages = ecs.fetch_mut::<MessageLog>();

    let item_store = ecs.read_storage::<Item>();
    let point_store = ecs.read_storage::<Point>();
    let player_pos = point_store
        .get(*player_entity)
        .expect("Could not get player position");

    let mut target_item: Option<Entity> = None;
    for (item_entity, _item, pos) in (&entities, &item_store, &point_store).join() {
        if pos == player_pos {
            target_item = Some(item_entity);
        }
    }

    match target_item {
        None => messages
            .entries
            .push("There is nothing here to pick up.".to_string()),
        Some(item) => {
            let mut wants_to_pickup_store = ecs.write_storage::<WantsToPickupItem>();
            wants_to_pickup_store
                .insert(
                    *player_entity,
                    WantsToPickupItem {
                        collected_by: *player_entity,
                        item: item,
                    },
                )
                .expect("Unable to add want-to-pickup");
        }
    }
}
