use bracket_color::prelude::ColorPair;
use bracket_geometry::prelude::{Point, Rect};
use bracket_terminal::prelude::DrawBatch;
use specs::prelude::*;

use super::super::components::{Item, Player, Renderable};
use super::super::map::{Map, TileGraphic};

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

pub fn get_camera_bounds_in_world(ecs: &World, camera_view: Rect) -> Option<Rect> {
    let positions = ecs.read_storage::<Point>();
    let players = ecs.read_storage::<Player>();

    for (player_pos, _player) in (&positions, &players).join() {
        let center_x = (camera_view.width() / 2) as i32;
        let center_y = (camera_view.height() / 2) as i32;

        return Some(Rect::with_size(
            player_pos.x - center_x,
            player_pos.y - center_y,
            camera_view.width() as i32,
            camera_view.height() as i32,
        ));
    }
    None
}

pub fn render_camera(ecs: &World, camera_view: Rect, camera_in_world: Rect, window_bounds: Rect) {
    render_terrain_in_camera(ecs, camera_view, camera_in_world, window_bounds);
    render_entities_in_camera(ecs, camera_view, camera_in_world);
}

pub fn render_terrain_in_camera(
    ecs: &World,
    camera_view: Rect,
    camera_in_world: Rect,
    window_bounds: Rect,
) {
    let map = ecs.fetch::<Map>();
    let map_bounds = map.bounds();

    let mut batch = DrawBatch::new();
    batch.target(Consoles::TilesTerrain as usize);
    batch.cls();

    window_bounds.for_each(|screen_pt: Point| {
        if camera_view.point_in_rect(screen_pt) {
            let map_pt = Point {
                x: camera_in_world.x1 + screen_pt.x,
                y: camera_in_world.y1 + screen_pt.y,
            };
            if map_bounds.point_in_rect(map_pt) {
                let tile_idx = map.to_index(map_pt);
                if map.visible_tiles[tile_idx] {
                    batch.set(screen_pt, VISIBLE, map.tiles[tile_idx] as u16);
                } else if map.revealed_tiles[tile_idx] {
                    batch.set(screen_pt, SEEN, map.tiles[tile_idx] as u16);
                } else {
                    batch.set(screen_pt, VISIBLE, TileGraphic::Void as u16);
                }
                return;
            }
        }
        batch.set(screen_pt, SEEN, TileGraphic::Void as u16);
    });

    batch
        .submit(Consoles::TilesTerrain as usize)
        .expect("Couldn't render tiles");
}

pub fn render_entities_in_camera(ecs: &World, camera_view: Rect, camera_in_world: Rect) {
    let map = ecs.fetch::<Map>();
    let entities = ecs.entities();
    let positions = ecs.read_storage::<Point>();
    let renderables = ecs.read_storage::<Renderable>();
    let items = ecs.read_storage::<Item>();

    let mut draw_items = DrawBatch::new();
    draw_items.target(Consoles::TilesEntitiesItems as usize);
    draw_items.cls();

    let mut draw_characters = DrawBatch::new();
    draw_characters.target(Consoles::TilesEntitiesCharacters as usize);
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
