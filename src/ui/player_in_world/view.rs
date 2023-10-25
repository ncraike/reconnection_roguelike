use bracket_geometry::prelude::{Point, Rect};
use bracket_terminal::prelude::{render_draw_buffer, BTerm, DrawBatch};
use specs::prelude::*;

use crate::ui::common::Consoles;
use crate::ui::units::{Box2D, Height, TextChars};
use crate::ui::utils::{get_mouse_point_in_text_chars, get_mouse_point_in_tiles2x, window_size};

use super::super::camera::{get_camera_bounds_in_world, render_camera};
use super::super::messages::render_messages;
use super::super::stats::render_stats;
use super::super::tooltips::render_tooltips;

pub const MESSAGE_BOX_HEIGHT: Height<TextChars> = Height(TextChars(6));

#[derive(Debug)]
pub struct PlayerInWorldView {
    pub camera_view_2x: Rect,
    pub message_log_view: Rect,
    pub stats_view: Rect,
    pub window_in_tiles: Rect,
    pub window_in_text: Rect,
    pub mouse_pt_in_tiles: Point,
    pub mouse_pt_in_text: Point,
}

impl PlayerInWorldView {
    pub fn from_context(ctx: &mut BTerm) -> PlayerInWorldView {
        let window = Box2D::new_from_size(window_size(ctx));
        let (camera_view, bottom_hud) =
            window.split_from_bottom(MESSAGE_BOX_HEIGHT.to_tiles2x_ceil());

        let mouse_pt_in_tiles2x = get_mouse_point_in_tiles2x(ctx);
        let mouse_pt_in_text_chars = get_mouse_point_in_text_chars(ctx);

        ctx.set_active_console(Consoles::Text as usize);
        let (width_in_text, height_in_text) = ctx.get_char_size();
        let message_log_width = width_in_text / 2;
        let stats_width = width_in_text - message_log_width;
        let mouse_pt_in_text = ctx.mouse_point();

        PlayerInWorldView {
            camera_view_2x: Rect::with_size(
                0,
                0,
                width_in_tiles,
                if height_in_tiles > TEXT_BOX_HEIGHT_IN_TILES {
                    height_in_tiles - TEXT_BOX_HEIGHT_IN_TILES
                } else {
                    height_in_tiles
                },
            ),
            message_log_view: Rect::with_size(
                0,
                height_in_text - TEXT_BOX_HEIGHT,
                message_log_width,
                TEXT_BOX_HEIGHT,
            ),
            stats_view: Rect::with_size(
                message_log_width,
                height_in_text - TEXT_BOX_HEIGHT,
                stats_width,
                TEXT_BOX_HEIGHT,
            ),
            window_in_tiles: Rect::with_size(0, 0, width_in_tiles, height_in_tiles),
            window_in_text: Rect::with_size(0, 0, width_in_text, height_in_text),
            mouse_pt_in_tiles: mouse_pt_in_tiles,
            mouse_pt_in_text: mouse_pt_in_text,
        }
    }
}

pub fn render_player_in_world_view(ecs: &World, ctx: &mut BTerm) {
    let main_view = PlayerInWorldView::from_context(ctx);
    let maybe_camera_in_world = get_camera_bounds_in_world(ecs, main_view.camera_view_2x);
    if maybe_camera_in_world.is_none() {
        // No player yet?
        return;
    }
    let camera_in_world = maybe_camera_in_world.unwrap();

    render_camera(
        ecs,
        main_view.camera_view_2x,
        camera_in_world,
        main_view.window_in_tiles,
    );
    let mut batch = DrawBatch::new();
    batch.target(Consoles::Text as usize);
    batch.cls();
    render_messages(ecs, &mut batch, main_view.message_log_view);
    render_stats(ecs, &mut batch, main_view.stats_view);
    render_tooltips(
        &mut batch,
        ecs,
        main_view.camera_view_2x,
        camera_in_world,
        main_view.mouse_pt_in_tiles,
        main_view.mouse_pt_in_text,
    );

    render_draw_buffer(ctx).expect("Couldn't render camera");
}
