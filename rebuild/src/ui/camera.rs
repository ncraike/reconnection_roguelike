use bracket_color::prelude::ColorPair;
use bracket_geometry::prelude::{Point, Rect};
use bracket_terminal::prelude::DrawBatch;
use specs::prelude::*;
use units::{Box2DI32, Position2DI32, UnitI32};

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
        if camera_view.contains(screen_pos) {
            let pos_in_world = camera_in_world.p1
                + WorldUnits::new_size2d(screen_pos.x.to_primitive(), screen_pos.y.to_primitive());
            let map_pt = Point {
                x: camera_in_world.x1 + screen_pos.x,
                y: camera_in_world.y1 + screen_pos.y,
            };
            if map_bounds.point_in_rect(map_pt) {
                let tile_idx = map.to_index(map_pt);
                if map.visible_tiles[tile_idx] {
                    batch.set(screen_pos, VISIBLE, map.tiles[tile_idx] as u16);
                } else if map.revealed_tiles[tile_idx] {
                    batch.set(screen_pos, SEEN, map.tiles[tile_idx] as u16);
                } else {
                    batch.set(screen_pos, VISIBLE, TileGraphic::Void as u16);
                }
                return;
            }
        }
        batch.set(screen_pos, SEEN, TileGraphic::Void as u16);
    });

    batch
        .submit(Consoles::TilesTerrain as usize)
        .expect("Couldn't render tiles");
}

pub fn render_entities_in_camera(
    ecs: &World,
    camera_view: Box2DI32<ScreenChars>,
    camera_in_world: Box2DI32<WorldUnits>,
) {
    let map = ecs.fetch::<Map>();
    let entities = ecs.entities();
    let positions = ecs.read_storage::<Point>();
    let renderables = ecs.read_storage::<Renderable>();
    let items = ecs.read_storage::<Item>();

    let mut draw_items = DrawBatch::new();
    draw_items.target(Consoles::WorldItems as usize);
    draw_items.cls();

    let mut draw_characters = DrawBatch::new();
    draw_characters.target(Consoles::WorldActors as usize);
    draw_characters.cls();

    for (entity, pos, render) in (&entities, &positions, &renderables).join() {
        let tile_idx = map.to_index(*pos);
        if map.visible_tiles[tile_idx] {
            let camera_pt = Point {
                x: pos.x - camera_in_world.x1,
                y: pos.y - camera_in_world.y1,
            };
            if camera_view.point_in_rect(camera_pt) {
                match items.get(entity) {
                    None => {
                        draw_characters.set(camera_pt, VISIBLE, render.graphic as u16);
                    }
                    Some(_item) => {
                        draw_items.set(camera_pt, VISIBLE, render.graphic as u16);
                    }
                }
            }
        }
    }

    draw_items
        .submit(Consoles::TilesEntitiesItems as usize)
        .expect("Couldn't render items");
    draw_characters
        .submit(Consoles::TilesEntitiesCharacters as usize)
        .expect("Couldn't render entities");
}
