use bracket_color::prelude::{ColorPair, RGB};
use bracket_geometry::prelude::{Point, Rect};
use bracket_terminal::prelude::DrawBatch;
use specs::prelude::*;

use super::super::components::Name;
use super::super::map::Map;

use super::colors;
use super::common::Consoles;

const TEXT_COLOR: RGB = colors::BLUE_LIGHT;

pub fn render_tooltips(
    batch: &mut DrawBatch,
    ecs: &World,
    camera_view: Rect,
    camera_in_world: Rect,
    mouse_pt_in_tiles: Point,
    mouse_pt_in_text: Point,
) {
    let names = ecs.read_storage::<Name>();
    let map = ecs.fetch::<Map>();

    if !camera_view.point_in_rect(mouse_pt_in_tiles) {
        // Pointing out of camera
        return;
    }

    let mouse_pt_in_world: Point = mouse_pt_in_tiles
        + Point {
            x: camera_in_world.x1,
            y: camera_in_world.y1,
        };
    if !map.bounds().point_in_rect(mouse_pt_in_world) {
        // Pointing out of map
        return;
    }

    let mouse_pt_as_index = map.to_index(mouse_pt_in_world);
    if !map.visible_tiles[mouse_pt_as_index] {
        // Point out of player vision
        return;
    }

    let entities_at_tile = map.tile_content[mouse_pt_as_index].clone();
    if entities_at_tile.is_empty() {
        // Not pointing at an entity
        return;
    }

    let mut tooltips: Vec<String> = Vec::new();

    batch.target(Consoles::Text as usize);

    for entity in entities_at_tile.iter() {
        let maybe_name = names.get(*entity);
        if let Some(name) = maybe_name {
            tooltips.push(name.name.to_string());
        }
    }
    for tooltip in tooltips.iter() {
        batch.print_color(
            mouse_pt_in_text + Point { x: 4, y: 0 },
            tooltip,
            ColorPair {
                fg: TEXT_COLOR.to_rgba(1.0),
                bg: colors::TRANSPARENT,
            },
        );
    }

    batch
        .submit(Consoles::Text as usize)
        .expect("Couldn't render text");
}
