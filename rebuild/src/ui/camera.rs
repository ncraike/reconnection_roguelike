use bracket_color::prelude::ColorPair;
use bracket_terminal::prelude::DrawBatch;
use specs::prelude::*;
use units::{Box2DI32, Position2DI32};

use crate::components::{Item, Player, Renderable, WorldPosition2D};
use crate::map::{Map, TileGraphic};
use crate::ui::units::ScreenChars;
use crate::world::units::WorldUnits;

use super::colors;
use super::common::Consoles;

const VISIBLE: ColorPair = ColorPair {
    fg: colors::OPAQUE_WHITE,
    bg: colors::OPAQUE_WHITE,
};
const SEEN: ColorPair = ColorPair {
    fg: colors::OPAQUE_GRAY,
    bg: colors::OPAQUE_GRAY,
};

pub fn get_camera_bounds_in_world(
    ecs: &World,
    camera_view: Box2DI32<ScreenChars>,
) -> Option<Box2DI32<WorldUnits>> {
    let positions = ecs.read_storage::<WorldPosition2D>();
    let players = ecs.read_storage::<Player>();

    for (player_position_component, _player) in (&positions, &players).join() {
        let player_position = player_position_component.to_world_units();
        let camera_size_in_world = WorldUnits::new_size2d(
            camera_view.width().to_primitive(),
            camera_view.height().to_primitive(),
        );

        return Some(WorldUnits::new_box2d_from_position_and_size(
            player_position - (camera_size_in_world / 2),
            camera_size_in_world,
        ));
    }
    None
}

pub fn render_camera(
    ecs: &World,
    camera_view: Box2DI32<ScreenChars>,
    camera_in_world: Box2DI32<WorldUnits>,
    window_bounds: Box2DI32<ScreenChars>,
) {
    render_terrain_in_camera(ecs, camera_view, camera_in_world, window_bounds);
    render_entities_in_camera(ecs, camera_view, camera_in_world);
}

pub fn render_terrain_in_camera(
    ecs: &World,
    camera_view: Box2DI32<ScreenChars>,
    camera_in_world: Box2DI32<WorldUnits>,
    window_bounds: Box2DI32<ScreenChars>,
) {
    let map = ecs.fetch::<Map>();
    let map_bounds = map.bounds();

    let mut batch = DrawBatch::new();
    batch.target(Consoles::WorldTerrain as usize);
    batch.cls();

    window_bounds.for_each(|screen_pos: Position2DI32<ScreenChars>| {
        let screen_pos_as_pt = screen_pos.to_bracket_geometry_point();
        if camera_view.contains(screen_pos) {
            let screen_pos_as_world_offset =
                WorldUnits::new_size2d(screen_pos.x.to_primitive(), screen_pos.y.to_primitive());
            let pos_in_world = camera_in_world.p1 + screen_pos_as_world_offset;
            if map_bounds.contains(pos_in_world) {
                let tile_idx = map.to_index(pos_in_world);
                if map.visible_tiles[tile_idx] {
                    batch.set(screen_pos_as_pt, VISIBLE, map.tiles[tile_idx] as u16);
                } else if map.revealed_tiles[tile_idx] {
                    batch.set(screen_pos_as_pt, SEEN, map.tiles[tile_idx] as u16);
                } else {
                    batch.set(screen_pos_as_pt, VISIBLE, TileGraphic::Void as u16);
                }
                return;
            }
        }
        batch.set(screen_pos_as_pt, SEEN, TileGraphic::Void as u16);
    });

    batch
        .submit(Consoles::WorldTerrain as usize)
        .expect("Couldn't render tiles");
}

pub fn render_entities_in_camera(
    ecs: &World,
    camera_view: Box2DI32<ScreenChars>,
    camera_in_world: Box2DI32<WorldUnits>,
) {
    let map = ecs.fetch::<Map>();
    let entities = ecs.entities();
    let positions = ecs.read_storage::<WorldPosition2D>();
    let renderables = ecs.read_storage::<Renderable>();
    let items = ecs.read_storage::<Item>();

    let mut draw_items = DrawBatch::new();
    draw_items.target(Consoles::WorldItems as usize);
    draw_items.cls();

    let mut draw_characters = DrawBatch::new();
    draw_characters.target(Consoles::WorldActors as usize);
    draw_characters.cls();

    for (entity, pos_comp, render) in (&entities, &positions, &renderables).join() {
        let pos = pos_comp.to_world_units();
        let tile_idx = map.to_index(pos);
        if map.visible_tiles[tile_idx] {
            let screen_pos_as_offset = pos - camera_in_world.p1;
            let screen_pos = ScreenChars::new_position2d(
                screen_pos_as_offset.width.to_primitive(),
                screen_pos_as_offset.height.to_primitive(),
            );
            if camera_view.contains(screen_pos) {
                match items.get(entity) {
                    None => {
                        draw_characters.set(
                            screen_pos.to_bracket_geometry_point(),
                            VISIBLE,
                            render.graphic as u16,
                        );
                    }
                    Some(_item) => {
                        draw_items.set(
                            screen_pos.to_bracket_geometry_point(),
                            VISIBLE,
                            render.graphic as u16,
                        );
                    }
                }
            }
        }
    }

    draw_items
        .submit(Consoles::WorldItems as usize)
        .expect("Couldn't render items");
    draw_characters
        .submit(Consoles::WorldActors as usize)
        .expect("Couldn't render entities");
}
