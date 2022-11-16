use bracket_color::prelude::{ColorPair, RGB};
use bracket_geometry::prelude::{Point, Rect};
use bracket_terminal::prelude::{render_draw_buffer, BTerm, DrawBatch};
use specs::prelude::*;

use super::components::{Player, Renderable};
use super::map::{Map, TileGraphic};

pub const DEFAULT_VIEW_WIDTH: u32 = 55;
pub const DEFAULT_VIEW_HEIGHT: u32 = 20;

pub fn get_view_bounds(ecs: &World, ctx: &mut BTerm) -> Option<Rect> {
    let positions = ecs.read_storage::<Point>();
    let players = ecs.read_storage::<Player>();

    for (player_pos, _player) in (&positions, &players).join() {
        ctx.set_active_console(0);
        let (x_chars, y_chars) = ctx.get_char_size();

        let center_x = (x_chars / 2) as i32;
        let center_y = (y_chars / 2) as i32;

        return Some(Rect::with_size(
            player_pos.x - center_x,
            player_pos.y - center_y,
            x_chars as i32,
            y_chars as i32,
        ));
    }
    None
}

pub fn rect_for_each_enumed<F>(&rect: &Rect, mut f: F)
where
    F: FnMut(Point, Point),
{
    let mut enum_y = 0;

    for y in rect.y1..=rect.y2 {
        let mut enum_x = 0;
        for x in rect.x1..=rect.x2 {
            f(Point::new(enum_x, enum_y), Point::new(x, y));
            enum_x += 1;
        }
        enum_y += 1;
    }
}

pub fn render_camera(ecs: &World, ctx: &mut BTerm) {
    let maybe_view_bounds = get_view_bounds(ecs, ctx);
    if maybe_view_bounds.is_none() {
        return;
    }
    let view_bounds = maybe_view_bounds.unwrap();
    let visible_color: ColorPair =
        ColorPair::new(RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(1.0, 1.0, 1.0));
    let seen_color: ColorPair =
        ColorPair::new(RGB::from_f32(0.7, 0.7, 0.7), RGB::from_f32(0.7, 0.7, 0.7));

    let map = ecs.fetch::<Map>();
    let map_bounds = map.bounds();

    let mut draws = DrawBatch::new();
    draws.target(0);
    draws.cls();

    let draw_tile = |screen_xy: Point, map_xy: Point| {
        if map_bounds.point_in_rect(map_xy) {
            let tile_idx = map.to_index(map_xy);
            if map.visible_tiles[tile_idx] {
                draws.set(screen_xy, visible_color, map.tiles[tile_idx] as u16);
            } else if map.revealed_tiles[tile_idx] {
                draws.set(screen_xy, seen_color, map.tiles[tile_idx] as u16);
            } else {
                draws.set(screen_xy, visible_color, TileGraphic::Void as u16);
            }
        } else {
            draws.set(screen_xy, seen_color, TileGraphic::Void as u16);
        }
    };
    rect_for_each_enumed(&view_bounds, draw_tile);
    draws.submit(0).expect("Couldn't draw tiles");

    let positions = ecs.read_storage::<Point>();
    let renderables = ecs.read_storage::<Renderable>();

    draws.target(1);
    draws.cls();
    for (pos, render) in (&positions, &renderables).join() {
        let tile_idx = map.to_index(*pos);
        if map.visible_tiles[tile_idx] {
            draws.set(
                Point {
                    x: pos.x - view_bounds.x1,
                    y: pos.y - view_bounds.y1,
                },
                ColorPair::new(RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0., 0., 0.)),
                render.graphic as u16,
            );
        }
    }
    draws.submit(1).expect("Couldn't draw entities");

    draws.target(2);
    draws.cls();
    draws.print_color(
        Point {
            x: 0,
            y: (DEFAULT_VIEW_HEIGHT - 3) as i32,
        },
        "Hello!",
        ColorPair::new(RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0., 0., 0.)),
    );
    draws.submit(2).expect("Couldn't draw text");

    render_draw_buffer(ctx).expect("Couldn't render camera");
}
