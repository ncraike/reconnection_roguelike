use bracket_color::prelude::{ColorPair, RGB};
use bracket_geometry::prelude::{Point, Rect};
use bracket_terminal::prelude::{render_draw_buffer, BTerm, DrawBatch};
use specs::prelude::*;

use super::components::{Player, Renderable};
use super::map::{Map, TileGraphic};

pub fn get_view_bounds(ecs: &World, ctx: &mut BTerm) -> Option<Rect> {
    let positions = ecs.read_storage::<Point>();
    let players = ecs.read_storage::<Player>();

    for (player_pos, _player) in (&positions, &players).join() {
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
    match maybe_view_bounds {
        Some(view_bounds) => {
            let visible_color: ColorPair =
                ColorPair::new(RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(1.0, 1.0, 1.0));
            let seen_color: ColorPair =
                ColorPair::new(RGB::from_f32(0.7, 0.7, 0.7), RGB::from_f32(0.7, 0.7, 0.7));

            let map = ecs.fetch::<Map>();
            let map_bounds = map.bounds();

            let mut draws = DrawBatch::new();
            draws.cls();
            draws.target(0);

            let draw_tile = |screen_xy: Point, map_xy: Point| {
                if map_bounds.point_in_rect(map_xy) {
                    let tile_idx = map.to_index(map_xy);
                    if map.visible_terrain[tile_idx] {
                        draws.set(screen_xy, visible_color, map.terrain[tile_idx] as u16);
                    } else if map.revealed_terrain[tile_idx] {
                        draws.set(screen_xy, seen_color, map.terrain[tile_idx] as u16);
                    } else {
                        draws.set(screen_xy, visible_color, TileGraphic::Void as u16);
                    }
                } else {
                    draws.set(screen_xy, seen_color, TileGraphic::Void as u16);
                }
            };
            rect_for_each_enumed(&view_bounds, draw_tile);

            let positions = ecs.read_storage::<Point>();
            let renderables = ecs.read_storage::<Renderable>();

            draws.target(1);
            for (pos, render) in (&positions, &renderables).join() {
                draws.set(
                    Point {
                        x: pos.x - view_bounds.x1,
                        y: pos.y - view_bounds.y1,
                    },
                    ColorPair::new(RGB::from_f32(1.0, 1.0, 1.0), RGB::from_f32(0., 0., 0.)),
                    render.graphic as u16,
                );
            }

            draws.submit(0).expect("Couldn't draw entities");
            render_draw_buffer(ctx).expect("Couldn't render camera");
        }
        None => return,
    }
}
