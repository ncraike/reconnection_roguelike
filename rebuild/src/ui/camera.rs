use bracket_color::named;
use bracket_color::prelude::{ColorPair, RGBA};
use bracket_terminal::prelude::{console, to_cp437, DrawBatch};
use specs::prelude::*;
use units::{Box2DI32, Position2DI32};

use crate::components::{Item, Player, Renderable, WorldPosition2D};
use crate::map::{Map, TileGraphic};
use crate::ui::common::Consoles;
use crate::ui::units::ScreenChars;
use crate::world::units::WorldUnits;

const VISIBLE_FG: (u8, u8, u8) = named::GRAY;
const VISIBLE_BG: (u8, u8, u8) = named::BLACK;
const SEEN_FG: (u8, u8, u8) = named::DIM_GRAY;
const SEEN_BG: (u8, u8, u8) = named::BLACK;

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

    let visible_color = ColorPair {
        fg: RGBA::named(VISIBLE_FG),
        bg: RGBA::named(VISIBLE_BG),
    };
    let seen_color = ColorPair {
        fg: RGBA::named(SEEN_FG),
        bg: RGBA::named(SEEN_BG),
    };

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
                    batch.set(
                        screen_pos_as_pt,
                        visible_color,
                        tile_to_glyph(map.tiles[tile_idx]),
                    );
                } else if map.revealed_tiles[tile_idx] {
                    batch.set(
                        screen_pos_as_pt,
                        seen_color,
                        tile_to_glyph(map.tiles[tile_idx]),
                    );
                } else {
                    batch.set(
                        screen_pos_as_pt,
                        visible_color,
                        tile_to_glyph(TileGraphic::Void),
                    );
                }
                return;
            }
        }
        batch.set(
            screen_pos_as_pt,
            seen_color,
            tile_to_glyph(TileGraphic::Void),
        );
    });

    batch
        .submit(Consoles::WorldTerrain as usize)
        .expect("Couldn't render tiles");
}

pub fn tile_to_glyph(tile: TileGraphic) -> u16 {
    match tile {
        TileGraphic::Void => to_cp437(' '),
        TileGraphic::Ground1
        | TileGraphic::Ground2
        | TileGraphic::Ground3
        | TileGraphic::Ground4 => to_cp437('.'),
        TileGraphic::Floor1 | TileGraphic::Floor2 => to_cp437(','),
        TileGraphic::WallHExternal | TileGraphic::WallHInternal => to_cp437('-'),
        TileGraphic::WallSECornerExternal
        | TileGraphic::WallSWCornerExternal
        | TileGraphic::WallNWCorner
        | TileGraphic::WallNECorner => to_cp437('+'),
        TileGraphic::WallV => to_cp437('|'),
        TileGraphic::PlayerCharacter => to_cp437('@'),
        TileGraphic::EnemyHound => to_cp437('h'),
        TileGraphic::EnemySmallStalker => to_cp437('s'),
        TileGraphic::EnemyBigStalker => to_cp437('S'),
        TileGraphic::ItemBandage => to_cp437('&'),
        TileGraphic::ItemFirstAidKit => to_cp437('='),
        _ => to_cp437('?'),
    }
}

pub fn render_entities_in_camera(
    ecs: &World,
    camera_view: Box2DI32<ScreenChars>,
    camera_in_world: Box2DI32<WorldUnits>,
) {
    let visible_color = ColorPair {
        fg: RGBA::named(VISIBLE_FG),
        bg: RGBA::named(VISIBLE_BG),
    };
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
                            visible_color,
                            tile_to_glyph(render.graphic),
                        );
                    }
                    Some(_item) => {
                        draw_items.set(
                            screen_pos.to_bracket_geometry_point(),
                            visible_color,
                            tile_to_glyph(render.graphic),
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
