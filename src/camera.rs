use bracket_color::prelude::{ColorPair, RGBA};
use bracket_geometry::prelude::{Point, Rect};
use bracket_terminal::prelude::DrawBatch;
use specs::prelude::*;

use super::components::{Player, Renderable};
use super::gui::Consoles;
use super::map::{Map, TileGraphic};

pub const WHITE: RGBA = RGBA {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};
pub const LIGHT_GRAY: RGBA = RGBA {
    r: 0.7,
    g: 0.7,
    b: 0.7,
    a: 1.0,
};
pub const VISIBLE_COLOR: ColorPair = ColorPair {
    fg: WHITE,
    bg: WHITE,
};
pub const SEEN_COLOR: ColorPair = ColorPair {
    fg: LIGHT_GRAY,
    bg: LIGHT_GRAY,
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

pub fn render_camera(ecs: &World, batch: &mut DrawBatch, camera_view: Rect, window_bounds: Rect) {
    let maybe_camera_in_world = get_camera_bounds_in_world(ecs, camera_view);
    if maybe_camera_in_world.is_none() {
        return;
    }
    let camera_in_world = maybe_camera_in_world.unwrap();
    render_terrain_in_camera(batch, ecs, camera_view, camera_in_world, window_bounds);
    render_entities_in_camera(batch, ecs, camera_view, camera_in_world);
}

pub fn render_terrain_in_camera(
    batch: &mut DrawBatch,
    ecs: &World,
    camera_view: Rect,
    camera_in_world: Rect,
    window_bounds: Rect,
) {
    let map = ecs.fetch::<Map>();
    let map_bounds = map.bounds();

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
                    batch.set(screen_pt, VISIBLE_COLOR, map.tiles[tile_idx] as u16);
                } else if map.revealed_tiles[tile_idx] {
                    batch.set(screen_pt, SEEN_COLOR, map.tiles[tile_idx] as u16);
                } else {
                    batch.set(screen_pt, VISIBLE_COLOR, TileGraphic::Void as u16);
                }
                return;
            }
        }
        batch.set(screen_pt, SEEN_COLOR, TileGraphic::Void as u16);
    });

    batch
        .submit(Consoles::TilesTerrain as usize)
        .expect("Couldn't render tiles");
}

pub fn render_entities_in_camera(
    batch: &mut DrawBatch,
    ecs: &World,
    camera_view: Rect,
    camera_in_world: Rect,
) {
    let map = ecs.fetch::<Map>();
    let positions = ecs.read_storage::<Point>();
    let renderables = ecs.read_storage::<Renderable>();

    batch.target(Consoles::TilesEntities as usize);
    batch.cls();
    for (pos, render) in (&positions, &renderables).join() {
        let tile_idx = map.to_index(*pos);
        if map.visible_tiles[tile_idx] {
            let camera_pt = Point {
                x: pos.x - camera_in_world.x1,
                y: pos.y - camera_in_world.y1,
            };
            if camera_view.point_in_rect(camera_pt) {
                batch.set(camera_pt, VISIBLE_COLOR, render.graphic as u16);
            }
        }
    }
    batch
        .submit(Consoles::TilesEntities as usize)
        .expect("Couldn't render entities");
}
