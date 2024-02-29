use bracket_color::prelude::{ColorPair, RGB};
use bracket_terminal::prelude::DrawBatch;
use specs::prelude::*;
use units::{Box2D, Position2D};

use crate::ui::units::ScreenChars;
use crate::world::units::WorldUnits;

use super::super::components::Name;
use super::super::map::Map;

use super::colors;
use super::common::Consoles;

const TEXT_COLOR: RGB = colors::BLUE_LIGHT;

pub fn render_tooltips(
    batch: &mut DrawBatch,
    ecs: &World,
    camera_view: Box2D<ScreenChars>,
    camera_in_world: Box2D<WorldUnits>,
    mouse_position: Position2D<ScreenChars>,
) {
    let names = ecs.read_storage::<Name>();
    let map = ecs.fetch::<Map>();

    if !camera_view.contains(mouse_position) {
        // Pointing out of camera
        return;
    }

    let mouse_pos_as_world_offset = WorldUnits::new_size2d(
        mouse_position.x.to_primitive(),
        mouse_position.y.to_primitive(),
    );

    let mouse_pos_in_world = camera_in_world.p1 + mouse_pos_as_world_offset;
    if !map.bounds().contains(mouse_pos_in_world) {
        // Pointing out of map
        return;
    }

    let mouse_pos_as_index = map.to_index(mouse_pos_in_world);
    if !map.visible_tiles[mouse_pos_as_index] {
        // Point out of player vision
        return;
    }

    let entities_at_tile = map.tile_content[mouse_pos_as_index].clone();
    if entities_at_tile.is_empty() {
        // Not pointing at an entity
        return;
    }

    let mut tooltips: Vec<String> = Vec::new();

    batch.target(Consoles::UIText as usize);

    for entity in entities_at_tile.iter() {
        let maybe_name = names.get(*entity);
        if let Some(name) = maybe_name {
            tooltips.push(name.name.to_string());
        }
    }
    for tooltip in tooltips.iter() {
        let tooltip_pos = mouse_position + ScreenChars::new_size2d(4, 0);
        batch.print_color(
            tooltip_pos.to_bracket_geometry_point(),
            tooltip,
            ColorPair {
                fg: TEXT_COLOR.to_rgba(1.0),
                bg: colors::TRANSPARENT,
            },
        );
    }

    batch
        .submit(Consoles::UIText as usize)
        .expect("Couldn't render text");
}
